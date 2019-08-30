// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use crate::arch::PAGE_SIZE_SHIFT;

use cortex_m_semihosting::{hprintln};

extern "C" {
    static __end: usize;
    static __end_of_ram: usize;
}

pub unsafe fn novm_init() {
    let mem_start = &__end;
    let mem_end = &__end_of_ram;
    let mem_size = mem_end - mem_start;
    //let xs:[u8; default_map_size] = [0; default_map_size];
}
