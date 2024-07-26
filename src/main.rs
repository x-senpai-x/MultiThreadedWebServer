//We should not create a lot of threads in a pool to prevent a DOS attack
//Thread Pool : A group of threads that are created at the start of the program and are kept alive for the duration of the program
//Task assigned to a thread in the pool 
//That thread is processing and if a new task is assigned it will be assigned to one of the remaining threads
//Pool maintains queue of incoming requests
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,    
};
use MultiThreadedWebS::ThreadPool;
fn main() {
    let listener=TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    //4 indicates number of elements we use in a collection of threads 
    for stream in listener.incoming(){
        let stream=stream.unwrap();
        /* 
        we dont want infinite threads so this method can't be used 
        thread::spawn(||{
            handle_connection(stream)
        });*/
        //rather the task of creating threads is given to threadpool
        pool.execute(||{
            handle_connection(stream);
        })

    }
    }
    fn handle_connection(mut stream:TcpStream){
        let buf_reader= BufReader::new(&mut stream);
        //Bufreader is a wrapper around a reader that buffers the input stream
    
        let request_line= buf_reader.lines().next().unwrap().unwrap();
        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "front-end/hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
    
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
    }

    //Connection is submitted as a task to threadpool using poool.execute