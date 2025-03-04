use common::{deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5001").unwrap();
    let stream_map: Arc<Mutex<HashMap<u32, Vec<(String, TcpStream)>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let map_clone = Arc::clone(&stream_map);
        std::thread::spawn(move || {
            let conn = handle_connection(stream, false, map_clone);
            match conn {
                Ok(_) => println!("Connection aborted successfully."),
                Err(err) => println!("Error deserializing message: {}", err),
            };
        });
    }
}

fn handle_connection(
    mut stream: TcpStream,
    abort: bool,
    stream_map: Arc<Mutex<HashMap<u32, Vec<(String, TcpStream)>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if abort {
            break;
        }
        let mut length_buffer = [0u8; 4];
        let _ = stream.read_exact(&mut length_buffer);
        let length = u32::from_be_bytes(length_buffer) as usize;

        let mut message_buffer = vec![0u8; length];
        stream.read_exact(&mut message_buffer)?;

        let msg_string = match String::from_utf8(message_buffer) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Error converting message bytes to string: {}", e);
                String::from("")
            }
        };
        let message = match deserialize(&msg_string) {
            Ok(msg) => msg,
            Err(err) => return Err(err),
        };
        match &message.msg_type {
            common::MessageType::Connect {
                username,
                chat_code,
            } => {

                let mut map = stream_map.lock().unwrap();
                if !map.contains_key(chat_code) {
                    map.insert(*chat_code, Vec::new());
                }
                let stream_clone = stream.try_clone()?;

                if let Some(connections) = map.get_mut(chat_code) {
                    connections.push((username.to_string(), stream_clone));
                }
            }
            common::MessageType::ChatMessage {
                username,
                chat_code,
                content,
            } => {
                let mut map = stream_map.lock().unwrap();
                if let Some(connections) = map.get_mut(chat_code) {
                    for conn in connections {
                        if conn.0 != *username {
                            conn.1.write_all(&content.as_bytes())?;
                            conn.1.flush()?;
                        }
                    }
                }
            }
            _ => {}
        }
        println!("{}", message);
    }
    Ok(())
}
