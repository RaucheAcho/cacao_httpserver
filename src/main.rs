mod server;
mod http;
use server::Server;
use http::Request;
use http::Method;

//debut de fonction principale
fn main() {
    let server = Server::new("127.0.0.1:5000".to_string());
    server.run();
}
//fin de fonction principale

/*
 GET /user?id=10 HTTP/1.1\r\n
 HEADERS \r\n
 BODY
*/