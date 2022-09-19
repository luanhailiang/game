



impl Handle for CmdLogin{
    fn handle(&mut self,me:Arc<Player>) ->(u32, String){
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"lkk123").unwrap();
        let claims: BTreeMap<String, String> = self.token.verify_with_key(&key).unwrap();
        println!("info{:?} {:?}",claims.keys(),claims.values());

        let new_state = Arc::new(Player{
            tx:me.tx.clone(),
            adr:me.adr.clone(),
            uid:claims.get("index").unwrap().to_owned(),
            eid:Default::default()
        });

        NetState::get_instance().lock().unwrap().insert(new_state.clone());
        tell_uid(&new_state.uid,self);
        tell_uid(&new_state.uid,self);
        tell_uid(&new_state.uid,self);
        tell_uid(&new_state.uid,self);
        EcsState::get_instance().lock().unwrap().run();

        (0,Default::default())
    }
}

impl Handle for CmdHeart{
    fn handle(&mut self,me:Arc<Player>) ->(u32, String){
        tell_uid(&me.uid,self);
        tell_uid(&me.uid,self);
        tell_uid(&me.uid,self);
        tell_uid(&me.uid,self);
        let ret = crate::dbase::rds::fetch_an_integer().unwrap();
        println!("redis val:{}",ret);
        (0,Default::default())
    }
}
