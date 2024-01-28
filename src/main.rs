// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use crab_os::{println, println_colored};
use crab_os::kernel::vga_buffer::{Color, ColorCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_colored!(ColorCode::new(Color::LightRed, Color::Black), "{}", info);
    crab_os::hlt_loop()
}


use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


#[no_mangle]
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Initializing crabOS...");
    crab_os::init();
    println!("we are so initialized");
    println!("me wen i code");


    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    crab_os::hlt_loop()
}