extern crate bindgen;
extern crate cc;

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

    // build a static lib
    cc::Build::new()
        .file("src/rpi_ws281x/mailbox.c")
        .file("src/rpi_ws281x/ws2811.c")
        .file("src/rpi_ws281x/pwm.c")
        .file("src/rpi_ws281x/pcm.c")
        .file("src/rpi_ws281x/dma.c")
        .file("src/rpi_ws281x/rpihw.c")
        // create a static lib to make cross-compiling
        // and uploading easier.
        .shared_flag(false)
        .compile("libws2811.a");

    // link to the created static lib
    println!("cargo:rustc-link-lib=static=ws2811");

    // this environment variable is declared by rustc/cargo
    // and is guaranteed to exist.
    let target = env::var("TARGET").unwrap();
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // generate an rust enum for the return type of ws2811_init (instead of the
        // the default of creating module-level consts).
        .rustified_enum("ws2811_return_t")
        .clang_arg("-target")
        .clang_arg(target);

    // Specifying the -target above seems to be sufficient for clang, but
    // just in case, allow for the user to override the toolset sysroot.
    // Note: this should be the path to the GCC ARM sysroot, *not* the libclang
    // sysroot!
    if let Ok(sysroot) = env::var("RPI_WS281X_SYSROOT") {
        builder = builder.clang_arg(format!("--sysroot={}", sysroot));
    }

    let bindings = builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
