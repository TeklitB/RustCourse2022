// Exercise
// Update the client ex8-client.rs -> ex9-client.rs to send ClientMessage::Hello
// and then read the ServerMessage response and print it

use std::net::TcpStream;
use std::io::Write;

use RustCourse2022::protocol::{ClientMessage, ServerMessage};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut socket = TcpStream::connect(("127.0.0.1", 12345))?;
    
    let msg = ClientMessage::Hello {
        name: "Emil".to_string(),
    };
    let buffer = bincode::serialize(&msg)?;
    socket.write(&buffer)?;

    let reply: ServerMessage = bincode::deserialize_from(&socket)?;

    println!("{:?}", &reply);

    Ok(())
}