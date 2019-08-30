// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT
use super::*;
use std::prelude::v1::*;

fn new_heap() -> Heap {
    const HEAP_SIZE: usize = 1000;
    let heap_addr = Box::into_raw(Box::new([0u8; HEAP_SIZE]));
    let heap = unsafe { Heap::new(heap_addr as usize, HEAP_SIZE)};
    assert_eq!(heap.base, heap_addr as usize);
    assert_eq!(heap.size, HEAP_SIZE);
    heap
}

#[test]
fn empty() {
    let mut heap = Heap::empty();
    assert_eq!(heap.base, 0);
    assert_eq!(heap.size, 0);
}

#[test]
fn new() {
    let mut heap = new_heap();
}
