use std::net::TcpListener;

mod connectionhandler;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";
use connectionhandler::handle_connection;


fn main() {
    let end_point = HOST.to_owned() + ":" + PORT;

    let listner = TcpListener::bind(end_point).unwrap();

    println!("The Web-Server is listening at port: {}",PORT);

    for stream in listner.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
        println!("Connection Established")
    }

}