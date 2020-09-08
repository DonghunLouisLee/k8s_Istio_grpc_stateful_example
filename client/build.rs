fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("axe-protobuf/endpoint_manager.proto")?;
    tonic_build::configure()
        .out_dir("src/util")
        .compile(&["protobuf/poc.proto"], &["protobuf"])
        .expect("failed to compile");
    Ok(())
}