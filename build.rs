use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .file_descriptor_set_path(out_dir.join("user_descriptor.bin"))
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap();


    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap();
}