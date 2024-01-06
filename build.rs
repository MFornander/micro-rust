// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("prost");
//     prost_build::compile_protos(&["proto/computemicro.proto", "proto/baseresponse.proto"], &["proto/"])?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("tonic-config");
    tonic_build::configure()
        .protoc_arg("--proto_path=proto")
        .compile(&["computemicro.proto", "baseresponse.proto"], &[""])?;
    Ok(())
}

// fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
//     tonic_buf_build::compile_from_buf(tonic_build::configure(), None)?;
//     Ok(())
// }
