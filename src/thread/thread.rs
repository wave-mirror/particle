// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#![deny(warnings)]

//use alloc::sync::Arc;
//use alloc::boxed::Box;
//use alloc::vec::Vec;
//use spin::Mutex;

#[derive(Debug)]
pub enum ThreadState {
    Suspended = 0,
    Ready,
    Running,
    Blocked,
    Sleeping,
    Death,
}

#[derive(Debug)]
pub struct Thread {
    /// The magic of this thread
    magic: i32,
    /// The priority of this thread
    priority: i32,
    /// The status of this thread
    state: ThreadState,
    // The name of this thread
    //name: Arc<Mutex<Box<[u8]>>>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            magic: 0x70617274,  // 'part'
            priority: 0,
            state: ThreadState::Suspended,
            //name: Arc::new(Mutex::new(Vec::new().into_boxed_slice())),
        }
    }
}
