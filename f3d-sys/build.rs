use std::fs::File;
use std::io::Write;
use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new("vendor/libf3d")
        .define("F3D_BUILD_APPLICATION", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("F3D_BINDINGS_C", "ON")
        .define("BUILD_SHARED_LIBS", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());

    println!("cargo:rustc-link-lib=dylib=f3d_c_api");

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("vendor/libf3d/c/engine_c_api.h")
        .header("vendor/libf3d/c/interactor_c_api.h")
        .header("vendor/libf3d/c/options_c_api.h")
        .header("vendor/libf3d/c/scene_c_api.h")
        .header("vendor/libf3d/c/window_c_api.h")
        .header(format!("{}/build/library/public/export.h", dst.display()))
        .clang_arg("-Ivendor/libf3d/c")
        .clang_arg(format!("-I{}/build/library/public", dst.display()))
        .generate()
        .expect("bindgen failed");

    let bindings_src = bindings
        .to_string()
        .replace("extern \"C\" {", "unsafe extern \"C\" {");

    File::create(out_dir.join("bindings.rs"))
        .and_then(|mut file| file.write_all(bindings_src.as_bytes()))
        .expect("cannot write bindings");
}
