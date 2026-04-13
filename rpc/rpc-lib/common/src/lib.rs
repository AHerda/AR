use bincode::{Decode, Encode};

pub const PACKET_SIZE: usize = 4096;

#[derive(Encode, Decode)]
pub enum OperationReq {
    Open(String, String),
    Read(usize),
    Write(Vec<u8>),
    Lseek(SeekFrom),
    Chmod(String, u16),
    Unlink(String),
    Rename(String, String),
}

pub type RpcResult<T> = Result<T, RpcError>;

#[derive(Encode, Decode, Clone)]
pub enum OperationResp {
    Open(RpcResult<()>),
    Read(RpcResult<Vec<u8>>),
    Write(RpcResult<u64>),
    Lseek(RpcResult<u64>),
    Chmod(RpcResult<()>),
    Unlink(RpcResult<()>),
    Rename(RpcResult<()>),
    JustErrors(RpcResult<()>),
}

#[derive(Encode, Decode)]
pub struct Request {
    pub auth_token: u64,
    pub seq: u64,
    pub operation: OperationReq,
}

/// Valid seq start at 1, if seq is 0 that means an error occured during decoding request
#[derive(Encode, Decode)]
pub struct Response {
    pub seq: u64,
    pub operation: OperationResp,
}

#[derive(Encode, Decode, Debug, Clone)]
pub enum RpcError {
    // FunctionErrors
    InvalidMode,
    Open,
    NoFile,
    Read,
    InvalidBufferSize,
    Write,
    Lseek,
    Chmod,
    InvalidMod,
    Unlink,
    Rename,
    NoInputs,
    // Interface errors
    UnauthorizedAccess,
    InvalidResponse,
    DecodeError,
    UnknownError,
}

#[derive(Encode, Decode)]
pub enum SeekFrom {
    Start(u64),
    End(u64),
    Current(u64),
}

impl SeekFrom {
    pub fn to_std(&self) -> std::io::SeekFrom {
        match self {
            SeekFrom::Start(n) => std::io::SeekFrom::Start(*n),
            SeekFrom::End(n) => std::io::SeekFrom::End(*n as i64),
            SeekFrom::Current(n) => std::io::SeekFrom::Current(*n as i64),
        }
    }
}
