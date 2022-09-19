use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use mlua::prelude::*;
use mlua::Function;
use mlua::{Error, Lua, LuaSerdeExt, Value};

use mlua::{UserData};

#[derive(Default,Copy, Clone)]
pub struct CardData {
    pub cards :i32,
}


impl UserData for CardData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("cards", |_, this| Ok(this.cards));
        fields.add_field_method_set("cards", |_, this, val| {
            this.cards = val;
            Ok(())
        });
    }
}
pub struct LuaState{
    lua: Lua
}
impl LuaState{
    pub fn get_instance() -> Arc<Mutex<LuaState>> {
        static mut SYS_STATE: Option<Arc<Mutex<LuaState>>> = None;
        unsafe {
            SYS_STATE.get_or_insert_with(|| {
                let lua = Lua::new();
                let text = fs::read_to_string("./script/main.lua").unwrap();
                lua.load(&text).exec().unwrap();
                Arc::new(Mutex::new(LuaState {lua:lua}))
            }).clone()
        }
    }
    pub fn exec(&self,fun:&str,val:&CardData)-> LuaResult<String>{
        // let game = GameState{current_round:0,winning_player:None};
        let lua = &self.lua;
        let globals = lua.globals();
        let main: Function = globals.get(fun)?;

        let ret = main.call::<_,Value>(*val)?;
        println!("{:?}",ret);
        let data = serde_json::to_string(&ret).map_err(Error::external)?;
        Ok(data)
    }
    pub fn call(&self,fun:&str,msg:&str)-> LuaResult<String>{
        let lua = &self.lua;
        let globals = lua.globals();
        let main: Function = globals.get(fun)?;
        let json:serde_json::Value = serde_json::from_str(msg).unwrap();
        let cmd = lua.to_value(&json)?;
        let ret = main.call::<_,Value>(cmd)?;
        let data = serde_json::to_string(&ret).map_err(Error::external)?;
        Ok(data)
    }
}

    



