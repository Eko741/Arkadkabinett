use std::{
    fs,
    io::{prelude::*, BufReader, Lines},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    process::Command
};



use hello::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| -> Result<(),()>{
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream)  -> Result<(), ()>{
    let mut buf_reader = BufReader::new(&mut stream).lines();
    
    let request_string = next_line(&mut buf_reader)?;

    let mut request_parts = request_string.split(" ")
        .nth(1)
        .unwrap_or("/")
        .split("/");
    

    request_parts.next();

    let request_type = request_parts.next().unwrap_or("404");
    
    //println!("{request_type}");

    let response: String = match request_type {
        "API" => api_request(request_parts.next().unwrap_or("")),
        "" => content_from_file("hello.html"),
        _ =>  content_from_file(request_type)
        
    };

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())

}

fn content_from_file(filename: &str) -> String{
    //println!("{filename}");
    match fs::read_to_string(filename) {

        Ok(content) => ok_header(&content),

        Err(_) => {
            let content = fs::read_to_string("404.html").expect("No 404 file");
            error_header(&content)
        },

    }

}

fn api_request (api: &str) -> String{
    match api {
        "start" => start_machine(),
        "stop" => stop_machine(),
        _ => error_header("Invalid API call"),
    }
}

fn start_machine() -> String{
    // Send "sudo nohup /usr/local/bin/retrogame &"
    Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/start.sh")
        .spawn();
    ok_header("Started machine succesfully")
}

fn stop_machine() -> String{
    // Send "sudo pkill retrogame"
    Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/stop.sh")
        .output();

    ok_header("Stopped machine succesfully")
}

fn error_header(content: &str) -> String{
    format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{content}", content.len())
}

fn ok_header(content: &str) -> String{
    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{content}", content.len())
}

fn next_line<'a>(lines: &'a mut Lines<BufReader<&mut TcpStream>>) -> Result<String, ()>{
    Ok(match lines.next() {
        None => return Err(()),        
        Some(r) => match r {
            Err(_) => return Err(()),
            Ok(s) => s,
        }   
    })
}
