use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::complile_protos("protos/client.proto")?;
    Ok(())
}