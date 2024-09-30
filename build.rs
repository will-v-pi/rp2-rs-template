//! Set up linker scripts for the rp235x-hal examples

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[cfg(all(feature = "rp2040", feature = "rp2350"))]
compile_error!(
    "\"rp2040\" and \"rp2350\" cannot be enabled at the same time - you must choose which to use"
);

#[cfg(not(any(feature = "rp2040", feature = "rp2350")))]
compile_error!("You must enable either \"rp2040\" or \"rp2350\"");

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // The file `memory.x` is loaded by cortex-m-rt's `link.x` script, which
    // is what we specify in `.cargo/config.toml` for Arm builds
    #[cfg(feature = "rp2040")]
    let memory_x = include_bytes!("rp2040.x");
    #[cfg(feature = "rp2350")]
    let memory_x = include_bytes!("rp2350.x");
    let mut f = File::create(out.join("memory.x")).unwrap();
    f.write_all(memory_x).unwrap();
    println!("cargo:rerun-if-changed=rp2040.x");
    println!("cargo:rerun-if-changed=rp2350.x");

    // The file `rp2350_riscv.x` is what we specify in `.cargo/config.toml` for
    // RISC-V builds
    let rp2350_riscv_x = include_bytes!("rp2350_riscv.x");
    let mut f = File::create(out.join("rp2350_riscv.x")).unwrap();
    f.write_all(rp2350_riscv_x).unwrap();
    println!("cargo:rerun-if-changed=rp2350_riscv.x");

    println!("cargo:rerun-if-changed=build.rs");
}
