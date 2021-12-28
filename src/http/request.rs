use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>, // Optional params, can be "none" or a string
    method: Method,
}