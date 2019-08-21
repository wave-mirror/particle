// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    has_fpu(&target);

    // Put the linker script somewhere the linker can find it
    let kernel_ld = include_bytes!("kernel.ld.in");
    let mut f = if env::var_os("CARGO_FEATURE_DEVICE").is_some() {
        let mut f = File::create(out_dir.join("kernel.ld")).unwrap();
        f.write_all(kernel_ld).unwrap();
        // *IMPORTANT*: The weak aliases (i.e. `PROVIDED`) must come *after*
        // `EXTERN(__INTERRUPTS)`. Otherwise the linker will ignore user
        // defined interrupts and always populate the table with the weak aliases.
        writeln!(
            f,
            r#"
/* Provides weak aliases (cf. PROVIDED) for device specific interrupt handlers */
/* This will usually be provided by a device crate generated using svd2rust (see `device.ld`)*/
INCLUDE device.ld"#).unwrap();
        f
    } else {
        let mut f = File::create(out_dir.join("kernel.ld")).unwrap();
        f.write_all(kernel_ld).unwrap();
        f
    };

    println!("cargo:rustc-link-search={}", out_dir.display());
}

fn has_fpu(target: &str) {
    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
