use core::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::memory::FrameAllocator;

use super::{
    table_entry::{TableEntry, TableEntryFlags, ADDRESS_MASK},
    ENTRY_COUNT,
};

pub enum Level2 {}
pub enum Level1 {}

pub trait TableLevel {}

impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

// TODO: Remove Trait
pub trait HierachicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierachicalLevel for Level2 {
    type NextLevel = Level1;
}

pub const P2: *mut Table<Level2> = ADDRESS_MASK as *mut _;

pub struct Table<L: TableLevel> {
    entries: [TableEntry; ENTRY_COUNT],
    level: PhantomData<L>,
}
impl<L> Index<usize> for Table<L>
where
    L: TableLevel,
{
    type Output = TableEntry;

    fn index(&self, index: usize) -> &TableEntry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L>
where
    L: TableLevel,
{
    fn index_mut(&mut self, index: usize) -> &mut TableEntry {
        &mut self.entries[index]
    }
}

impl<L> Table<L>
where
    L: TableLevel,
{
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl<L> Table<L>
where
    L: HierachicalLevel,
{
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A: FrameAllocator>(
        &mut self,
        index: usize,
        allocator: &mut A,
    ) -> &mut Table<L::NextLevel> {
        if self.next_table(index).is_none() {
            assert!(
                !self.entries[index]
                    .flags()
                    .contains(TableEntryFlags::HUGE_PAGE),
                "mapping code does not support huge pages"
            );
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, TableEntryFlags::PRESENT | TableEntryFlags::WRITABLE);
            self.next_table_mut(index).unwrap().zero()
        }
        self.next_table_mut(index).unwrap()
    }

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(TableEntryFlags::PRESENT)
            && !entry_flags.contains(TableEntryFlags::HUGE_PAGE)
        {
            let table_address = self as *const _ as usize;
            Some((table_address << 10) | (index << 12))
        } else {
            None
        }
    }
}
