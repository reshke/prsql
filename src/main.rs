use std::net::TcpStream;
use std::io::prelude::*;

mod lib;

fn request(stream: &mut TcpStream) -> ()  {
    println!("Connected to the server!");
    //let query = "SELECT 1";

    let sm = lib::StartupMsg::new();

    println!("proto ver num is {}", sm.ProtocolVersionNumber);

    let bytes = sm.tobytes();

    stream.write(&bytes);

    println!("write to the server!");
    // stream.read(&mut [0; 128]);

    return
}

fn main() {
    if let Ok(mut stream) =  TcpStream::connect("localhost:6432") {
        request(&mut stream);
    } else {
        println!("Couldn't connect to server...");
    }
}
