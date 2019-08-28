// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use spin::{Once, RwLock};

/// Thread struct
mod thread;

/// Thread struct list
mod list;

pub use self::thread::Thread;
pub use self::list::ThreadList;

static IDLE_THREAD: Once<Thread> = Once::new();

/// Threads list
static THREAD_LIST: Once<RwLock<ThreadList>> = Once::new();

/// Initialize threading system
///
/// This function is called once, from kmain()
pub fn thread_early_init() {
    // create a thread to cover the curring running state
}
