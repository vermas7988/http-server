use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // e.g. request: GET /search?name=abc HTTP1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request_str = str::from_utf8(buf)?;
        let (method_str, request_str) =
            get_next_word(request_str).ok_or(ParseError::InvalidRequest)?;
        let (path_str, request_str) =
            get_next_word(request_str).ok_or(ParseError::InvalidRequest)?;
        let (protocol_str, _) = get_next_word(request_str).ok_or(ParseError::InvalidRequest)?;

        if protocol_str != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method_str.parse()?;

        let (query_string, path_str_without_query) = if let Some(i) = path_str.find('?') {
            (Some(&path_str[i + 1..]), &path_str[..i])
        } else {
            (None, path_str)
        };

        Ok(Self {
            path: path_str_without_query,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
