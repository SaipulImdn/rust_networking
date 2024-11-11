use std::net::TcpStream;
use std::io::{self, Write, Read};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let mut input = String::new();
    println!("Enter message to send to the server:");

    io::stdin().read_line(&mut input)?;
    stream.write_all(input.as_bytes())?;
    
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Response from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
