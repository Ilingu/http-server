use http::Request;
use http::Method;
use server::Server;

mod server; // Import ./server.rs
mod http; // Import ./http

/* MAIN */
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
