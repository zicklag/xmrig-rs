use std::{env, ffi::OsString, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut cfg = cmake::Config::new("xmrig");

    cfg.profile("Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .no_build_target(true);

    if let Some(include) = std::env::var_os("DEP_ARGON2_INCLUDE") {
        let mut arg = OsString::from("-I");
        arg.push(include);
        cfg.cflag(arg);
    }
    if let Some(include) = std::env::var_os("DEP_OPENSSL_INCLUDE") {
        let mut arg = OsString::from("-I");
        arg.push(include);
        cfg.cflag(arg);
    }
    if let Some(include) = std::env::var_os("DEP_UV_INCLUDE") {
        let mut arg = OsString::from("-I");
        arg.push(include);
        cfg.cflag(arg);
    }
    if let Some(include) = std::env::var_os("DEP_HWLOC_INCLUDE") {
        let mut arg = OsString::from("-I");
        arg.push(include);
        cfg.cflag(arg);
    }

    cfg.build();

    println!("cargo:rerun-if-changed=xmrig");

    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("build").display()
    );

    println!(
        "cargo:rustc-link-search={}",
        out_dir
            .join("build")
            .join("src")
            .join("3rdparty")
            .join("argon2")
            .display()
    );

    println!(
        "cargo:rustc-link-search={}",
        out_dir
            .join("build")
            .join("src")
            .join("3rdparty")
            .join("libethash")
            .display()
    );

    println!(
        "cargo:rustc-link-search={}",
        out_dir
            .join("build")
            .join("CMakeFiles")
            .join("xmrig.dir")
            .join("src")
            .display()
    );

    println!("cargo:rustc-link-lib=static=xmrig");
    println!("cargo:rustc-link-lib=static=xmrig-asm");
    println!("cargo:rustc-link-lib=static=ethash");
    println!("cargo:rustc-link-lib=static=argon2");
    println!("cargo:rustc-link-lib=static=argon2-avx512f");
    println!("cargo:rustc-link-lib=static=argon2-sse2");
    println!("cargo:rustc-link-lib=static=argon2-ssse3");
    println!("cargo:rustc-link-lib=static=argon2-avx2");
    println!("cargo:rustc-link-lib=static=argon2-xop");
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=static=uv");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=rt");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=hwloc");
    // println!("cargo:rustc-link-lib=static=stdc++");
}
