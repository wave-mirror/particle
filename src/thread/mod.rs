// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

pub mod thread;

/// Initialize threading system
///
/// This function is called once, from kmain()
pub fn thread_early_init() {
    // create a thread to cover the curring running state
}
