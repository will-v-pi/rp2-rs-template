//! # GPIO 'Blinky' Example
//!
//! This application demonstrates how to control a GPIO pin on the rp235x.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
#[cfg(feature = "rp2350")]
use panic_halt as _;
#[cfg(feature = "rp2040")]
use panic_probe as _;

// Alias for our HAL crate
use hal::entry;

#[cfg(feature = "rp2350")]
use rp235x_hal as hal;

#[cfg(feature = "rp2040")]
use rp2040_hal as hal;

// use bsp::entry;
// use bsp::hal;
// use rp_pico as bsp;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
#[cfg(feature = "rp2040")]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

//`target_abi`, `target_arch`, `target_endian`, `target_env`, `target_family`, `target_feature`, `target_has_atomic`, `target_has_atomic_equal_alignment`, `target_has_atomic_load_store`, `target_os`, `target_pointer_width`, `target_thread_local`, `target_vendor`
/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
#[cfg(feature = "rp2350")]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the rp235x peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[entry]
fn main() -> ! {
    info!("Program start");
    // Grab our singleton objects
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    #[cfg(feature = "rp2040")]
    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    #[cfg(feature = "rp2350")]
    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure GPIO25 as an output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        timer.delay_ms(200);
        info!("off!");
        led_pin.set_low().unwrap();
        timer.delay_ms(200);
    }
}

/// Program metadata for `picotool info`
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"Blinky Example"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// End of file
