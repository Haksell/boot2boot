use core::ptr::Pointee;

pub enum TagType {
    End,
    Mmap,
    ElfSections,
    Custom(u32),
}

impl From<u32> for TagType {
    fn from(value: u32) -> Self {
        match value {
            0 => TagType::End,
            6 => TagType::Mmap,
            9 => TagType::ElfSections,
            c => TagType::Custom(c),
        }
    }
}

impl From<TagType> for u32 {
    fn from(value: TagType) -> Self {
        match value {
            TagType::End => 0,
            TagType::Mmap => 6,
            TagType::ElfSections => 9,
            TagType::Custom(c) => c,
        }
    }
}

#[repr(C)]
pub struct Tag {
    pub typ: u32,
    pub size: u32,
}

pub trait TagTrait: Pointee {
    const ID: TagType;

    fn dst_size(base_tag: &Tag) -> <Self as Pointee>::Metadata;

    unsafe fn from_base_tag(tag: &Tag) -> &Self {
        &*core::ptr::from_raw_parts(tag as *const _ as *const (), Self::dst_size(tag))
    }
}
