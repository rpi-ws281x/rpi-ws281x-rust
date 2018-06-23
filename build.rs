extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={:?}", out_path);

    let _output = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .arg("--remote")
        .output()
        .expect("Failed to execute hostname command.");

    gcc::Build::new()
        .file("src/rpi_ws281x/mailbox.c")
        .file("src/rpi_ws281x/ws2811.c")
        .file("src/rpi_ws281x/pwm.c")
        .file("src/rpi_ws281x/pcm.c")
        .file("src/rpi_ws281x/dma.c")
        .file("src/rpi_ws281x/rpihw.c")
        .shared_flag(true)
        .compile("libws2811.so");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=libws2811.so");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings/bindings.rs")
        .expect("Couldn't write bindings!");
}
