use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::io::{Read};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        eprintln!("Failed to parse a req: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server { // pub = public
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {  addr } // Server + auto return
    }

    pub fn run(self, mut handler: impl Handler) { // self = this
        println!("Listening on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop { // infinite loop
            match listener.accept() { // match = must covered every responses possibilities
                Ok((mut stream, _)) => { // can replace args with "_" to tell we don't care of the values returned
                    let mut buffer = [0; 1024];                
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Receive a request: {:?}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to parse a req: {}", e);
                            }
                        },
                        Err(e) => eprintln!("Failed to read from conn: {}", e)
                    }
                },
                Err(e) => eprintln!("Failed to establish a conn: {}", e),
                // _ => println!("Catch All variant of res") // Like "default" in JS
            }
        }
    }
}