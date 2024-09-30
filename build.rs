//! Set up linker scripts for the rp235x-hal examples

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use regex::Regex;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rp2040)");
    println!("cargo::rustc-check-cfg=cfg(rp2350)");

    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // The file `memory.x` is loaded by cortex-m-rt's `link.x` script, which
    // is what we specify in `.cargo/config.toml` for Arm builds
    let target;
    if true {
        target = "thumbv6m-none-eabi";
        let memory_x = include_bytes!("rp2040.x");
        let mut f = File::create(out.join("memory.x")).unwrap();
        f.write_all(memory_x).unwrap();
        println!("cargo::rustc-cfg=rp2040");
        println!("cargo:rerun-if-changed=rp2040.x");
    } else {
        if false {
            target = "thumbv8m.main-none-eabihf";
        } else {
            target = "riscv32imac-unknown-none-elf";
        }
        let memory_x = include_bytes!("rp2350.x");
        let mut f = File::create(out.join("memory.x")).unwrap();
        f.write_all(memory_x).unwrap();
        println!("cargo::rustc-cfg=rp2350");
        println!("cargo:rerun-if-changed=rp2350.x");
    }

    let re = Regex::new(r"target = .*").unwrap();
    let config_toml = include_str!(".cargo/config.toml");
    let result = re.replace(config_toml, format!("target = \"{}\"", target));
    let mut f = File::create(".cargo/config.toml").unwrap();
    f.write_all(result.as_bytes()).unwrap();

    // The file `rp2350_riscv.x` is what we specify in `.cargo/config.toml` for
    // RISC-V builds
    let rp2350_riscv_x = include_bytes!("rp2350_riscv.x");
    let mut f = File::create(out.join("rp2350_riscv.x")).unwrap();
    f.write_all(rp2350_riscv_x).unwrap();
    println!("cargo:rerun-if-changed=rp2350_riscv.x");

    println!("cargo:rerun-if-changed=build.rs");
}
