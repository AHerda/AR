use std::{fs::File, net::UdpSocket};

use bincode::config;
use common::{OperationResp, PACKET_SIZE, Request, Response, RpcError};

mod impls;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4444").unwrap();
    let mut buf = [0u8; PACKET_SIZE];

    let mut last_call_cache: Option<(u64, OperationResp)> = None;

    let config = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
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
                    } else if let Some((last_seq, last_resp)) = last_call_cache.as_ref()
                        && *last_seq >= seq
                    {
                        if *last_seq > seq {
                            println!(
                                "Skipping request with already executed seq {} while on seq {}",
                                seq, last_seq
                            );
                            continue;
                        }
                        println!("Returning last made operation with seq: {}", seq);
                        last_resp.clone()
                    } else {
                        let operation_res = req.operation.exec(&mut file, &mut mode);
                        last_call_cache = Some((seq, operation_res.clone()));
                        operation_res
                    }
                }
                Err(e) => {
                    eprintln!("Error: failed to decode the data with error: {:?}", e);
                    OperationResp::JustErrors(Err(RpcError::DecodeError))
                }
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
