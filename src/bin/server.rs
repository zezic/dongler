use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Debug)]
enum Entmsg {
    RecvToBc(String),
    NewClient(Sender<String>),
}

fn handle_client(mut stream: TcpStream, thread_tx: Sender<Entmsg>, brx: Receiver<String>) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    loop {
        stream
            .set_read_timeout(Some(std::time::Duration::from_millis(10)))
            .unwrap();
        match stream.read(&mut data) {
            Ok(size) => {
                // echo everything!
                let clienter = str::from_utf8(&data[0..size]).unwrap().to_string();
                thread_tx.send(Entmsg::RecvToBc(clienter)).unwrap();
                //stream.write(&data[0..size]).unwrap();
            }
            Err(_) => {}
        };
        let msg = brx.recv_timeout(std::time::Duration::from_millis(10));
        match msg {
            Ok(txt) => {
                let butter = txt.as_bytes();
                stream.write_all(butter).unwrap();
            }
            Err(egg) => {}
        }
    }
}

fn broadcaster(rx: Receiver<Entmsg>) {
    let mut clients: Vec<Sender<String>> = vec![];
    loop {
        //std::thread::sleep(std::time::Duration::from_secs(1));
        let entmsg = rx.recv().unwrap();
        match entmsg {
            Entmsg::RecvToBc(msgstring) => {
                println!("broadcaster recv msg: {}", msgstring.clone());
                for user in &clients {
                    user.send(msgstring.clone()).unwrap();
                }
            }
            Entmsg::NewClient(btx) => {
                clients.push(btx);

                println!("new client arrived");
            }
        }
    }
}

fn main() {
    let (tx, rx): (Sender<Entmsg>, Receiver<Entmsg>) = mpsc::channel();

    thread::Builder::new()
        .name("broadcast".to_string())
        .spawn(move || {
            // connection succeeded
            broadcaster(rx)
        })
        .unwrap();
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let thread_tx = tx.clone();
                let (btx, brx): (Sender<String>, Receiver<String>) = mpsc::channel();
                thread_tx.send(Entmsg::NewClient(btx)).unwrap();
                thread::Builder::new()
                    .name(format!("{}", stream.peer_addr().unwrap()))
                    .spawn(move || {
                        // connection succeeded
                        handle_client(stream, thread_tx, brx)
                    })
                    .unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
