use std::error::Error;
use super::method::Method;
use std::convert::TryFrom;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
pub struct Request {
    path: String,
    query_string: Option<String>,
    method:Method,
}









impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }


}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter)->FmtResult{
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter)->FmtResult{
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,

}
impl ParseError {
    fn message(&self)->&str{
        match self{
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidRequest => "Invalid Request",
        }
    }
}

impl Error for ParseError{}
