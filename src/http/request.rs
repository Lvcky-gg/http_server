use std::error::Error;
use super::method::Method;
use std::convert::TryFrom;
use std::fmt::Display;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method:Method,
}









impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }


}

impl Display for ParseError {

}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,

}

impl Error for ParseError{}
