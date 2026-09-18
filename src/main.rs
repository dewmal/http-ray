use std::io::{self, Read};
use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_steam(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    println!("Read {n}");

    let body = "hello";
    let head_text = "HTTP/1.1 200 OK";
    let head_text = format!("{head_text}\r\nContent-Type: text/html");
    let head_text = format!("{head_text}\r\nContent-Length: {}", body.len());
    let head_text = format!("{head_text}\r\nConnection: close\r\n\r\n");

    stream.write_all(head_text.as_str().as_bytes())?;
    stream.write_all(body.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() {
    println!("Server started");
    let add_str = "127.0.0.1";
    let port = 23445;
    let socket_address: SocketAddr = format!("{add_str}:{port}").parse().unwrap();

    let port = socket_address.port();
    let ip = socket_address.ip();

    println!("IP:{}, Port:{}", ip, port);

    let listerner = TcpListener::bind(socket_address).unwrap();
    for stream_raw in listerner.incoming() {
        match stream_raw {
            Ok(stream) => match handle_steam(stream) {
                Ok(_) => println!("Done"),
                Err(er) => println!("Error :{:?}", er),
            },
            Err(er) => {
                println!("Error :{:?}", er);
            }
        }
    }
}
