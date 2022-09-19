mod handle;
mod player;
mod script;
mod world;
mod proto;
mod dbase;

use std::{
    env,
    io::Error as IoError,
    net::SocketAddr, sync::Arc,
};

use futures_channel::mpsc::{unbounded};
use futures_util::{future::{self}, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;


use world::ecs::EcsState;
use player::net::*;
use handle::cmd;
use proto::pb::Command;

fn handle_message(me:Arc<Player>,msg:Message) -> Result<Message, &'static str> {
    let mut cmd = Command::from_vec(msg.into_data());
    cmd::handle(me.clone(),&mut cmd);
    let bak =Message::Binary(cmd.to_vec());
	Ok(bak)
}

async fn handle_connection( raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    let player = Player{tx:tx,adr:addr.to_string(),eid:Default::default(),uid:Default::default()};
    let state = Arc::new(player);

    NetState::get_instance().lock().unwrap().insert(state.clone());

    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|msg| {
        // println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        // 这里hash性能略差，可写需要加锁麻烦且容易死锁
        let player = NetState::get_instance().lock().unwrap().get_adr(&addr.to_string());
		let ret = handle_message(player.clone(),msg);
		player.tx.unbounded_send(ret.unwrap()).unwrap();
        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr.to_string());
    NetState::get_instance().lock().unwrap().remove(state);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
	EcsState::get_instance().lock().unwrap().init();

    let addr = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:2794".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    if cfg!(feature = "protojson"){
        println!("network protocol using json");
    }else{
        println!("network protocol using buff");
    }
    println!("Listening on: {}", addr);
    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}

