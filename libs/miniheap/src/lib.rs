// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#![feature(const_fn)]
#![feature(allocator_api)]
#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::alloc::{Alloc, AllocErr, Layout};
use core::mem;
use core::ptr::NonNull;
use list::{Chunk, FreeChunkList};

mod list;

#[cfg(test)]
mod test;

pub struct Heap {
    base: usize,
    size: usize,
    free_list: FreeChunkList,
}

impl Heap {
    pub const fn empty() -> Heap {
        Heap {
            base: 0,
            size: 0,
            free_list: FreeChunkList::empty(),
        }
    }

    /// Initializes an empty heap
    ///
    /// # Unsafety
    ///
    /// This function must be called at most once and must only be used
    /// on an empty heap
    pub unsafe fn init(&mut self, heap_base: usize, heap_size: usize) {
        self.base = heap_base;
        self.size = heap_size;
        self.free_list = FreeChunkList::new(heap_base, heap_size);
    }

    /// Creates a new heap with the given `heap_base` and `heap_size`.
    /// The heap base address must be valid and the memory int the
    /// `[heap_base, heap_base + heap_size]` range must not be used for
    /// anything else. This function is unsafe because it can cause
    /// undefined behavior if the given address is invalid.
    pub unsafe fn new(heap_base: usize, heap_size: usize) -> Heap {
        Heap {
            base: heap_base,
            size: heap_size,
            free_list: FreeChunkList::new(heap_base, heap_size),
        }
    }

    /// Allocates a chunk of the given size with the given alignment.
    /// Returns a pointer to the beginning of that chunk if it was
    /// successful. Else it returns `None`. This function scans the
    /// list of free memory blocks and uses the first block that is
    /// big enough. The runtime is in O(N) where N is the number of
    /// free blocks, but it should be reasonably fast for small
    /// allocations
    pub fn allocate_first_fit(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let mut size = layout.size();
        if size < FreeChunkList::min_size() {
            size = FreeChunkList::min_size();
        }
        let size = align_up(size, mem::align_of::<Chunk>());
        let layout = Layout::from_size_align(size, layout.align()).unwrap();

        self.free_list.allocate_first_fit(layout)
    }

    /// Frees the given allocation. `ptr` must be a pointer returned
    /// by a call to the `allocate_first_fit` function with identical
    /// size and alignment. Undefined behavior may occur for invalid
    /// arguments, thus this function is unsafe.
    ///
    /// This function walks the list of free memory blocks and inserts
    /// the freed block at the correct place. if the freed block is
    /// adjacent to another free block, the blocks are merged again.
    /// This operation is in `O(N)` since the list needs to be sorted
    /// by address.
    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let mut size = layout.size();
        if size < FreeChunkList::min_size() {
            size = FreeChunkList::min_size();
        }
        let size = align_up(size, mem::align_of::<Chunk>());
        let layout = Layout::from_size_align(size, layout.align()).unwrap();

        self.free_list.deallocate(ptr, layout);
    }
}

unsafe impl Alloc for Heap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        self.allocate_first_fit(layout)
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        self.deallocate(ptr, layout)
    }
}

/// align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}
