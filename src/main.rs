use server::Server;
use http::request::Request;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self {
                addr
            }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            // method: super::method::Method,
            method: Method,
        }
    }

    mod method {
        pub enum Method {
            GET,
            POST,
            PUT,
            DELETE,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
