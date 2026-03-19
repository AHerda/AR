use std::net::UdpSocket;

use bincode::config;
use common::{Request, Response};

mod impls;
use impls::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:4444").unwrap();
    let mut buf = [0u8; 4096];

    let config = config::standard();

    loop {
        let (len, addr) = socket.recv_from(&mut buf).expect("Didn't receive any data");
        let mut req: Request = bincode::decode_from_slice(&buf[..len], config)?.0;

        println!("Request seq: {}", req.seq);

        let success = req.operation.exec();

        let resp = Response {
            seq: req.seq,
            operation: req.operation,
        };

        let data = bincode::encode_to_vec(&resp, config).unwrap();
        socket.send_to(&data, addr).unwrap();
    }
}

trait Exec {
    fn exec(&mut self);
}
