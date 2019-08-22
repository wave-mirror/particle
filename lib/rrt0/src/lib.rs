// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

//! Initialization code ("crt0") written in Rust
//! This is for bare metal systems where there is no ELF loader or OS to
//! take care of initializing RAM for the program.
//!
//! # Initializing RAM

#![deny(warnings)]
#![no_std]

use core::{mem, ptr};

/// Zeroes the `.bss` section
///
/// # Arguments
///
/// - `sbss`. Pointer to the start of the `.bss` section.
/// - `ebss`. Pointer to the end of the `.bss` section.
pub unsafe fn zero_bss<T>(mut sbss: *mut T, ebss: *mut T)
where
    T: Copy,
{
    while sbss < ebss {
        ptr::write_volatile(sbss, mem::zeroed());
        sbss = sbss.offset(1);
    }
}

/// Initialize the `.data` section
///
/// # Arguments
///
/// - `sdata`. Pointer to the start of the `.data` section.
/// - `edata`. Pointer to the end of the `.data` section.
pub unsafe fn init_data<T>(
    mut sdata: *mut T,
    edata: *mut T,
    mut sidata: *const T,
) where
    T: Copy,
{
    while sdata < edata {
        ptr::write(sdata, ptr::read(sidata));
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }
}
