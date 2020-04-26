use super::align_up;
use super::Locked;
use alloc::alloc::{GlobalAlloc, Layout};
use alloc::vec::Vec;
use core::mem;
use core::ptr;
use ggos::util::bit_array::BitArray;
use x86_64::structures::paging::{Page, PageSize, Size4KiB};
use x86_64::VirtAddr;

const DMA_VIRT_START: usize = 0x_8000_0000_0000;
const DMA_SIZE: u16 = 512;

pub struct dma_allocator<S: PageSize = Size4KiB> {
    pub start: VirtAddr,
    pub phy_pages: [UnusedPhysFrame; DMA_SIZE],
    pub avails: u16,
    pub useds: u16,
    pub returns: u16,
}

impl dma_allocator {
    fn new<S: PageSize = Size4KiB>(s: S) -> Self {
        Self {
            start: VirtAddr::new(DMA_VIRT_START as u64),
            avail_pages: DMA_SIZE,
        }
    }

    unsafe fn alloc_pages(&self, pages: u16) -> *mut u8 {
        let mut dma = self.lock();

        let alloc_start = self.start as usize;

        if pages > self.avail_pages {
            ptr::null_mut()
        }

        self.start = alloc_start + S::SIZE * pages;
        self.avail_pages -= pages;

        VirtAddr::new(alloc_start);
    }

    unsafe fn free(&self, ptr: *mut u8, size: usize) {}
}
