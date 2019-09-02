use std::io::prelude::*;
use std::io::{Result, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use protobuf::{Message, CodedInputStream};
use nuitalog_rust::EjaculationRequest;

fn main() -> Result<()> {
    let server = TcpListener::bind("127.0.0.1:8000")?;

    for client in server.incoming() {
        handle_client(client?)?;
    }
    Ok(())
}

fn handle_client(client: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(&client);
    let mut writer = BufWriter::new(&client);
    let request = read_request(&mut reader)?;
    println!("Received: {}", request.url);
    Ok(())
}

fn read_request(reader: &mut impl BufRead) -> Result<EjaculationRequest> {
    let mut result = EjaculationRequest::new();
    let mut stream = CodedInputStream::from_buffered_reader(reader);
    result.merge_from(&mut stream)?;

    Ok(result)
}
