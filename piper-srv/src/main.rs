/*
piper-srv
a simple demo server utilizing the piper (piper://) protocol
the executable will serve files out of a "srv" directory placed adjacent to the executable.
*/
use std::{thread,fs,env};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write,self, BufRead};


//spawned on each request, works completely on it's own
//afterall, Piper is Stateless.
fn handle_request(mut stream: TcpStream){
    //pull in the bytes
    let mut reader = io::BufReader::new(&mut stream);
    let mut received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    let mut response: Vec<u8> = vec![];
    reader.consume(received.len());
    //trim the content len, since we're just working without it/don't care.
    received.remove(0);
    received.remove(0);
    let mut target_uri = match std::str::from_utf8(&*received){
        Ok(res) => res,
        Err(err) => {
            //decode err = internal server error = 0x23
            response.push(0x23);
            response.extend((0 as u64).to_ne_bytes());
            stream.write_all(&*response);
            return;
        }
    };
    println!("[Info] Request is for URI {}",target_uri);
    //fecth the file locally
    let target_path = format!("srv/{}",target_uri);
    let ext = target_uri.split(".").last().unwrap();
    let file = match fs::read_to_string(target_path) {
        Ok(file) => file,
        Err(err) => {
            println!("[Err] File read err: {}",err);
            //file's missing. send a 0x22
            response.push(0x22);
            response.extend((0 as u64).to_ne_bytes());
            stream.write_all(&*response);
            return;
        }
    };



    //normal execution
    //pt.1: content type
    //note that the extensions here are purely for demo purposes. Server impls are free to do what they wish.
    match ext{
        //text (UTF8)
        "txt" => response.push(0x00),
        //gemtxt (UTF8)
        "gmi" => response.push(0x01),
        //a for ascii
        "atxt" =>{ response.push(0x02);},
        //redirect -> piper URL
        "predir" => response.push(0x20),
        //redirect -> non piper URL
        "wredir" => response.push(0x21),
        &_ => response.push(0x10)
    }
    response.extend((file.len() as u64).to_ne_bytes());
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
