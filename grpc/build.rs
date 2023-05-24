fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/product.proto")?;
    tonic_build::configure()
        .out_dir("src")
        .build_server(true)
        .compile(&["proto/product.proto"], &["proto"])?;
    Ok(())
}
