use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use common::{Message, deserialize};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let msg = handle_connection(stream);
        match msg {
            Ok(msg) => println!("{:?}", msg),
            Err(err) => println!("Error deserializing message: {}", err)
        };
    }
}

fn handle_connection(stream: TcpStream) -> Result<Message, Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&stream);

    let incoming_message: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Incoming message: {:?}", incoming_message);
    let message = match deserialize(incoming_message.join(" ")) {
        Ok(msg) => msg,
        Err(err) => return Err(err),
    };

    Ok(message)
}
