mod mapper;
mod table;
mod table_entry;
mod temporary_page;

use core::ops::{Add, Deref, DerefMut};

use crate::{
    instructions::{cr3_read, cr3_write, tlb_flush_all},
    vga_buffer::VGA_ADDRESS,
    MULTIBOOT,
};

use self::{mapper::Mapper, table_entry::TableEntryFlags, temporary_page::TemporaryPage};

use super::{Frame, FrameAllocator, PAGE_SIZE};

const ENTRY_COUNT: usize = 1024;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Self {
        Self {
            number: address / PAGE_SIZE,
        }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    // TODO: Rename Page Directory
    fn p2_index(&self) -> usize {
        (self.number >> 10) & 0x3ff
    }

    fn p1_index(&self) -> usize {
        self.number & 0x3ff
    }

    pub fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter { start, end }
    }
}

impl Add<usize> for Page {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        Self {
            number: self.number + rhs,
        }
    }
}

#[derive(Clone)]
pub struct PageIter {
    start: Page,
    end: Page,
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    unsafe fn new() -> Self {
        Self {
            mapper: Mapper::new(),
        }
    }

    pub fn with<F: FnOnce(&mut Mapper)>(
        &mut self,
        table: &mut InactivePageTable,
        temporary_page: &mut TemporaryPage,
        f: F,
    ) {
        {
            let backup = Frame::containing_address(cr3_read());
            let p2_table = temporary_page.map_table_frame(backup.clone(), self);
            self.p2_mut()[1023].set(
                table.p2_frame.clone(),
                TableEntryFlags::PRESENT | TableEntryFlags::WRITABLE,
            );
            tlb_flush_all();
            f(self);
            p2_table[1023].set(backup, TableEntryFlags::PRESENT | TableEntryFlags::WRITABLE);
            tlb_flush_all();
        }
        temporary_page.unmap(self);
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        let old_table = InactivePageTable {
            p2_frame: Frame::containing_address(cr3_read()),
        };
        unsafe { cr3_write(new_table.p2_frame.start_address() as u32) }
        old_table
    }
}

pub struct InactivePageTable {
    p2_frame: Frame,
}

impl InactivePageTable {
    pub fn new(
        frame: Frame,
        active_table: &mut ActivePageTable,
        temporary_page: &mut TemporaryPage,
    ) -> Self {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);
            table.zero();
            table[1023].set(
                frame.clone(),
                TableEntryFlags::PRESENT | TableEntryFlags::WRITABLE,
            );
        }
        temporary_page.unmap(active_table);
        Self { p2_frame: frame }
    }
}

pub fn remap_the_kernel<A: FrameAllocator>(allocator: &mut A) -> ActivePageTable {
    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe }, allocator);
    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };
    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        for section in MULTIBOOT.elf_sections() {
            if !section.is_allocated() {
                continue;
            }
            println!(
                "Mapping section from {:#x} to {:#x}",
                section.start_address(),
                section.end_address()
            );
            let flags = TableEntryFlags::from_elf_section_flags(&section);
            let start_frame = Frame::containing_address(section.start_address() as usize);
            let end_frame = Frame::containing_address(section.end_address() as usize - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }

        mapper.identity_map(
            Frame::containing_address(VGA_ADDRESS),
            TableEntryFlags::WRITABLE,
            allocator,
        );

        for frame in Frame::range_inclusive(
            Frame::containing_address(MULTIBOOT.start_address),
            Frame::containing_address(MULTIBOOT.end_address - 1),
        ) {
            mapper.identity_map(frame, TableEntryFlags::PRESENT, allocator);
        }
    });
    let old_table = active_table.switch(new_table);
    println!("NEW TABLE!!!");

    // TODO: stack probes (https://github.com/rust-lang/rust/issues/16012)
    let old_p4_page = Page::containing_address(old_table.p2_frame.start_address());
    active_table.unmap(old_p4_page, allocator);
    println!("Guard page at {:#x}", old_p4_page.start_address());
    active_table
}
