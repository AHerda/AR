use super::{Exec, Mode};
use common::{OperationReq, OperationResp, RpcError, RpcResult, SeekFrom};

use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{Read, Seek, Write};
use std::os::unix::fs::PermissionsExt;

impl Exec for OperationReq {
    fn exec(&self, file: &mut Option<File>, mode: &mut Option<Mode>) -> OperationResp {
        match self {
            OperationReq::Open(pathname, mode_str) => {
                OperationResp::Open(open(pathname, mode_str, file, mode))
            }
            OperationReq::Read(buffer_size) => OperationResp::Read(read(*buffer_size, file, mode)),
            OperationReq::Write(buffer, size) => {
                OperationResp::Write(write(buffer, size, file, mode))
            }
            OperationReq::Lseek(seek_from) => OperationResp::Lseek(lseek(seek_from, file)),
            OperationReq::Chmod(pathname, r#mod) => {
                OperationResp::Chmod(chmod(pathname, *r#mod as u32))
            }
            OperationReq::Unlink(pathname) => OperationResp::Unlink(unlink(pathname)),
            OperationReq::Rename(old_path, new_path) => {
                OperationResp::Rename(rename(old_path, new_path))
            }
        }
    }
}

fn open(
    pathname: &String,
    mode_str: &str,
    file: &mut Option<File>,
    mode: &mut Option<Mode>,
) -> RpcResult<()> {
    match mode_str {
        "r" => {
            *mode = Some(Mode::Read);
            *file = OpenOptions::new().read(true).open(pathname).ok();
        }
        "r+" => {
            *mode = Some(Mode::ReadAndWrite);
            *file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(pathname)
                .ok();
        }
        "w" => {
            *mode = Some(Mode::Write);
            *file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(pathname)
                .ok();
        }
        "w+" => {
            *mode = Some(Mode::ReadAndWrite);
            *file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(pathname)
                .ok();
        }
        "a" => {
            *mode = Some(Mode::Write);
            *file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(pathname)
                .ok();
        }
        "a+" => {
            *mode = Some(Mode::ReadAndWrite);
            *file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(pathname)
                .ok();
        }
        _ => return Err(RpcError::InvalidMode),
    };

    if file.is_none() {
        *mode = None;
        Err(RpcError::Open)
    } else {
        Ok(())
    }
}

fn read(buffer_size: usize, file: &mut Option<File>, mode: &Option<Mode>) -> RpcResult<Vec<u8>> {
    if file.is_none() {
        return Err(RpcError::NoFile);
    }

    if mode.as_ref().unwrap() == &Mode::Write {
        return Err(RpcError::InvalidMode);
    }

    let mut buffer = vec![0u8; buffer_size];
    if let Ok(bytes_read) = file.as_ref().unwrap().read(&mut buffer) {
        Ok(buffer[..bytes_read].to_vec())
    } else {
        Err(RpcError::Read)
    }
}

fn write(
    buffer: &Vec<u8>,
    &size: &usize,
    file: &mut Option<File>,
    mode: &Option<Mode>,
) -> RpcResult<u64> {
    if file.is_none() {
        return Err(RpcError::NoFile);
    }
    if mode.as_ref().unwrap() == &Mode::Read {
        return Err(RpcError::InvalidMode);
    }
    if size > buffer.len() {
        return Err(RpcError::InvalidBufferSize);
    }

    if let Ok(()) = file.as_mut().unwrap().write_all(&buffer[..size]) {
        Ok(size as u64)
    } else {
        Err(RpcError::Write)
    }
}

fn lseek(seek_from: &SeekFrom, file: &mut Option<File>) -> RpcResult<u64> {
    if file.is_none() {
        return Err(RpcError::NoFile);
    }

    file.as_mut()
        .unwrap()
        .seek(seek_from.to_std())
        .map_err(|_| RpcError::Lseek)
}

fn chmod(pathname: &str, r#mod: u32) -> RpcResult<()> {
    if r#mod > 0o7777 {
        return Err(RpcError::InvalidMod);
    }
    let file = File::open(pathname).map_err(|_| RpcError::Open)?;
    let perms = Permissions::from_mode(r#mod);
    match file.set_permissions(perms) {
        Ok(()) => Ok(()),
        Err(_) => Err(RpcError::Chmod),
    }
}

fn unlink(pathname: &str) -> RpcResult<()> {
    std::fs::remove_file(pathname).map_err(|_| RpcError::Unlink)
}

fn rename(old_path: &String, new_path: &String) -> RpcResult<()> {
    fs::rename(old_path, new_path).map_err(|_| RpcError::Rename)
}
