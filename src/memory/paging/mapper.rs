use core::ptr::Unique;

use crate::memory::{paging::ENTRY_COUNT, Frame, FrameAllocator, PAGE_SIZE};

use super::{
    table::{Level2, Table, P2},
    table_entry::TableEntryFlags,
    Page, PhysicalAddress, VirtualAddress,
};

pub struct Mapper {
    p2: Unique<Table<Level2>>,
}

impl Mapper {
    pub unsafe fn new() -> Self {
        Self {
            p2: Unique::new_unchecked(P2),
        }
    }

    pub fn p2(&self) -> &Table<Level2> {
        unsafe { self.p2.as_ref() }
    }

    pub fn p2_mut(&mut self) -> &mut Table<Level2> {
        unsafe { self.p2.as_mut() }
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    // very sus
    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        let p1 = self.p2().next_table(page.p2_index());
        p1.and_then(|p1| p1[page.p1_index()].pointed_frame())
            .or_else(|| {
                let p2_entry = &self.p2()[page.p2_index()];
                if let Some(start_frame) = p2_entry.pointed_frame() {
                    if p2_entry.flags().contains(TableEntryFlags::HUGE_PAGE) {
                        assert!(start_frame.number % ENTRY_COUNT == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p1_index(),
                        });
                    }
                }
                None
            })
    }

    pub fn map_to<A: FrameAllocator>(
        &mut self,
        page: Page,
        frame: Frame,
        flags: TableEntryFlags,
        allocator: &mut A,
    ) {
        let p1 = self.p2_mut().next_table_create(page.p2_index(), allocator);
        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | TableEntryFlags::PRESENT);
    }

    pub fn map<A: FrameAllocator>(&mut self, page: Page, flags: TableEntryFlags, allocator: &mut A) {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    pub fn identity_map<A: FrameAllocator>(
        &mut self,
        frame: Frame,
        flags: TableEntryFlags,
        allocator: &mut A,
    ) {
        self.map_to(
            Page::containing_address(frame.start_address()),
            frame,
            flags,
            allocator,
        )
    }

    // TODO: properly
    pub fn unmap<A: FrameAllocator>(&mut self, _page: Page, _: &mut A) {
        // assert!(self.translate(page.start_address()).is_some());
        // let p1 = self
        //     .p4_mut()
        //     .next_table_mut(page.p4_index())
        //     .and_then(|p3| p3.next_table_mut(page.p3_index()))
        //     .and_then(|p2| p2.next_table_mut(page.p2_index()))
        //     .expect("mapping code does not support huge pages");
        // // let frame = p1[page.p1_index()].pointed_frame().unwrap();
        // p1[page.p1_index()].set_unused();
        // tlb_flush(page.start_address() as u64);
        // TODO: free p1, p2, p3 tables if empty
        // allocator.deallocate_frame(frame)
    }
}
