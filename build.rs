fn main() {
    // Tell Cargo to re-run this script if the proto file changes
    println!("cargo:rerun-if-changed=proto/email.proto");

    prost_build::compile_protos(&["proto/email.proto"], &["proto/"])
        .expect("Failed to compile protobuf messages");
}
