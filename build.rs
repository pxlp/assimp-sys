use std::env;

fn main() {
    let module_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    let assimp_path = format!("{}/../pixelport_deps/assimp/v3.2/{}/", module_path, target);
    println!("cargo:rustc-link-search=native={}", assimp_path);

    println!("cargo:rustc-link-lib=static=assimp");
    println!("cargo:rustc-link-lib=stdc++");


    println!("cargo:rerun-if-changed=build.rs");
}
