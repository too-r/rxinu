#![feature(abi_x86_interrupt)]
#![feature(alloc, allocator_api, global_allocator)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_max_value)]
#![feature(const_unique_new, const_atomic_usize_new)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(unique)]
#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

extern crate bit_field;
extern crate compiler_builtins;
extern crate linked_list_allocator;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86;

#[macro_use]
pub mod arch;
pub mod device;
pub mod scheduling;
pub mod syscall;

use alloc::String;
use arch::memory::heap::{HEAP_SIZE, HEAP_START};

#[no_mangle]
/// Entry point for rust code
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    arch::interrupts::disable();
    {
        arch::init(multiboot_information_address);

        kprintln!("\nIt did not crash!");
    }
    arch::interrupts::enable();

    kprintln!("\nHEAP START = 0x{:x}", HEAP_START);
    kprintln!("HEAP END = 0x{:x}\n", HEAP_START + HEAP_SIZE);

    syscall::create(rxinu_main, String::from("rxinu_main"));

    loop {
        #[cfg(feature = "serial")]
        {
            use device::uart_16550 as uart;
            uart::read(1024);
        }
        #[cfg(feature = "vga")]
        {
            use device::keyboard::ps2 as kbd;
            kbd::read(1024);
        }
    }
}

/// Main initialization process for rxinu
pub extern "C" fn rxinu_main() {
    arch::console::clear_screen();

    kprintln!("In main process!\n");
    syscall::create(created_process, String::from("rxinu_test"));
}

pub extern "C" fn test_process() {
    kprintln!("In test process!");
}

pub extern "C" fn created_process() {
    kprintln!("\nIn rxinu_main::created_process!");
    kprintln!("\nYou can now type...");
}

pub extern "C" fn cycle_process_a() {
    kprint!(".");
    syscall::create(cycle_process_b, String::from("cycle_process_b"));
}

pub extern "C" fn cycle_process_b() {
    kprint!(".");
    syscall::create(cycle_process_a, String::from("cycle_process_a"));
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    kprintln!("\n\nPANIC in {} at line {}:", file, line);
    kprintln!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

use arch::memory::heap::HeapAllocator;

#[global_allocator]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
