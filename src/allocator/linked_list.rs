// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use linked_list_allocator::Heap;
use spin::Mutex;

pub struct Allocator;

static HEAP: Mutex<Option<Heap>> = Mutex::new(None);

impl Allocator {
    pub unsafe fn init(offset: usize, size: usize) {
    }
}
