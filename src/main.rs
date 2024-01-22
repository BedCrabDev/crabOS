// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;
use crate::vga_buffer::{Color, ColorCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_colored!(ColorCode::new(Color::LightRed, Color::Black), "{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("crabOS!! real");
    println!("me wen i code");
    panic!("help im bad at coding");
    loop {}
}