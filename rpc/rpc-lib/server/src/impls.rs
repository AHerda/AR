use super::{Exec, Mode};
use common::{OperationReq, OperationResp, RpcError, SeekFrom};

use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{Read, Seek, Write};
use std::os::unix::fs::PermissionsExt;

impl Exec for OperationReq {
    fn exec(&self, file: &mut Option<File>, mode: &mut Option<Mode>) -> OperationResp {
        match self {
            OperationReq::Open(pathname, mode_char) => open(pathname, mode_char, file, mode),
            OperationReq::Read(buffer_size) => read(*buffer_size, file, mode),
            OperationReq::Write(buffer, size) => write(buffer, size, file, mode),
            OperationReq::Lseek(seek_from) => lseek(seek_from, file, mode),
            OperationReq::Chmod(pathname, r#mod) => chmod(pathname, *r#mod as u32, file, mode),
            _ => unreachable!(),
        }
    }
}

fn open(
    pathname: &String,
    mode_char: &char,
    file: &mut Option<File>,
    mode: &mut Option<Mode>,
) -> OperationResp {
    let res = match mode_char {
        'r' => {
            *mode = Some(Mode::Read);
            *file = File::open(pathname).ok();
            OperationResp::Open(Ok(()))
        }
        'w' => {
            *mode = Some(Mode::Write);
            *file = File::create(pathname).ok();
            OperationResp::Open(Ok(()))
        }
        'a' => {
            *mode = Some(Mode::Write);
            *file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(pathname)
                .ok();
            OperationResp::Open(Ok(()))
        }
        _ => return OperationResp::Open(Err(RpcError::InvalidMode)),
    };

    if file.is_none() {
        *mode = None;
        OperationResp::Open(Err(RpcError::Open))
    } else {
        res
    }
}

fn read(buffer_size: usize, file: &mut Option<File>, mode: &Option<Mode>) -> OperationResp {
    if file.is_none() {
        return OperationResp::Read(Err(RpcError::NoFile));
    }

    if mode.as_ref().unwrap() == &Mode::Write {
        return OperationResp::Read(Err(RpcError::InvalidMode));
    }

    let mut buffer = vec![0u8; buffer_size];
    if let Ok(bytes_read) = file.as_ref().unwrap().read(&mut buffer) {
        OperationResp::Read(Ok(buffer[..bytes_read].to_vec()))
    } else {
        OperationResp::Read(Err(RpcError::Read))
    }
}

fn write(
    buffer: &Vec<u8>,
    size: &usize,
    file: &mut Option<File>,
    mode: &Option<Mode>,
) -> OperationResp {
    if file.is_none() {
        return OperationResp::Write(Err(RpcError::NoFile));
    }
    if mode.as_ref().unwrap() == &Mode::Read {
        return OperationResp::Write(Err(RpcError::InvalidMode));
    }
    if size > &buffer.len() {
        return OperationResp::Write(Err(RpcError::InvalidBufferSize));
    }

    if let Ok(()) = file.as_mut().unwrap().write_all(buffer) {
        OperationResp::Write(Ok(*size as u64))
    } else {
        OperationResp::Write(Err(RpcError::Write))
    }
}

fn lseek(seek_from: &SeekFrom, file: &mut Option<File>, mode: &Option<Mode>) -> OperationResp {
    if file.is_none() {
        return OperationResp::Lseek(Err(RpcError::NoFile));
    }
    if mode.as_ref().unwrap() == &Mode::Write {
        return OperationResp::Lseek(Err(RpcError::InvalidMode));
    }

    if let Ok(offset) = file.as_mut().unwrap().seek(seek_from.to_std()) {
        OperationResp::Lseek(Ok(offset))
    } else {
        OperationResp::Lseek(Err(RpcError::Lseek))
    }
}

fn chmod(
    pathname: &str,
    r#mod: u32,
    file: &mut Option<File>,
    mode: &Option<Mode>,
) -> OperationResp {
    if r#mod > 0o7777 {
        return OperationResp::Chmod(Err(RpcError::InvalidMod));
    }
    if let Ok(file) = File::open(pathname) {
        let perms = Permissions::from_mode(r#mod);
        match file.set_permissions(perms) {
            Ok(()) => OperationResp::Chmod(Ok(())),
            Err(_) => OperationResp::Chmod(Err(RpcError::Chmod)),
        }
    } else {
        OperationResp::Chmod(Ok(()))
    }
}
