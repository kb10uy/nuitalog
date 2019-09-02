use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let protobuf_files = vec!["protobuf/EjaculationRequest.proto"];

    for file in protobuf_files.iter() {
        let customize = protoc_rust::Customize {
            ..Default::default()
        };
        protoc_rust::run(protoc_rust::Args {
            input: &[file],
            out_dir: "src/protobuf",
            includes: &[],
            customize,
        })?;
    }

    Ok(())
}
