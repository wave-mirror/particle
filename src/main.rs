// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#![no_main]
#![no_std]

extern crate panic_halt;
use cortex_m_rt_macros::entry;

#[entry]
fn main() -> ! {
    loop {}
}
