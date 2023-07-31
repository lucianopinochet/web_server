use std::{io::Write, fs};
#[warn(unused_imports)]
use std::{
	net::{TcpListener, TcpStream},
	io::{prelude::BufRead, BufReader},
	thread,
	time::Duration
};
use web_server::ThreadPool;

fn main() -> std::io::Result<()>{
  let listener:TcpListener = TcpListener::bind("127.0.0.1:3000")?;
	let pool = ThreadPool::new(4);
	
	for stream in listener.incoming().take(10){
			let stream = stream.unwrap();

			pool.execute(|| {
				handle_connection(stream)
			});
	}
	Ok(())
}

fn handle_connection(mut stream: TcpStream){
	let buf_reader:BufReader<&mut TcpStream> = BufReader::new(&mut stream);
	let http_request:String = buf_reader.lines().next().unwrap().unwrap();

	let (status, content) = match &http_request[..]{
		"GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "home.html"),
		"GET /other HTTP/1.1" => ("HTTP/1.1 200 OK", "other.html"),
		"GET /sleep HTTP/1.1" => {
			thread::sleep(Duration::from_secs(5));
			("HTTP/1.1 200 OK", "home.html")
		},
		_ => ("HTTP/1.1 404 NOT FOUND", "error_404.html")
	};
  let content = fs::read_to_string(content).unwrap();
	let length = content.len();
	let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");
	
	stream.write_all(response.as_bytes()).unwrap();
	
	println!("{http_request:#?}")
}