use std::convert::TryFrom;
use std::result::Result;

use crate::utils::error::{MyError, ErrorKind};

pub trait TryFromByteSlice
where Self: Sized
{
    type Error;
    fn try_from_be_byte_slice(slice: &[u8]) -> Result<Self, Self::Error>; 
}

impl TryFromByteSlice for u16 {
    type Error = MyError;

    fn try_from_be_byte_slice(slice: &[u8]) -> Result<u16, MyError>{
        if slice.len() != 2 {
            return Err(MyError::new(ErrorKind::SliceLengthError(2, slice.len())));
        }
        
        Ok(u16::from_be_bytes(<[u8; 2]>::try_from(slice).unwrap()))
    }
}

impl TryFromByteSlice for u32 {
    type Error = MyError;

    fn try_from_be_byte_slice(slice: &[u8]) -> Result<u32, MyError>{
        if slice.len() != 4 {
            return Err(MyError::new(ErrorKind::SliceLengthError(4, slice.len())));
        }
        
        Ok(u32::from_be_bytes(<[u8; 4]>::try_from(slice).unwrap()))
    }
}
