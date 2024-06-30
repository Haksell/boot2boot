mod elf_sections;
mod memory_map;
mod tag;

pub use self::{
    elf_sections::{ElfSection, ElfSectionFlags},
    memory_map::MemoryArea,
};
use self::{
    elf_sections::{ElfSectionIter, ElfSectionsTag},
    memory_map::MemoryMapTag,
    tag::{Tag, TagTrait, TagType},
};

pub struct MultiBoot {
    pub start_address: usize,
    first_tag: usize,
    pub end_address: usize,
}

impl MultiBoot {
    pub unsafe fn load(multiboot_address: usize) -> Self {
        let total_size = *(multiboot_address as *const u32) as usize;
        Self {
            start_address: multiboot_address,
            first_tag: multiboot_address + 8,
            end_address: multiboot_address + total_size,
        }
    }

    pub fn elf_sections(&self) -> ElfSectionIter {
        self.get_tag::<ElfSectionsTag>().unwrap().sections()
    }

    pub fn memory_areas(&self) -> &[MemoryArea] {
        &self.get_tag::<MemoryMapTag>().unwrap().areas
    }

    fn get_tag<T: TagTrait + ?Sized>(&self) -> Option<&T> {
        let mut current = self.first_tag as *const Tag;
        loop {
            let tag = unsafe { &*current };
            match tag.typ.into() {
                TagType::End => return None,
                _ => {
                    let ptr_offset = (tag.size as usize + 7) & !7;
                    current = unsafe { current.cast::<u8>().add(ptr_offset).cast::<Tag>() };
                    if tag.typ == T::ID.into() {
                        return Some(unsafe { TagTrait::from_base_tag(tag) });
                    }
                }
            }
        }
    }
}
