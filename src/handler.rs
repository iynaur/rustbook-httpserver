use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::str;

pub struct ConnectionHandler;

impl ConnectionHandler {

    pub fn handle(mut stream: TcpStream) {
        let data = ConnectionHandler::read(&mut stream);
        let (status, filename) = ConnectionHandler::get_response(data);
        dbg!(&filename);
        let html = ConnectionHandler::get_file_contents(&filename);
        // dbg!(&html);
        ConnectionHandler::write_response(stream, status, &html);
    }

    fn parse (request_line:&str) -> String {
        dbg!(&request_line);
        let mut vec_line = request_line.split_whitespace();

        match vec_line.nth(1) {
            Some(p) => 
            {
                let request_line = p.to_string();
                let mut url:Vec<&str> = request_line.split("/").collect();
                url.remove(0);
                let file = url.join("/");
                dbg!(file)
            }
            None => 
            {            
                println!("has no value");
                String::new()
            }        
        }
        
    }

    fn get_response(buffer: [u8; 512]) -> (&'static str, String) {
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        if buffer.starts_with(get) {
            ("200 OK", "hello.html".to_string())
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "hello.html".to_string())
        } else {
            let sparkle_heart = str::from_utf8(&buffer).unwrap();
            let filename = ConnectionHandler::parse(&sparkle_heart);
            ("200 OK\r\nContent-Type: application/octet-stream", filename.to_string())
        }
    }

    fn read(mut stream: &TcpStream) -> [u8; 512] {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        buffer
    }

    fn write_response(mut stream: TcpStream, status: &str, contents: &Vec<u8>) {
        // let response = ConnectionHandler::format_response(status, contents);
        stream.write("HTTP/1.1 ".as_bytes()).unwrap();
        stream.write(status.as_bytes()).unwrap();
        stream.write("\r\n\r\n".as_bytes()).unwrap();
        stream.write(contents).unwrap();
        stream.flush().unwrap();
    }

    // fn format_response(status: &str, contents: &Vec<u8>) -> String {
    //     let http_version = "HTTP/1.1"
    //     // format!("{} {}\r\n\r\n{}", http_version, status, contents)
    // }

    fn get_file_contents(filename: &str) -> Vec<u8> {
        let mut contents = vec![];

        let file = File::open(filename);

        match file {
            Ok(mut rfile) => {
                // println!("working with version: {:?}", v);
                dbg!(&rfile);
                // file.read_to_string(&mut contents).unwrap();
                rfile.read_to_end(&mut contents).unwrap();
                // dbg!(&contents);

            }
            Err(e) => {
                println!("error get_file_contents: {:?}", e);
            }
        }

        
        contents
    }
}