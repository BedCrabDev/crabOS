// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use crab_os::{println, println_colored};
use crab_os::kernel::vga_buffer::{Color, ColorCode};
extern crate alloc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_colored!(ColorCode::new(Color::LightRed, Color::Black), "{}", info);
    crab_os::hlt_loop()
}


use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Page;
use crab_os::kernel::allocator;
use crab_os::kernel::memory::BootInfoFrameAllocator;
use crab_os::kernel::task::executor::Executor;
use crab_os::kernel::task::Task;

entry_point!(kernel_main);


#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use crab_os::kernel::memory;
    use x86_64::{VirtAddr};

    println!("Initializing crabOS...");
    crab_os::init();
    println!("Initialized!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(crab_os::kernel::task::keyboard::print_keypresses()));
    executor.run();

    crab_os::hlt_loop()
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
