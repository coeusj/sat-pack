fn main() -> Result<(), Box<dyn std::error::Error>> {
     tonic_prost_build::configure()
        .compile_protos(
            &["proto/ccsds/ccsds.proto"],
            &["proto"])
        .unwrap();

    tonic_prost_build::compile_protos("proto/ccsds/ccsds.proto").unwrap();
    return Ok(());
}