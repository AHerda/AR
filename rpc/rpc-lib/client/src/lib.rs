use bincode::config;
use common::{
    OperationReq, OperationResp, PACKET_SIZE, Request, Response, RpcError, RpcResult, SeekFrom,
};
use std::net::UdpSocket;
use std::time::Duration;

// 2 * 8 for auth_token (u64) and seq (u64)
// 4 for OperationReq enum encoding
// 8 for length of encoded vec
pub const MAX_WRITE_BUFFER: usize = PACKET_SIZE - (2 * 8 + 4 + 8);
// 8 for seq (u64)
// 4 for OperationResp enum encoding
// 4 for RcpResult enum encoding
// 8 for length of encoded vec
pub const MAX_READ_BUFFER: usize = PACKET_SIZE - (8 + 4 + 4 + 8);

#[derive(Debug)]
pub enum ClientError {
    Timeout,
    EncodeError,
    DecodeError,
    InvlaidSeq,
    UnknownError,
}

pub struct RpcServer {
    socket: UdpSocket,
    server_addr: String,
    auth_token: u64,
    next_seq: u64,
    _timeout: Duration,
}

impl RpcServer {
    pub fn new(server_addr: &str, timeout_ms: u64) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available local port
        let _timeout = Duration::from_millis(timeout_ms);
        socket.set_read_timeout(Some(_timeout))?;

        Ok(Self {
            socket,
            server_addr: server_addr.to_string(),
            auth_token: 12345, // Hardcoded auth token
            next_seq: 1,
            _timeout,
        })
    }

    /// Internal helper to send a request and wait for the specific sequence response
    fn call(&mut self, operation: OperationReq) -> Result<OperationResp, ClientError> {
        let seq = self.next_seq;
        self.next_seq += 1;

        let req = Request {
            auth_token: self.auth_token,
            seq,
            operation,
        };

        let config = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let encoded = bincode::encode_to_vec(&req, config).map_err(|_| ClientError::EncodeError)?;

        let mut buf = [0u8; PACKET_SIZE * 2];
        let mut result: Result<OperationResp, ClientError> = Err(ClientError::UnknownError);

        // Simple retry logic: try up to 3 times if we timeout
        for _ in 0..3 {
            self.socket
                .send_to(&encoded, &self.server_addr)
                .map_err(|_| ClientError::Timeout)?;

            match self.socket.recv_from(&mut buf) {
                Ok((len, _)) => {
                    let (resp, _): (Response, _) = bincode::decode_from_slice(&buf[..len], config)
                        .map_err(|_| ClientError::DecodeError)?;

                    // Ensure the response matches our sequence number
                    if resp.seq == seq || resp.seq == 0 {
                        return Ok(resp.operation);
                    } else {
                        return Err(ClientError::InvlaidSeq);
                    }
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    result = Err(ClientError::Timeout);
                    continue; // Retry
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return Err(ClientError::UnknownError);
                }
            }
        }
        result
    }

    // --- Public API Mapping ---

    pub fn open(&mut self, path: String, mode: String) -> Result<RpcResult<()>, ClientError> {
        match self.call(OperationReq::Open(path, mode))? {
            OperationResp::Open(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
    }

    pub fn read(&mut self, mut len: usize) -> Result<RpcResult<Vec<u8>>, ClientError> {
        let mut res = Vec::with_capacity(len);
        while len > 0 {
            let bytes_to_read = std::cmp::min(len, MAX_READ_BUFFER);
            len -= bytes_to_read;

            match self.call(OperationReq::Read(bytes_to_read))? {
                OperationResp::Read(Ok(buf)) => {
                    res.extend_from_slice(&buf);
                    if buf.len() != bytes_to_read {
                        break;
                    }
                }
                OperationResp::Read(Err(e)) => {
                    if res.len() == 0 {
                        return Ok(Err(e));
                    } else {
                        break;
                    }
                }
                _ => {
                    if res.len() == 0 {
                        return Ok(Err(RpcError::InvalidResponse));
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(Ok(res))
    }

    pub fn write(&mut self, data: Vec<u8>) -> Result<RpcResult<u64>, ClientError> {
        let mut sum = 0;
        for chunk in data.chunks(MAX_WRITE_BUFFER) {
            match self.call(OperationReq::Write(chunk.to_vec()))? {
                OperationResp::Write(Ok(val)) => {
                    sum += val;
                    if val != chunk.len() as u64 {
                        break;
                    }
                }
                OperationResp::Write(Err(e)) => {
                    if sum == 0 {
                        return Ok(Err(e));
                    } else {
                        break;
                    }
                }
                x => {
                    println!("Unexpected response: {:?}", x);
                    if sum == 0 {
                        return Ok(Err(RpcError::InvalidResponse));
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(Ok(sum))
    }

    pub fn lseek(&mut self, pos: SeekFrom) -> Result<RpcResult<u64>, ClientError> {
        match self.call(OperationReq::Lseek(pos))? {
            OperationResp::Lseek(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
    }

    pub fn chmod(&mut self, path: String, mode: u16) -> Result<RpcResult<()>, ClientError> {
        match self.call(OperationReq::Chmod(path, mode))? {
            OperationResp::Chmod(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
    }

    pub fn unlink(&mut self, path: String) -> Result<RpcResult<()>, ClientError> {
        match self.call(OperationReq::Unlink(path))? {
            OperationResp::Chmod(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
    }

    pub fn rename(
        &mut self,
        old_path: String,
        new_path: String,
    ) -> Result<RpcResult<()>, ClientError> {
        match self.call(OperationReq::Rename(old_path, new_path))? {
            OperationResp::Open(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
    }
}
