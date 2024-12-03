use std::env;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    let bindgen = bindgen::Builder::default()
        .generate_comments(true)
        // .header("./liquid-dsp/include/liquid.h");
        .header("/usr/include/liquid/liquid.h");

    let bindgen = if env::var("CARGO_FEATURE_NUM_COMPLEX").is_ok() {
        bindgen
            .blocklist_type("liquid_float_complex")
            .blocklist_type("liquid_double_complex")
    } else {
        bindgen
    };

    let bindgen = bindgen.generate().expect("Unable to generate bindings");

    bindgen
        .write_to_file(format!("{}/liquid.rs", out_dir))
        .expect("Couldn't write bindings!");

    // add library search path
    // println!("cargo:rustc-link-search=./liquid-dsp/");
    // println!("cargo:rustc-link-lib=static=liquid");
    println!("cargo:rustc-link-lib=dylib=liquid");

    println!("cargo:rustc-link-lib=dylib=fftw3f");
}
