// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use core::mem::size_of;

/// A sorted list of Node. It uses itself to store its chunks
pub struct FreeChunkList {
    head: Chunk,  // dummy
}

impl FreeChunkList {
    /// Creates an empty 'FreeChunkList'
    pub fn empty() -> FreeChunkList {
        FreeChunkList {
            head: Chunk {
                size: 0,
                next: None,
            },
        }
    }

    /// Creates a `FreeList` that contains the given free chunk. This function
    /// is unsafe because it creates a node at the given `node_addr`. This can
    /// cause undefined behavior if this address is invalid or if memory from
    /// the`[node_addr, node_addr+size` range is used somewhere else.
    pub unsafe fn new(chunk_addr: usize, chunk_size: usize) -> FreeChunkList {
        assert!(size_of::<Chunk>() == Self::min_size());

        let ptr = chunk_addr as *mut Chunk;
        ptr.write(Chunk {
            size: chunk_size,
            next: None,
        });

        FreeChunkList {
            head: Chunk {
                size: 0,
                next: Some(&mut *ptr),
            }
        }
    }

    /// Returns the minimal allocation size. Smaller allocations or
    /// deallocations are not allowd.
    pub fn min_size() -> usize {
        size_of::<usize>() * 2
    }
}

pub struct Chunk {
    size: usize,
    next: Option<&'static mut Chunk>
}

impl Chunk {
    /// Returns the basic information about the Chunk
    fn info(&self) -> ChunkInfo {
        ChunkInfo {
            addr: self as *const _ as usize,
            size: self.size,
        }
    }
}

/// The basic informatioin about a chunk
#[derive(Debug, Clone, Copy)]
pub struct ChunkInfo {
    addr: usize,
    size: usize,
}
