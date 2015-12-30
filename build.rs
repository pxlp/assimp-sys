extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;

fn main() {
    // Use system libassimp if it exists
    if let Ok(..) = pkg_config::Config::new().atleast_version("3.2.0").find("assimp") {
        return
    }

    let target = env::var("TARGET").unwrap();

    // Compile assimp from source
    // Disable unnecessary stuff, it takes long enough to compile already
    let dst = Config::new("assimp")
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("LIBRARY_SUFFIX", "")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    // Link to correct versions of assimp
    // NOTE: MSVC has to link to release libs to avoid CRT mismatch
    println!("cargo:rustc-link-lib=static=assimp");

    // Link to libstdc++
    if target.contains("windows-gnu") {
        println!("cargo:rustc-flags=-l dylib=stdc++");
    } else if !target.contains("windows") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
