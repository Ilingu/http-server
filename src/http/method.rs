pub enum Method {
    GET(String), // 0 // GET will contain a String
    DELETE(u64), // 1 // DELETE will contain a 64byte uint
    POST, // 2
    PUT, // ...
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}