use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{BufRead, BufReader, Write},
};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    // default 404 response, may be overwritten
    let mut response: String = "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found\r\n".to_string();

    // either use .nth(1).unwrap() or .collect::<Vec<&str>>()[1] after the split
    let path = http_request.split_whitespace().nth(1).unwrap(); // path from the request
    
    if path == "/" { // main index.html file
        let contents = fs::read_to_string("./pages/index.html").unwrap();
        let length = contents.len();

        response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");
    }
    else {
        // try to find any other path
        let pages = fs::read_dir("./pages").unwrap();
        for page in pages {
            let page = page.unwrap().file_name().into_string().unwrap();

            if path == format!("/{}", page) { // path includes an / at the beginning
                println!("./pages/{}/index.html", page);
                let contents = fs::read_to_string(format!("./pages/{}/index.html", page)).unwrap_or_else(|_| {"404 Not Found".to_string()});
                let length = contents.len();
                response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");
            }
        }
    }

    stream.write_all(response.as_bytes()).unwrap();
}