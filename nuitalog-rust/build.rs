use std::error::Error;
use std::borrow::Borrow;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new("../protobuf");

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protobuf",
        includes: &[root.to_string_lossy().borrow()],
        input: &[
            "../protobuf/basic.proto",
            "../protobuf/ejaculation.proto",
            "../protobuf/response.proto",
        ],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    })?;

    Ok(())
}
