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
        .expect("Failed to execute git command.");

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
}
