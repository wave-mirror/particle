// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use crate::arch::PAGE_SIZE_SHIFT;
use crate::allocator;

use cortex_m_semihosting::{hprintln};

pub unsafe fn novm_init() {
    extern "C" {
        static __end: usize;
        static __end_of_ram: usize;
    }

    let mem_start = &__end as *const usize as usize;
    let mem_end = &__end_of_ram as *const usize as usize;
    let mem_size = mem_end - mem_start;

    hprintln!("start={:X}, end={:X}, size={}", mem_start, mem_end, mem_size);

    allocator::heap_init(mem_start, mem_size);
}
