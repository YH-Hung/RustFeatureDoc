// build.rs is a conventional build script
fn main() {
    tonic_prost_build::compile_protos("proto/route_guide.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}