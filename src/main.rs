mod logger;
use std::env;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    let port_s = env::var("PORT").unwrap_or("9000".to_string());
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port = port_s.parse::<i32>().unwrap();

    logger::info(format!("Booting on {}:{}", host, port).as_str());
    
    let listener = TcpListener::bind(format!("{}:{}", host, port).as_str()).unwrap();
    //let pool = threadpool::ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}


/*
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
*/