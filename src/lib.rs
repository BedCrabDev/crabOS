#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry point
#![feature(abi_x86_interrupt)]

pub mod kernel;

pub fn init() {
    kernel::interrupts::init_idt();
    kernel::gdt::init();
    unsafe { kernel::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
// ok why did we do this its just not worknig
// guh

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::vga_buffer::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_colored {
    ($color:expr, $($arg:tt)*) => {{
        $crate::kernel::vga_buffer::_print_colored(format_args!($($arg)*), $color);
    }};
}

#[macro_export]
macro_rules! println_colored {
    ($color:expr) => ($crate::print_colored!($color, "\n"));
    ($color:expr, $($arg:tt)*) => ($crate::print_colored!($color, "{}\n", format_args!($($arg)*)));
}
