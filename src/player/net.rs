use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{ UnboundedSender};
use prost::Message;
use serde::Serialize;
use tungstenite::protocol::Message as WSMessage;

use crate::proto::pb::{Command, MyName};

//其他web协议需要修改这里
type Tx = UnboundedSender<WSMessage>;


#[derive(Clone)]

pub struct Player{
    pub tx: Tx,
    pub adr: String,
    pub uid: String,
    pub eid: String,
}

pub struct NetState{
    adr: HashMap<String, Arc<Player>>,   //IP地址(socket  时注册)
    uid: HashMap<String, Arc<Player>>,   //用户ID(token验证时注册)
    eid: HashMap<String, Arc<Player>>,   //对象ID(创建游戏实体时)
}

impl NetState{
    pub fn get_instance() -> Arc<Mutex<NetState>> {
        static mut WEB_STATE: Option<Arc<Mutex<NetState>>> = None;
        unsafe {
            WEB_STATE.get_or_insert_with(|| {
                Arc::new(Mutex::new(NetState {adr:HashMap::new(),uid:HashMap::new(),eid:HashMap::new()}))
            }).clone()
        }
    }
    pub fn get_adr(&mut self,adr:&String)->Arc<Player> {
        self.adr.get(adr).unwrap().clone()
    }

    pub fn insert(&mut self,me:Arc<Player>){
        println!("NetState->insert adr:{},uid:{},eid:{}",me.adr,me.uid,me.eid);
        if me.eid != ""{
            self.eid.insert(me.eid.clone(), me.clone());
        }
        if me.uid != ""{
            self.uid.insert(me.uid.clone(), me.clone());
        }
        if me.adr != ""{
            self.adr.insert(me.adr.clone(), me.clone());
        }
    }
    pub fn remove(&mut self,me:Arc<Player>){
        if me.eid != ""{
            self.eid.remove(&me.eid);
        }
        if me.uid != ""{
            self.uid.remove(&me.uid);
        }
        if me.adr != ""{
            self.adr.remove(&me.adr);
        }
    }
    pub fn tell_adr(&mut self, adr:&String, cmd:&(impl Serialize+Message+MyName) ){
        let msg = WSMessage::Binary(Command::from_cmd(cmd).to_vec());
        self.adr.get(adr).unwrap().tx.unbounded_send(msg).unwrap();
    }

    pub fn tell_uid(&mut self, uid:&String, cmd:&(impl Serialize+Message+MyName) ){
        let msg = WSMessage::Binary(Command::from_cmd(cmd).to_vec());
        self.uid.get(uid).unwrap().tx.unbounded_send(msg).unwrap();
    }

    pub fn tell_eid(&mut self, eid:&String, cmd:&(impl Serialize+Message+MyName) ){
        let msg = WSMessage::Binary(Command::from_cmd(cmd).to_vec());
        self.eid.get(eid).unwrap().tx.unbounded_send(msg).unwrap();
    }

    pub fn tell_link(&mut self,  cmd:&(impl Serialize+Message+MyName) ){
        let msg = WSMessage::Binary(Command::from_cmd(cmd).to_vec());
        let broadcast_recipients = self.adr.iter().map(|(_, ws_sink)| ws_sink);
        for recp in broadcast_recipients {
            recp.tx.unbounded_send(msg.clone()).unwrap();
        }
    }
    pub fn tell_auth(&mut self,  cmd:&(impl Serialize+Message+MyName) ){
        let msg = WSMessage::Binary(Command::from_cmd(cmd).to_vec());
        let broadcast_recipients = self.uid.iter().map(|(_, ws_sink)| ws_sink);
        for recp in broadcast_recipients {
            recp.tx.unbounded_send(msg.clone()).unwrap();
        }
    }
    pub fn tell_room(&mut self,  cmd:&(impl Serialize+Message+MyName) ){
        let msg = WSMessage::Binary(Command::from_cmd(cmd).to_vec());
        let broadcast_recipients = self.eid.iter().map(|(_, ws_sink)| ws_sink);
        for recp in broadcast_recipients {
            recp.tx.unbounded_send(msg.clone()).unwrap();
        }
    }
}