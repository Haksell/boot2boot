use core::arch::asm;

use crate::interrupts::DescriptorTablePointer;

#[inline]
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn lgdt(gdt: &DescriptorTablePointer) {
    unsafe {
        asm!("lgdt [{}]", in(reg) gdt, options(readonly, nostack, preserves_flags));
    }
}

#[inline]
pub fn cr3_read() -> usize {
    let cr3: usize;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
    }
    cr3
}

#[inline]
pub unsafe fn cr3_write(addr: u32) {
    unsafe {
        asm!("mov cr3, {}", in(reg) addr, options(nostack, preserves_flags));
    }
}

#[inline]
fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

#[inline]
pub fn tlb_flush(addr: u32) {
    unsafe {
        asm!("invlpg [{}]", in(reg) addr, options(nostack, preserves_flags));
    }
}

#[inline]
pub fn tlb_flush_all() {
    let value: u32;

    unsafe {
        asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
        asm!("mov cr3, {}", in(reg) value, options(nostack, preserves_flags));
    }
}
