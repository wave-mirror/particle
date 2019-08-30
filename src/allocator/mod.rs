// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

mod miniheap;

pub use self::miniheap::Allocator;

pub unsafe fn heap_init(heap_base: usize, heap_size: usize) {
    // Initialize global heap
    Allocator::init(heap_base, heap_size);
}
