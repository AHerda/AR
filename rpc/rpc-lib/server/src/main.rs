use std::{fs::File, net::UdpSocket};

use bincode::config;
use common::{OperationResp, Request, Response, RpcError};

mod impls;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:4444").unwrap();
    let mut buf = [0u8; 4096];

    let config = config::standard();
    let mut file: Option<File> = None;
    let mut mode: Option<Mode> = None;

    loop {
        let (len, addr) = socket.recv_from(&mut buf).expect("Didn't receive any data");
        let mut seq: u64 = 0;
        let operation_result: OperationResp =
            match bincode::decode_from_slice::<Request, _>(&buf[..len], config) {
                Ok((req, _)) => {
                    seq = req.seq;
                    if !authorize(req.auth_token) {
                        eprintln!("ERROR: Unauthorized access!!!!");
                        OperationResp::JustErrors(Err(RpcError::UnauthorizedAccess))
                    } else {
                        req.operation.exec(&mut file, &mut mode)
                    }
                }
                Err(_) => OperationResp::JustErrors(Err(RpcError::DecodeError)),
            };

        let resp = Response {
            seq: seq,
            operation: operation_result,
        };

        let data = bincode::encode_to_vec(&resp, config).unwrap();
        socket.send_to(&data, addr).unwrap();
    }
}

#[derive(PartialEq, Eq)]
enum Mode {
    Read,
    Write,
    ReadAndWrite,
}

trait Exec {
    fn exec(&self, file: &mut Option<File>, mode: &mut Option<Mode>) -> OperationResp;
}

fn authorize(_auth_token: u64) -> bool {
    // TODO: todo!("Some logic to authorize client");
    true
}
