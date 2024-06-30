use crate::{
    memory::Frame,
    multiboot::{ElfSection, ElfSectionFlags},
};

pub struct TableEntry(usize);

// move this
pub const ADDRESS_MASK: usize = 0xffff_f000;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct TableEntryFlags: u32 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
    }
}

impl TableEntryFlags {
    pub fn from_elf_section_flags(section: &ElfSection) -> Self {
        let mut flags = TableEntryFlags::empty();
        if section.flags().contains(ElfSectionFlags::ALLOCATED) {
            flags |= TableEntryFlags::PRESENT;
        }
        if section.flags().contains(ElfSectionFlags::WRITABLE) {
            flags |= TableEntryFlags::WRITABLE;
        }
        flags
    }
}

impl TableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> TableEntryFlags {
        TableEntryFlags::from_bits_truncate(self.0 as u32)
    }

    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(TableEntryFlags::PRESENT) {
            Some(Frame::containing_address(self.0 & ADDRESS_MASK))
        } else {
            None
        }
    }

    pub fn set(&mut self, frame: Frame, flags: TableEntryFlags) {
        assert!(frame.start_address() & !ADDRESS_MASK == 0);
        self.0 = frame.start_address() | flags.bits() as usize;
    }
}
