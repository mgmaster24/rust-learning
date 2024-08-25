use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not create the TCP listener");
    let pool = hello_web_server::ThreadPool::build(4).expect("Error building thread pool.");
    for steam in listener.incoming().take(2) {
        pool.execute(|| {
            handle_connection(steam.expect("Could not fetch incoming stream"));
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&mut stream);
    let request_line = buf_read.lines().next();
    match request_line {
        Some(o) => match o {
            Ok(r) => {
                let (status_line, filename) = match r.as_str() {
                    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
                    "GET /sleep HTTP/1.1" => {
                        thread::sleep(Duration::from_secs(5));
                        ("HTTP/1.1 200 OK", "hello.html")
                    }
                    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
                };
                let contents = fs::read_to_string(filename).expect("Error reading html file.");
                let length = contents.len();
                stream
                    .write_all(
                        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
                            .as_bytes(),
                    )
                    .expect("Error sending response to client");
            }
            Err(e) => {
                panic!("Error getting next like from request. {e}")
            }
        },
        None => (),
    };
}
