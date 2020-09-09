fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
        .compile(&["poc-protobuf/poc.proto"], &["poc-protobuf"])
        .expect("failed to compile");
    Ok(())
}