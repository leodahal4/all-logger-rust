fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["protos/logService.proto"], &["protos/"])
        .unwrap_or_else(|e| panic!("unable to compile the protos {}",e));
    Ok(())
}
