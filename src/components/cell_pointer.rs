use std::mem;
use std::convert::TryFrom;

use serde_derive::Serialize;

use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;
use crate::utils::convert::TryFromBytes;

#[derive(Debug, Serialize)]
pub struct CellPointer {
    pub offset: u16,
}

impl TryFromBytes for CellPointer {
    fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        if bytes.len() != mem::size_of::<u16>() {
            return Err(MyError::new(ErrorKind::OddLength(bytes.len())));
        }

        Ok(
            CellPointer {
                offset: u16::try_from_be_bytes(bytes).unwrap(),
            }
        )
    }
    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        if bytes.len() != mem::size_of::<u16>() {
            return Err(MyError::new(ErrorKind::OddLength(bytes.len())));
        }

        Ok(
            CellPointer {
                offset: u16::try_from_be_bytes(bytes).unwrap(),
            }
        )
    }
}

impl TryFrom<&[u8]> for CellPointer {
    type Error = MyError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        CellPointer::try_from_be_bytes(value)
    }
}
