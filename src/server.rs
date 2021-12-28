use std::net::TcpListener;

pub struct Server { // pub = public
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {  addr } // Server + auto return
    }

    pub fn run(self) { // self = this
        println!("Listening on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop { // infinite loop
            match listener.accept() { // match = must covered every responses possibilities
                Ok((stream, _)) => { // can replace args with "_" to tell we don't care of the values returned
                    
                },
                Err(e) => eprintln!("Failed to establish a conn: {}", e),
                // _ => println!("Catch All variant of res") // Like "default" in JS
            }
        }
    }
}