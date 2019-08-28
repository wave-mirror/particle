// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#![no_main]
#![no_std]

extern crate alloc;
extern crate spin;

extern crate panic_halt;

pub mod arch;
pub mod allocator;
pub mod thread;

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;

//use cortex_m_semihosting::{hprintln};

pub fn kmain() -> ! {
    thread::thread_early_init();
    arch::arch_early_init();
    // hprintln!("Welcome to Particle!").unwrap();

    loop {}
}
