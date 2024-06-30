#![no_std]
#![feature(
    abi_x86_interrupt,
    exclusive_range_pattern,
    ptr_metadata,
    ptr_internals
)]

#[macro_use]
mod vga_buffer;

mod interrupts;
mod keyboard;
mod memory;
mod multiboot;
mod port;
mod shell;

#[macro_use]
extern crate bitflags;

use crate::multiboot::{ElfSectionFlags, MultiBoot};
use core::arch::asm;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;

lazy_static! {
    static ref MULTIBOOT: MultiBoot =
        unsafe { MultiBoot::load(MULTIBOOT_START.load(Ordering::SeqCst)) };
}

static MULTIBOOT_START: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_start: usize) {
    MULTIBOOT_START.store(multiboot_start, Ordering::SeqCst);

    vga_buffer::WRITER.lock().clear_vga_buffer();
    shell::SHELL.lock().init();

    println!("Memory areas:");
    for area in MULTIBOOT.memory_areas() {
        println!(
            "     start: 0x{:x}, length: 0x{:x}",
            area.start_address, area.size
        );
    }

    for section in MULTIBOOT.elf_sections() {
        println!("{section:?}");
    }

    let kernel_start = MULTIBOOT
        .elf_sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address())
        .min()
        .unwrap();

    let kernel_end = MULTIBOOT
        .elf_sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.end_address())
        .max()
        .unwrap();

    println!(
        "kernel_start: {:#x}, kernel_end: {:#x}",
        kernel_start, kernel_end
    );
    println!(
        "multiboot_start: {:#x}, multiboot_end: {:#x}",
        MULTIBOOT.start_address, MULTIBOOT.end_address
    );

    let mut _frame_allocator = memory::frame::AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        MULTIBOOT.start_address,
        MULTIBOOT.end_address,
        &MULTIBOOT.memory_areas(),
    );

    interrupts::init();
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: Yellow on Black
    println!("{}", info);
    hlt_loop()
}

fn hlt_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
