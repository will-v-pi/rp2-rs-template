Compile for RP2040 with `cargo build --target=thumbv6m-none-eabi --features=rp2040`

RP2350 Arm `cargo build --target=thumbv8m.main-none-eabihf --features=rp2350`
RP2350 Risc-V `cargo build --target=riscv32imac-unknown-none-elf --features=rp2350`


There may be a cleaner way to do this, where you infer the hal from the toolchain, and then you don't need features - but I couldn't see a way to get conditional compilation working without features

Possibly needs a whole new rp2-hal crate to get it working nicely
