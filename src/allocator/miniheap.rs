// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use core::alloc::{AllocErr, GlobalAlloc, Layout};
use miniheap::Heap;
use spin::Mutex;

pub struct Allocator;

static HEAP: Mutex<Option<Heap>> = Mutex::new(None);

impl Allocator {
    pub unsafe fn init(heap_base: usize, heap_size: usize) {
        *HEAP.lock() = Some(Heap::new(heap_base, heap_size));
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        loop {
            let res = if let Some(ref mut heap) = *HEAP.lock() {
                heap.allocate_first_fit(layout)
            } else {
                panic!("minheap_allocator: miniheap not initialized");
            };
            None
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}
