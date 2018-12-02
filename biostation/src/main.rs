//! A PlayStation 2 BIOS written in Rust.

#![no_std]
#![no_main]
#![deny(missing_docs)]
#![feature(asm)]
#![feature(naked_functions)]

extern crate prussia_dma as dma;
extern crate prussia_intc as intc;

use core::fmt::Write;
use core::panic::PanicInfo;

use prussia_debug::EEOut;
use prussia_rt::interrupts;

mod exceptions;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Write the panic message to EE output.
    let mut out = EEOut;
    writeln!(out, "{}", info).unwrap();

    // Then crash to trigger the emulator.
    unsafe { asm!("break") };
    unreachable!();
}

#[no_mangle]
fn main() -> ! {
    // Disable spurious interrupts while setting up.
    interrupts::disable();

    // Stop interrupts from the interrupt controller.
    // Remember that writing 1 to a mask toggles it; so writing it to itself clears it.
    intc::Mask::load().store();

    // And from the DMA controller.
    dma::Status::load().store();

    // Enable interrupts now that we've set everything up.
    interrupts::enable();

    panic!("Hello, World");

    //loop {}
}