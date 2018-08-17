# Welcome to the Rust Bindings for rpi_ws281x
This project is going through major overhauls, so expect breaking
changes on most version changes.

## API Design - Help Wanted
Please take a look at the `feature-new-api` branch and issue #1 and give
some feedback!

## Usage
Currently in major flux, more details when I finish the "Stage 1" overhaul.
The current version on crates.io is pretty close to a bare bindgen wrap of
the C library.  The next version will look a lot more like typical Rust,
inspired by the efforts of @Meh who wrote manual bindings to this libray
a while back.

## Contributing
Code is licensed under the MIT license, so as long as you are cool with
that, feel free to open an issue, talk about proposed changes, then open
a PR!  I would love a helping hand, just have to make sure things don't
get too messy either.

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
