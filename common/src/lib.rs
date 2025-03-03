use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    Connect { username: String },
    Disconnect { username: String },
    ChatMessage { username: String, content: String }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub msg_type: MessageType,
    pub timestamp: u64, // Unix timestamp
}

impl Message {
    pub fn new(msg_type: MessageType) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Message { msg_type, timestamp }
    }

    pub fn new_chat(username: String, content: String) -> Self {
        Self::new(MessageType::ChatMessage { username, content })
    }
    
    pub fn new_connect(username: String) -> Self {
        Self::new(MessageType::Connect { username })
    }

    pub fn new_disconnect(username: String) -> Self {
        Self::new(MessageType::Disconnect { username })
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.msg_type {
            MessageType::Connect { username } => {
                write!(f, "[CONNECT] {} has joined the chat", username)
            }
            MessageType::Disconnect { username } => {
                write!(f, "[DISCONNECT] {} has left the chat", username)
            }
            MessageType::ChatMessage { username, content } => {
                write!(f, "[{}] {}", username, content)
            }
        }
    }
}

pub fn serialize(message: Message) -> Result<String, Box<dyn std::error::Error>>{
    let serialized = serde_json::to_string(&message)?;
    Ok(serialized)
}

pub fn deserialize(message: String) -> Result<Message, Box<dyn std::error::Error>> {
    let deserialized = serde_json::from_str(&message)?;
    Ok(deserialized)
}
