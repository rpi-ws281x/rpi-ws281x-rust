[![crates.io](http://meritbadge.herokuapp.com/rs_ws281x)](https://crates.io/crates/rs_ws281x)
[![docs.rs](https://docs.rs/rs_ws281x/badge.svg)](https://docs.rs/rs_ws281x)
# Welcome to the Rust Bindings for rpi_ws281x

## PSA - Higher Level Abstraction
If you are looking at this crate and thinking "boy, I'd like to add {insert higher abstraction}"
then you may be interested in one of the below projects. This project is meant to stick strictly
to the driver's feature set (and possibly a subset of that even).

- [Candela: A networked LED control protocol](https://github.com/JMurph2015/candela)

> _Remember to audit your dependencies, we take no responsibility for any security vulnerabilities
resulting from the use of the above mentioned crates_

If you have a crate that builds on this one and want it featured here, feel free to open a PR.


## Usage Warning
This project is still a work-in-progress, and the API is very much subject to change.
Most of the major stuff is covered by fairly idiomatic Rust.  However, we are always
striving to cover more to the level of @Meh who wrote manual bindings to this libray
a while back.

## Contributing
Code is licensed under the MIT license, so as long as you are cool with
that, feel free to open an issue, talk about proposed changes, then open
a PR!  I would love a helping hand, just have to make sure things don't
get too messy either.

## Compiling on Raspbian
- Install Rust from https://rustup.rs/ 
- Run `rustup target add arm-unknown-linux-gnueabihf`
- Install `sudo apt install libclang-dev`
- Build with `cargo build`

## Cross-compiling on Ubuntu/Debian
- Install Rust from https://rustup.rs/ 
- Run `rustup target add arm-unknown-linux-gnueabihf`
- Install `sudo apt install libclang-dev gcc-arm-linux-gnueabihf`
- Create the directory `.cargo` in the root of your Rust project and create a local cargo configuration file `config.toml` in it.
- Paste the following lines into `config.toml` to always build for the `arm-unknown-linux-gnueabihf` target, to use the GCC linker for this architecture and to set the sysroot so that the linker can find the necessary header files:
```
[build]
target = "arm-unknown-linux-gnueabihf"

[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[env]
RPI_WS281X_SYSROOT = "/usr/arm-linux-gnueabihf"
```
- Build with `cargo build`

### AArch64 Builds
- Run `rustup target add aarch64-unknown-linux-gnu`
- Install cross-compiler toolchain:
`sudo apt install libclang-dev gcc-aarch64-linux-gnu`
- Build with: `cargo build --target=aarch64-unknown-linux-gnu [YOUR OPTIONS]`


## Cross-compiling on Windows

- Make sure Git is installed. This is used to clone the latest rpi-ws2811 lib.
- Download and install libclang from [LLVM]; [32-bit for Windows][1]
- Download and install [GCC][2] for the [Raspberry Pi][3].
- Using rustup, install the 32-bit GCC Rust toolchain and set as default:
    - `rustup default stable-i686-pc-windows-gnu`
- Add the Raspberry Pi architecture target to the GCC toolchain:
    - For all Raspberry Pi versions:
    - `rustup target add arm-unknown-linux-gnueabihf`
    - For an optimized Raspberry Pi 3 target (the ARMv7 architecture):
    - `rustup target add armv7-unknown-linux-gnueabihf`
- At the root of your Rust project, create a new directory and name it `.cargo`.
  Inside that directory, create a file and paste the contents:
    - ```
      [build]
      target = "arm-unknown-linux-gnueabihf"

      [target.arm-unknown-linux-gnueabihf]
      linker = "C:/SysGCC/raspberry/bin/arm-linux-gnueabihf-gcc.exe"
      ```
    - This informs Rust/Cargo to always use the arm-unknown-linux-gnueabihf target
      (so that --target doesn't always have to be passed to cargo) and so that the
      GCC linker for the given architecture is used.
    - Be sure to change the linker path or the target architecture if you installed
      the GCC ARM toolset in a different directory or are using the ARMv7 target
      instead.
- It is suggested to create a build script so that the necessary environment variables
  can be set. This library needs to know where to find the GCC toolset in order to
  cross compile the rpi-ws2811 C library for the Raspberry Pi.

An example build script might look like this (using Git for Windows Bash):

```
#!/usr/bin/env bash

# inform rpi-ws2811-rust where the GCC sysroot is
export RPI_WS281X_SYSROOT=C:/SysGCC/raspberry/arm-linux-gnueabihf/sysroot
# point to the GCC ARM compiler/linker.
export CC_arm_unknown_linux_gnueabihf=C:/SysGCC/raspberry/bin/arm-linux-gnueabihf-gcc.exe
# point to the GCC ARM archiver
export AR_arm_unknown_linux_gnueabihf=C:/SysGCC/raspberry/bin/arm-linux-gnueabihf-ar.exe

# run the build command
cargo build $@
```

[LLVM]: http://releases.llvm.org/download.html
[1]: http://releases.llvm.org/6.0.1/LLVM-6.0.1-win32.exe
[2]: http://gnutoolchains.com/raspberry/
[3]: http://sysprogs.com/files/gnutoolchains/raspberry/raspberry-gcc6.3.0-r3.exe
