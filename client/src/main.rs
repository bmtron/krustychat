use std::io;
use std::{io::Read, io::Write, net::TcpStream};
use common::{serialize, Message, MessageType};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    chatcode: u32
}
    

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Brendan is the man.");


    let tcp_stream = TcpStream::connect("127.0.0.1:5001")?;
    
    run_chat(tcp_stream, args.username, args.chatcode);
    Ok(())
}

fn run_chat(mut stream: TcpStream, username: String, chatcode: u32) {
    let mut read_stream = stream.try_clone().expect("Failed to clone stream");

    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server closed connection");
                    break;
                }
                Ok(n) => {
                    // process server message
                    println!("Received: {}", String::from_utf8_lossy(&buffer[0..n]));
                }
                Err(e) => {
                    println!("Error reading from the server: {}", e);
                    break;
                }
            }
        }
    });
    

    let connect_msg = Message::new_connect(username.clone(), chatcode);

    if let Ok(serialized) = serialize(&connect_msg) {
        let msg_size = serialized.len() as u32;
        let length_bytes = msg_size.to_be_bytes();
        let _ = stream.write_all(&length_bytes);
        let _ = stream.write_all(serialized.as_bytes());
        stream.flush().unwrap();
    }
    loop {
        let mut input = String::new();
        let username_mutable = username.clone();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "quit" {
                    break;
                }

                let msg = Message::new_chat(username_mutable, chatcode, String::from(input));
                if let Ok(serialized) = serialize(&msg) {

                    let msg_size = serialized.len() as u32;
                    let length_bytes = msg_size.to_be_bytes();
                    let _ = stream.write_all(&length_bytes);
                    if let Err(e) = stream.write_all(serialized.as_bytes()) {
                        println!("Error sending message: {}", e);
                        break;
                    }
                    stream.flush().unwrap();
                }
            }
            Err(_) => {
                println!("Error reading user input");
                break;
            }
        };
    }
}
