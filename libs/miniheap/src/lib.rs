// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod test;

mod list;

use list::FreeChunkList;

pub struct Heap {
    base: usize,
    size: usize,
    free_list: FreeChunkList,
}

impl Heap {
    pub fn empty() -> Heap {
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
}
