use super::Exec;
use common::{Operation, RpcError};

impl Exec for Operation {
    fn exec(&mut self) {
        match self {
            Operation::Open { input, output } => {
                assert!(output.is_none());
                if let Some(input) = input {
                } else {
                    *output = Some(Err(RpcError::NoInputs));
                }
            }
            _ => (),
        }
    }
}

fn open(pathname: String, mode: char) -> Result<Option<u64>, RpcError> {
    Ok(None)
}
