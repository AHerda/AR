use std::{fs::File, net::UdpSocket};

use bincode::config;
use common::{OperationResp, Request, Response};

mod impls;
use impls::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:4444").unwrap();
    let mut buf = [0u8; 4096];

    let config = config::standard();
    let mut file: Option<File> = None;
    let mut mode: Option<Mode> = None;

    loop {
        let (len, addr) = socket.recv_from(&mut buf).expect("Didn't receive any data");
        let mut req: Request = bincode::decode_from_slice(&buf[..len], config)?.0;

        println!("Request seq: {}", req.seq);

        let response = req.operation.exec(&mut file, &mut mode);

        let resp = Response {
            seq: req.seq,
            operation: response,
        };

        let data = bincode::encode_to_vec(&resp, config).unwrap();
        socket.send_to(&data, addr).unwrap();
    }
}

#[derive(PartialEq, Eq)]
enum Mode {
    Read,
    Write,
}

trait Exec {
    fn exec(&self, file: &mut Option<File>, mode: &mut Option<Mode>) -> OperationResp;
}
