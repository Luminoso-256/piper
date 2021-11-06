/*
piper-srv
a simple demo server utilizing the piper (piper://) protocol
the executable will serve files out of a "srv" directory placed adjacent to the executable.
*/
#![feature(num_as_ne_bytes)]
use std::{thread,fs,env};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write,self, BufRead};


//spawned on each request, works completely on it's own
//afterall, Piper is Stateless.
fn handle_request(mut stream: TcpStream){
    //pull in the bytes
    let mut reader = io::BufReader::new(&mut stream);
    let mut received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());
    //trim the content len, since we're just working without it/don't care.
    received.remove(0);
    received.remove(0);
    let mut target_uri = std::str::from_utf8(&*received).unwrap();
    println!("[Info] Request is for URI {}",target_uri);
    //fecth the file locally
    let target_path = format!("srv/{}",target_uri);
    let ext = target_uri.split(".").last().unwrap();
    let file = fs::read_to_string(target_path).unwrap();
    let mut response: Vec<u8> = vec![];
    //write in the header!

    //pt.1: content type
    match ext{
        "txt" => {response.push(0x00)}
        &_ => {response.push(0x10)}
    }
    response.extend((file.len() as u64).as_ne_bytes());
    response.extend(file.into_bytes());
    stream.write_all(&*response);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:60").unwrap();
    println!("[Info] Listening on Port 60");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[Info] Request incoming from: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_request(stream)
                });
            }
            Err(e) => {
                println!("Error connecting to tcp stream: {}", e);
            }
        }
    }
}
