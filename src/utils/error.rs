use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    SliceLengthError(usize/*expected_length*/, usize/*actual_length*/),
}

#[derive(Debug)]
pub struct MyError {
    error_kind: ErrorKind,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self.error_kind {
            ErrorKind::SliceLengthError(expected_length, actual_length) => write!(f, "The length of slice is not correct! The expected length is {}, but the actual length is {}", expected_length, actual_length),
            _ => write!(f, "Error in MyError, Unknow error kind!")
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
