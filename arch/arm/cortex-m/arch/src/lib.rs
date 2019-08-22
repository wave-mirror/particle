// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

//! Startup code and minimal runtime for Cortex-M microcontrollers
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

//! - `reset`. This is the reset handler. The microcontroller will executed this
//! function upon booting. This function will call the user program entry point
//! (cf. [`#[entry]`]) using the `main` symbol so you may also find that symbol
//! in your program; if you do, `main` will contain your application code. Some
//! other times `main` gets inlined into `reset` so you won't find it.

// Entry point
#[doc(hidden)]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        // These symbols code from `kernel.ld`
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;

        static __sidata: u32;
    }

    extern "Rust" {
        fn main() -> !;

        fn __pre_init();
    }

    __pre_init();

    // Initialize RAM
    rrt0::zero_bss(&mut __sbss, &mut __ebss);
    rrt0::init_data(&mut __sdata, &mut __edata, &__sidata);

    match () {
        #[cfg(not(has_fpu))]
        () => main(),
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn default_pre_init() {}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS_VECTOR: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector { handler: nmi },
    // Exception 3: Hard Fault Interrupt.
    Vector { handler: hard_fault },
    // Exception 4: Memory Management Interrupt
    #[cfg(not(armv6m))]
    Vector { handler: mem_manage },
    #[cfg(armv6m)]
    Vector { resrved: 0 },
    // Exception 5: Bus Fault Interrupt
    #[cfg(not(armv6m))]
    Vector { handler: bus_fault },
    #[cfg(armv6m)]
    Vector { resrved: 0 },
    // Exception 6: Usage Fault Interrupt
    #[cfg(not(armv6m))]
    Vector { handler: usage_fault },
    #[cfg(armv6m)]
    Vector { resrved: 0 },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    #[cfg(armv8m)]
    Vector { handler: secure_fault },
    #[cfg(not(armv8m))]
    Vector { reserved: 0 },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: svc_call },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: debug_monitor,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: pendsv },
    // Exception 15: System Tick Interrupt.
    Vector { handler: systick },
];

extern "C" {
    fn nmi();

    fn hard_fault();

    #[cfg(not(armv6m))]
    fn mem_manage();

    #[cfg(not(armv6m))]
    fn bus_fault();

    #[cfg(not(armv6m))]
    fn usage_fault();

    #[cfg(armv8m)]
    fn secure_fault();

    fn svc_call();

    #[cfg(not(armv6m))]
    fn debug_monitor();

    fn pendsv();

    fn systick();
}
