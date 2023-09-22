use std::io::Result;
fn main() -> Result<()> {
    // Invoke the prost protoc compiler. Just point it to a list of files you want to compile.
    // The 2nd argument is a list of paths to include directories for protobuf include statements
    prost_build::compile_protos(&["src/proto/test-jigs.proto"], &["src/proto"])?;
    Ok(())
}
