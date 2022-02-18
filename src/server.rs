use std::io::Read;
use std::net::TcpListener;

pub struct Server{
    addr:String,
}
// methode pour Server instanciation et methode d'instance
impl Server{
   pub fn new(addr:String)-> Self {
        Server{
            addr:addr,
        }
    }
   pub fn run(self){
       println!("cacao a bien démarré sur {} !", self.addr);
       let listener = TcpListener::bind(&self.addr).unwrap();
       loop{
           match listener.accept() {
               Ok((mut stream, _)) =>{
                   let mut buffer = [0; 1024];
                   match stream.read(&mut buffer){
                       Ok(_)=>{
                           println!("la requête est accepté: {}", String::from_utf8_lossy(&buffer))
                       },
                       Err(e) =>{
                           println!("la connexion na pas pus être établit {}", e)
                       }
                   }
               },
               Err(e) =>{println!("{}", e)}
           }
            
       }
    }
}