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

    // Remove once https://github.com/alexcrichton/cmake-rs/issues/3 is fixed
    let use_mingw_temp_hack = target.contains("windows-gnu");
    if use_mingw_temp_hack {
        temp_mingw_hack();
        return;
    }

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
        println!("cargo:rustc-link-lib=static=stdc++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn temp_mingw_hack() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let assimp_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assimp");
    let assimp_path_str = assimp_path.to_str().unwrap();

    fs::create_dir_all(&out_dir).unwrap();
    temp_mingw_hack_run(Command::new("cmake").args(&[
        "-G", "MinGW Makefiles",
        "-DBUILD_SHARED_LIBS=FALSE",
        "-DASSIMP_BUILD_ASSIMP_TOOLS=OFF",
        "-DASSIMP_BUILD_TESTS=OFF",
        "-DBUILD_SHARED_LIBS=OFF",
        "-DCMAKE_BUILD_TYPE=Release",
        "-DLIBRARY_SUFFIX=",
        &assimp_path_str])
        .current_dir(&Path::new(&out_dir)), "cmake");

    temp_mingw_hack_run(Command::new("mingw32-make").args(&[
        "assimp"
        ])
        .current_dir(&Path::new(&out_dir)), "mingw32-make");

    println!("cargo:rustc-link-search=native={}", Path::new(&out_dir).join("code").to_str().unwrap());
    println!("cargo:rustc-link-lib=static=assimp");
    println!("cargo:rustc-link-lib=static=stdc++");
}
fn temp_mingw_hack_run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            panic!("failed to execute command: {}\nis `{}` not installed?",
                          e, program);
        }
        Err(e) => panic!("failed to execute command: {}", e),
    };
    if !status.success() {
        panic!("command did not execute successfully, got: {}", status);
    }
}
