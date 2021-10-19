use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    OddLength(usize/*odd length*/),
    SliceLengthError(usize/*expected_length*/, usize/*actual_length*/),
    UnknowPageType(u8/*the page type found*/),
    NotImplemented,
    InvalidVarInt,
    UnreachableCode,
}

#[derive(Debug)]
pub struct MyError {
    error_kind: ErrorKind,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self.error_kind {
            ErrorKind::OddLength(odd_length) => write!(f, "The slice length should not be odd {}", odd_length),
            ErrorKind::SliceLengthError(expected_length, actual_length) => write!(f, "The length of slice is not correct! The expected length is {}, but the actual length is {}", expected_length, actual_length),
            ErrorKind::UnknowPageType(actual_page_type) => write!(f, "The page type {} doesn't exist.", actual_page_type),
            ErrorKind::NotImplemented => write!(f, "Function not implemented."),
            ErrorKind::InvalidVarInt => write!(f, "Invalid Variable-Length Integer."),
            ErrorKind::UnreachableCode => write!(f, "Unreachable code."),
            _ => write!(f, "Error in MyError, Unknow error kind!"),
        }; 
        result
    }
}

impl Error for MyError {}

impl MyError {
    pub fn new(error_kind: ErrorKind) -> Self {
        MyError {
            error_kind,
        }
    }
}
