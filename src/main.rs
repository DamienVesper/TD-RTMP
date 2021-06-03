extern crate bytes;
extern crate slab;

extern crate rml_rtmp;
extern crate reqwest;

mod log;

mod connection;
mod server;

use std::collections::{ HashSet };
use std::net::{ TcpListener, TcpStream };
use std::sync::mpsc::{ channel, Receiver, TryRecvError };
use std::thread;
use slab::Slab;
use ::connection::{ Connection, ConnectionError, ReadResult };
use ::server::{ Server, ServerResult };

fn main () {
    let address = "0.0.0:1935";
    let listener = TcpListener::bind(&address).unwrap();

    let (stream_sender, stream_receiver) channel();

    thread::spawn(|| { handle_connections(stream_receiver) });

    log("Listening for connections on {}", address);
}
