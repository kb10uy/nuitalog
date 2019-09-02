use std::io::{Result, BufWriter};
use std::net::TcpStream;
use protobuf::Message;

use nuitalog_rust::EjaculationRequest;

fn main() -> Result<()> {
    let client = TcpStream::connect("127.0.0.1:8000")?;
    let mut writer = BufWriter::new(&client);

    let mut request = EjaculationRequest::new();
    request.set_url("https://kb10uy.org".to_owned());
    request.write_to_writer(&mut writer)?;
    Ok(())
}
