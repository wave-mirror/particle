// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

#[cfg(cortex_m)]
pub mod cortex_m;

#[cfg(cortex_m)]
pub use self::cortex_m::*;
