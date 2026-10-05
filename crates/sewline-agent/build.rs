fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/agent_governance.proto")?;
    Ok(())
}