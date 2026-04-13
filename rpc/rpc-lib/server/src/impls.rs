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
            OperationReq::Write(buffer) => OperationResp::Write(write(buffer, file, mode)),
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
    println!("Opening file {} in mode {}", pathname, mode_str);
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
        _ => {
            eprintln!(r#"Error: invalid mode, valid modes: ["r", "r+", "w", "w+", "a", "a+"]"#);
            return Err(RpcError::InvalidMode);
        }
    };

    if file.is_none() {
        *mode = None;
        eprintln!("Error while opening file");
        Err(RpcError::Open)
    } else {
        println!("Success!");
        Ok(())
    }
}

fn read(buffer_size: usize, file: &mut Option<File>, mode: &Option<Mode>) -> RpcResult<Vec<u8>> {
    println!("Reading from file");
    if file.is_none() {
        eprintln!("Error: no file open");
        return Err(RpcError::NoFile);
    }

    if mode.as_ref().unwrap() == &Mode::Write {
        eprintln!("Error: cannot read from write-only file");
        return Err(RpcError::InvalidMode);
    }

    let mut buffer = vec![0u8; buffer_size];
    if let Ok(bytes_read) = file.as_ref().unwrap().read(&mut buffer) {
        println!("Success reading {} bytes", bytes_read);
        Ok(buffer[..bytes_read].to_vec())
    } else {
        eprintln!("Error reading from file");
        Err(RpcError::Read)
    }
}

fn write(buffer: &Vec<u8>, file: &mut Option<File>, mode: &Option<Mode>) -> RpcResult<u64> {
    println!("Writing to file");
    if file.is_none() {
        eprintln!("Error: no file open");
        return Err(RpcError::NoFile);
    }
    if mode.as_ref().unwrap() == &Mode::Read {
        eprintln!("Error: cannot write to read-only file");
        return Err(RpcError::InvalidMode);
    }

    if let Ok(()) = file.as_mut().unwrap().write_all(buffer) {
        println!("Success writing {} bytes", buffer.len());
        Ok(buffer.len() as u64)
    } else {
        eprintln!("Error writing to file");
        Err(RpcError::Write)
    }
}

fn lseek(seek_from: &SeekFrom, file: &mut Option<File>) -> RpcResult<u64> {
    println!("Seeking in file");

    if file.is_none() {
        eprintln!("Error: no file open");
        return Err(RpcError::NoFile);
    }

    file.as_mut()
        .unwrap()
        .seek(seek_from.to_std())
        .map_err(|_| {
            eprintln!("Error seeking in file");
            RpcError::Lseek
        })
}

fn chmod(pathname: &str, r#mod: u32) -> RpcResult<()> {
    println!("Changing mod of file {} to {:o}", pathname, r#mod);

    if r#mod > 0o7777 {
        eprintln!("Error: invalid mode {}", r#mod);
        return Err(RpcError::InvalidMod);
    }
    let file = File::open(pathname).map_err(|_| RpcError::Open)?;
    let perms = Permissions::from_mode(r#mod);
    file.set_permissions(perms).map_err(|_| {
        eprint!("Error changing permisions");
        RpcError::Chmod
    })
}

fn unlink(pathname: &str) -> RpcResult<()> {
    println!("Removing file {}", pathname);
    std::fs::remove_file(pathname).map_err(|_| {
        eprintln!("Error unlinking file");
        RpcError::Unlink
    })
}

fn rename(old_path: &String, new_path: &String) -> RpcResult<()> {
    fs::rename(old_path, new_path).map_err(|_| {
        eprintln!("Error renaming file");
        RpcError::Rename
    })
}
