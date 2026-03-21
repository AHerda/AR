use bincode::config;
use common::{OperationReq, OperationResp, Request, Response, RpcError, RpcResult, SeekFrom};
use std::net::UdpSocket;
use std::time::Duration;

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

        let config = config::standard();
        let encoded = bincode::encode_to_vec(&req, config).map_err(|_| ClientError::EncodeError)?;

        let mut buf = [0u8; 4096];
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
                Err(ref e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    result = Err(ClientError::Timeout);
                    continue; // Retry
                }
                Err(_) => return Err(ClientError::UnknownError),
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

    pub fn read(&mut self, len: usize) -> Result<RpcResult<Vec<u8>>, ClientError> {
        match self.call(OperationReq::Read(len))? {
            OperationResp::Read(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
    }

    pub fn write(&mut self, data: Vec<u8>) -> Result<RpcResult<u64>, ClientError> {
        let len = data.len();
        match self.call(OperationReq::Write(data, len))? {
            OperationResp::Write(res) => Ok(res),
            _ => Ok(Err(RpcError::InvalidResponse)),
        }
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
