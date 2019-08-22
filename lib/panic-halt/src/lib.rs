// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

//! Set the panicking behavior to halt
//!
//! This crate contains an implementation of `panic_fmt` that simply
//! halt in an infinite loop

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
