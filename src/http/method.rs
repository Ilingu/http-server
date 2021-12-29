use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET, // 0 // GET will contain a String
    DELETE, // 1 // DELETE will contain a 64byte uint
    POST, // 2
    PUT, // ...
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;