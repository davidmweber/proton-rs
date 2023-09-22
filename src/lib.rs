// Pulls in the output Rust code from the protoc compiler. The OUT_DIR variable points to
// some magical place in the build target directory
pub mod test_jigs {
    include!(concat!(env!("OUT_DIR"), "/test_jigs.rs"));
}
