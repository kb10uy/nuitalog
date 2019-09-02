use std::error::Error;
use std::borrow::Borrow;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new("..//protobuf");

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protobuf",
        includes: &[root.to_string_lossy().borrow()],
        input: &["../protobuf/ejaculation.proto"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    })?;

    Ok(())
}
