fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/client_server.proto")?;
    tonic_build::compile_protos("../proto/local_management.proto")?;
    Ok(())
}