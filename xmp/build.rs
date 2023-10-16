use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/xmp.c");
    println!("cargo:rerun-if-changed=src/xmp.h");

    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let outfile = outdir.join("bindings.rs");
    let fuselib = pkg_config::probe_library("fuse3").expect("couldn't find fuse3");

    if cfg!(debug_assertions) {
        cc::Build::new()
            .file("./src/xmp.c")
            .includes(&fuselib.include_paths)
            .flag("-O0")
            .flag("-g")
            .compile("xmp");
    } else {
        cc::Build::new()
            .file("./src/xmp.c")
            .includes(&fuselib.include_paths)
            .flag("-O3")
            .compile("xmp");
    }

    let bindings = bindgen::Builder::default()
        .clang_args(
            fuselib
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .header("./src/xmp.h")
        .derive_default(true)
        .generate()
        .expect("failed to generate bindings");

    bindings.write_to_file(&outfile).unwrap();

    let bindings_raw = fs::read_to_string(&outfile).unwrap();

    let functions_start = bindings_raw
        .find("extern \"C\" {\n    pub fn xmp_init(\n")
        .expect("couldn't find start of xmp functions");

    let functions_end = bindings_raw
        .find("#[repr(C)]\n#[derive(Debug, Default, Copy, Clone)]\npub struct __locale_data")
        .expect("couldn't find end of xmp functions");

    let bindings = format!(
        "use fuse_sys::*;\n{}",
        &bindings_raw[functions_start..functions_end]
    );

    fs::write(&outfile, bindings).unwrap();
}
