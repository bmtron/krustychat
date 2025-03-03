use std::{io::Write, net::TcpStream};

use common::{serialize, Message, MessageType};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Brendan is the man.");


    let connect_msg = Message::new_connect(String::from("bmtron"));
    let serialized_msg = match serialize(connect_msg) {
        Ok(msg) => msg,
        Err(err) => { println!("Error serializing message: {}", err); String::from("ERROR") }
    };
    
    let mut tcp_server = TcpStream::connect("127.0.0.1:5001")?;
    let _ = tcp_server.write(&serialized_msg.as_bytes());
    Ok(())
}
