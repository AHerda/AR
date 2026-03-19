use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum Operation {
    Open {
        input: Option<(String, char)>,
        output: Option<Result<(), RpcError>>,
    },
    Read {
        input: Option<usize>,
        output: Option<Result<u64, RpcError>>,
    },
    Write {
        input: Option<(Vec<u8>, usize)>,
        output: Option<Result<u64, RpcError>>,
    },
    Lseek {
        input: Option<SeekFrom>,
        output: Option<Result<u64, RpcError>>,
    },
    Chmod {
        input: Option<(String, u16)>,
        output: Option<Result<(), RpcError>>,
    },
    Unlink {
        input: Option<String>,
        output: Option<Result<(), RpcError>>,
    },
    Rename {
        input: Option<(String, String)>,
        output: Option<Result<(), RpcError>>,
    },
}

#[derive(Encode, Decode)]
pub struct Request {
    pub token: u64,
    pub seq: u64,
    pub operation: Operation,
}

#[derive(Encode, Decode)]
pub struct Response {
    pub seq: u64,
    // pub success: bool,
    pub operation: Operation,
}

#[derive(Encode, Decode)]
pub enum RpcError {
    Open,
    Read,
    Write,
    Lseek,
    Chmod,
    Unlink,
    Rename,
    NoInputs,
}

#[derive(Encode, Decode)]
pub enum SeekFrom {
    Start(u64),
    End(u64),
    Current(u64),
}
