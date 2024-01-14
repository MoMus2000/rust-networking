use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

pub fn spawn_tcp_server(){
    let listener = TcpListener::bind("0.0.0.0:8080") // TCP Socket Listening
        .expect("Could not bind");
    for stream in listener.incoming() {
        match stream{
            Err(e) => { eprintln!("Failed {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    let _ = server(stream);
                });
            }
        }
    }
}

fn server(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        let result_string = std::str::from_utf8(&buf).expect("Error parsing");
        let result_string = &result_string[..bytes_read];
        println!("{}", result_string);
        if bytes_read == 0 { return Ok(()); }
        stream.write(&buf[..bytes_read])?;
    }
}
