use std::collections::HashMap;
use std::io::{self, Error, Read};
use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
};

#[derive(Debug)]
struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub header: HashMap<String, String>,
    pub body: String,
}

impl Request {
    fn from(mut stream: &TcpStream) -> Result<Self, Error> {
        let mut method = String::new();
        let mut path = String::new();
        let mut version = String::new();
        let mut header: HashMap<String, String> = HashMap::new();
        let mut buffer = [0; 8];
        let mut request = Vec::new();
        let mut body = Vec::new();
        loop {
            let n = stream.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..n]);

            if let Some(header_left) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                let body_start = header_left + 4;
                let header_content = String::from_utf8_lossy(&request[..header_left]);
                let mut lines = header_content.lines();

                let request_line: Vec<_> = lines.next().expect("").split(" ").collect();
                method = request_line[0].to_string();
                path = request_line[1].to_string();
                version = request_line[2].to_string();
                for line in lines.skip(1) {
                    let header_parts: Vec<_> = line.split(":").collect();
                    let key = header_parts[0];
                    let value = header_parts[1];
                    header.insert(key.to_string(), value.trim().to_string());
                }

                let content_length: usize = header
                    .get("Content-Length")
                    .expect("0")
                    .parse::<usize>()
                    .unwrap();

                let body_recevied = request.len() - body_start;
                if body_recevied >= content_length {
                    body.extend_from_slice(&request[body_start..]);
                    break;
                }
            }
        }

        Ok(Self {
            header,
            body: String::from_utf8(body).expect("Found Invalid Body"),
            method,
            version,
            path,
        })
    }
}

fn handle_steam(mut stream: TcpStream) -> Result<(), io::Error> {
    let request = Request::from(&stream)?;

    println!(
        "Read From Request {} {} {} {:?}",
        request.method, request.path, request.version, request.body
    );

    let body = "Done";
    let head_text = "HTTP/1.1 200 OK";
    let head_text = format!("{head_text}\r\nContent-Type: text/html");
    let head_text = format!("{head_text}\r\nContent-Length: {}", body.len());
    let head_text = format!("{head_text}\r\nConnection: close\r\n\r\n");

    stream.write_all(head_text.as_str().as_bytes())?;
    stream.write_all(body.as_bytes())?;
    stream.flush()?;

    stream.shutdown(std::net::Shutdown::Both)?;

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
