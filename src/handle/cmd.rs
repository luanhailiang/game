use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use prost::Message;

use serde::Serialize;
use sha2::Sha256;
use std::{collections::BTreeMap, sync::Arc};

use crate::player::net::{Player,NetState};
use crate::world::ecs::EcsState;
use crate::proto::pb::*;

include!("handle.rs");

// fn tell_adr(adr:&String, cmd:&(impl Serialize+Message+MyName) ){
//     NetState::get_instance().lock().unwrap().tell_adr(adr,cmd);
// }
fn tell_uid(uid:&String, cmd:&(impl Serialize+Message+MyName) ){
    NetState::get_instance().lock().unwrap().tell_uid(uid,cmd);
}
// fn tell_eid(eid:&String, cmd:&(impl Serialize+Message+MyName) ){
//     NetState::get_instance().lock().unwrap().tell_eid(eid,cmd);
// }
fn tell_link(cmd:&(impl Serialize+Message+MyName)){
    NetState::get_instance().lock().unwrap().tell_link(cmd);
}
fn tell_auth(cmd:&(impl Serialize+Message+MyName)){
    NetState::get_instance().lock().unwrap().tell_auth(cmd);
}
fn tell_room(cmd:&(impl Serialize+Message+MyName)){
    NetState::get_instance().lock().unwrap().tell_room(cmd);
}
fn exec(cmd:&mut (impl Handle+Serialize+Message+MyName),me:Arc<Player>,bus:&mut Command){
    let (code,info) = cmd.handle(me);
    bus.code = code;
    bus.info = info;
    bus.dump_cmd(cmd);
}

// cmd match 自动生成宏
// match cmd.name.as_str() {
//     "CmdLogin" => exec(&mut serde_json::from_slice::<CmdLogin>(&cmd.data).unwrap(),me.clone(),cmd),
//     "CmdHeart" => exec(&mut serde_json::from_slice::<CmdLogin>(&cmd.data).unwrap(),me.clone(),cmd),
//     _ => {
//         println!("error message:{}",cmd.name);
//         cmd.code = 1;
//         cmd.info = "command not found".to_string();
//     },
// }
// #[macro_export]
macro_rules! cmd_match {
    ($_player:expr, $_command:expr, $($result:ident),*) => (
        match $_command.name.as_str() {
            $(
                stringify!($result) => exec(&mut serde_json::from_slice::<$result>(&$_command.data).unwrap(),$_player.clone(),$_command),
            )*
            _ => {
                println!("error message:<{}>",$_command.name);
                $_command.code = 1;
                $_command.info = "command not found".to_string();
            },
        }
    );
}

pub fn handle(me:Arc<Player>, cmd:&mut Command){
    cmd_match! { me, cmd,
        CmdLogin,
        CmdHeart
    };
}


