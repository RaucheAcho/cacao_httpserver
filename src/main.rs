struct Server{
    addr:String,
}

//début requête
enum Method {
    GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH
}
struct Request{
    path:String,
    query:Option<String>,
    method:Method
}
//fin requête

// methode pour Server instanciation et methode d'instance
impl Server{
    fn new(addr:String)-> Self {
        Server{
            addr:addr,
        }
    }
    fn run(self){
       println!("cacao a bien démarré sur {} !", self.addr);
    }
}
// fin methode pour server



//debut de fonction principale
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
//fin de fonction principale

/*
 GET /user?id=10 HTTP/1.1\r\n
 HEADERS \r\n
 BODY
*/