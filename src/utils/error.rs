use std::{error::Error, fmt::write};
use std::fmt;

use serde_derive::Serialize;

use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};

#[derive(Debug, Serialize)]
pub enum ErrorKind {
    OddLength(usize/*odd length*/),
    SliceLengthError(usize/*expected_length*/, usize/*actual_length*/),
    UnknowPageType(u8/*the page type found*/),
    NotImplemented,
    InvalidVarInt,
    UnreachableCode,
}

#[derive(Debug, Serialize)]
pub struct MyError {
    error_kind: ErrorKind,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match &self.error_kind {
            ErrorKind::OddLength(odd_length) => write!(f, "The slice length should not be odd {}", odd_length),
            ErrorKind::SliceLengthError(expected_length, actual_length) => write!(f, "The length of slice is not correct! The expected length is {}, but the actual length is {}", expected_length, actual_length),
            ErrorKind::UnknowPageType(actual_page_type) => write!(f, "The page type {} doesn't exist.", actual_page_type),
            ErrorKind::NotImplemented => write!(f, "Function not implemented."),
            ErrorKind::InvalidVarInt => write!(f, "Invalid Variable-Length Integer."),
            ErrorKind::UnreachableCode => write!(f, "Unreachable code."),
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

#[derive(Debug, Serialize)]
pub enum HttpErrorKind{
    InternalError(String),
    BadClientData(String),
    Timeout(String),
    PageIndexError(usize),
}

#[derive(Debug, Serialize)]
pub struct HttpError {
    error_kind: HttpErrorKind,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match &self.error_kind {
            HttpErrorKind::InternalError(msg) => write!(f, "{}", msg),
            HttpErrorKind::BadClientData(msg) => write!(f, "{}", msg),
            HttpErrorKind::Timeout(msg) => write!(f, "{}", msg),
            HttpErrorKind::PageIndexError(page_idx) => write!(f, "Page index error: {}", page_idx),
        }; 
        result
    }
}

impl Error for HttpError {}

impl HttpError {
    pub fn new(error_kind: HttpErrorKind) -> Self {
        HttpError {
            error_kind,
        }
    }
}
impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.error_kind {
            HttpErrorKind::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HttpErrorKind::BadClientData(_) => StatusCode::BAD_REQUEST,
            HttpErrorKind::Timeout(_) => StatusCode::BAD_GATEWAY,
            HttpErrorKind::PageIndexError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

