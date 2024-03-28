fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protobuffs/logger-service/logger-service.proto")?;
    Ok(())
}
