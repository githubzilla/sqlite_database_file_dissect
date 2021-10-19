use integer_encoding::*;

use crate::utils::error::MyError;
use crate::utils::error::ErrorKind;

pub fn cut_varint_bytes(bytes: &[u8]) -> Result<&[u8], MyError> {
    if bytes.len() == 0 {
        return Err(MyError::new(ErrorKind::InvalidVarInt));
    }
    let a0 = bytes[0];

    if a0<=240 {
        return Ok(&bytes[0..0])
    } else if a0>=241 && a0<=248 {
        if bytes.len() < 2 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=1])
    } else if a0 == 249 {
        if bytes.len() < 3 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=2])
    } else if a0 == 250 {
        if bytes.len() < 4 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=3])
    } else if a0 == 251 {
        if bytes.len() < 5 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=4])
    } else if a0 == 252 {
        if bytes.len() < 6 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=5])
    } else if a0 == 253 {
        if bytes.len() < 7 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=6])
    } else if a0 == 254 {
        if bytes.len() < 8 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=7])
    } else if a0 == 255 {
        if bytes.len() < 9 {
            return Err(MyError::new(ErrorKind::InvalidVarInt));
        } 
        return Ok(&bytes[0..=8])
    }

    Err(MyError::new(ErrorKind::InvalidVarInt))
}
