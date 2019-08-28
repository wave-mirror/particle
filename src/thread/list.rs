// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#![deny(warnings)]

use alloc::sync::Arc;
use alloc::collections::VecDeque;
use spin::RwLock;

use super::thread::Thread;

/// Thread list type
pub type ThreadList = VecDeque<(usize, Arc<RwLock<Thread>>)>;
