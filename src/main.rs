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
use x86_64::VirtAddr;
use crab_os::kernel::memory::active_level_4_table;

entry_point!(kernel_main);


#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Initializing crabOS...");
    crab_os::init();
    println!("we are so initialized");
    println!("me wen i code");


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        use x86_64::structures::paging::PageTable;

        if !entry.is_unused() {
            println_colored!(ColorCode::new(Color::LightGreen, Color::Black),"L4 Entry {}: {:?}", i, entry);


            // get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-empty entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println_colored!(ColorCode::new(Color::LightCyan, Color::Black), "  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }

    crab_os::hlt_loop()
}