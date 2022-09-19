

use prost::Message;
use serde::Serialize;
use std::{
    sync::{Arc},
};

use crate::player::net::Player;

pub trait Handle {
    fn handle(self: &mut Self,value:Arc<Player>)->(u32,String);
}
pub trait MyName {
    fn name(self:&Self)->String;
}
tonic::include_proto!("command");

pub struct Command {
    pub code :u32,
    pub info :String,
    pub name :String,
    pub data :Vec<u8>,
}

impl Command{
    #[cfg(feature = "protojson")]
    pub fn from_cmd(cmd:&(impl Message+Serialize+MyName)) -> Command{
        Command{
            code: Default::default(),
            info: Default::default(),
            name: cmd.name(),
            data: serde_json::to_string(&cmd).unwrap().as_bytes().to_vec(),
        }
    }
    #[cfg(not(feature = "protojson"))]
    pub fn from_cmd(cmd:&(impl Message+Serialize+MyName)) -> Command{
        Command{
            code: Default::default(),
            info: Default::default(),
            name: cmd.name(),
            data: cmd.encode_to_vec(),
        }
    }
    #[cfg(feature = "protojson")]
    pub fn from_vec(bin:Vec<u8>) -> Command{
        let mut ret = bin.split(|num| *num == b'|');
        let path = ret.next().unwrap();
        let data = ret.next().unwrap();
   
        let temp = path.to_vec();
        let mut rest = temp.rsplit(|num| *num == b'.');
        
        let name = rest.next().unwrap();
        Command{
            code: Default::default(),
            info: Default::default(),
            name: String::from_utf8(name.to_vec()).unwrap(),
            data: data.to_vec(),
        }
    }
    #[cfg(not(feature = "protojson"))]
    pub fn from_vec(data:Vec<u8>) -> Command{
        let buff = BuffMessage::decode(&*data).unwrap();
        Command{
            code: Default::default(),
            info: Default::default(),
            name: buff.name,
            data: buff.data,
        }
    }
    #[cfg(feature = "protojson")]
    pub fn dump_cmd(&mut self, cmd:&(impl Message+Serialize+MyName)){
        self.name = cmd.name();
        self.data = serde_json::to_string(&cmd).unwrap().as_bytes().to_vec();
    }  
    #[cfg(not(feature = "protojson"))]
    pub fn dump_cmd(&mut self, cmd:&(impl Message+Serialize+MyName)){
        self.name = cmd.name();
        self.data = cmd.encode_to_vec();
    }  

    #[cfg(feature = "protojson")]
    pub fn to_vec(&self) -> Vec<u8>{
        format!("{}|{}|{}|{}",
            self.code,
            self.info,
            self.name,
            String::from_utf8(self.data.clone()).unwrap()
        ).as_bytes().to_vec()
    }
    #[cfg(not(feature = "protojson"))]
    pub fn to_vec(&self) -> Vec<u8>{
        BackMessage{
            code:self.code,
            info:self.info.clone(),
            buff: Some(BuffMessage{
              name: self.name.clone(),
              data: self.data.clone(),
            }),
        }.encode_to_vec()
    }

}
