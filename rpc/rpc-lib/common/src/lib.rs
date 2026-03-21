use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum OperationReq {
    Open(String, String),
    Read(usize),
    Write(Vec<u8>, usize),
    Lseek(SeekFrom),
    Chmod(String, u16),
    Unlink(String),
    Rename(String, String),
}

pub type RpcResult<T> = Result<T, RpcError>;

#[derive(Encode, Decode)]
pub enum OperationResp {
    Open(RpcResult<()>),
    Read(RpcResult<Vec<u8>>),
    Write(RpcResult<u64>),
    Lseek(RpcResult<u64>),
    Chmod(RpcResult<()>),
    Unlink(RpcResult<()>),
    Rename(RpcResult<()>),
    JustErrors(RpcResult<()>)
}

#[derive(Encode, Decode)]
pub struct Request {
    pub auth_token: u64,
    pub seq: u64,
    pub operation: OperationReq,
}

#[derive(Encode, Decode)]
pub struct Response {
    pub seq: u64,
    pub operation: OperationResp,
}

#[derive(Encode, Decode, Debug)]
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
    Timeout,
    UnauthorizedAccess,
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
