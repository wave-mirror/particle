// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use alloc::alloc::{AllocErr, Layout};
use core::mem::size_of;
use core::ptr::NonNull;

/// A sorted list of Node. It uses itself to store its chunks
pub struct FreeChunkList {
    head: Chunk,  // dummy
}

impl FreeChunkList {
    /// Creates an empty 'FreeChunkList'
    pub const fn empty() -> FreeChunkList {
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

    /// Searches the list of a big enough hole. A chunk is big enough if it
    /// can hold an allocation of `layout.size()` bytes with the given
    /// `layout.align()`. If such a chunk is found in the list, a block of
    /// the required size is allocated from it. Then the start address of
    /// that block is returned.
    /// This function uses the "fist fit" strategy, so it uses the head
    /// chunk that is big enough. The runtime is O(N) but it should be
    /// reasonably fast for small allocations.
    pub fn allocate_first_fit(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        assert!(layout.size() >= Self::min_size());

        allocate_first_fit(&mut self.head, layout)
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

/// The result returned by `split_chunk` and `allocate_first_fit`.
/// Contains the address and size of the allocation (in the `info` field),
/// and the front and back padding.
struct Allocation {
    info: ChunkInfo,
    frond_padding: Option<ChunkInfo>,
    back_padding: Option<ChunkInfo>,
}

/// Splits the given chunk into `(front_padding, chunk, back_padding)` if
/// it's big enough to allocate `required_layout.size()` bytes with the
/// `required_layout.align()`. Else `None` is returned. Front padding
/// occurs if the required alignment is higher than the chunk's alignment.
/// Back padding occurs if the required size is smaller than the size of
/// the aligned chunk. All padding must be at least `FreeChunkList::min_size()`
/// big or the chunk is unusable.
fn split_chunk(info: ChunkInfo, required_layout: Layout) -> Option<Allocation> {
    let required_size = required_layout.size();
    let required_align = required_layout.align();
}

/// Searches the list starting at the next chunk of `previous` for a big
/// enough chunk. A chunk is big enough if it can hold an allocation of
/// `layout.size()` bytes with the given `layout.align()`. When a chunk
/// is used for an allocation, there may be some needed padding before
/// and/or after the allocation. This padding is returned as part of the
/// `Allocation`. The caller must take care of freeing it again.
/// This function uses the "first fit" strategy, so it breaks as soon as
/// a big enough chunk is found (and returns it).
fn allocate_first_fit(mut previous: &mut Chunk, layout: Layout) -> Result<Allocation, AllocErr> {
    loop {
        let allocation: Option<Allocation> = previous
            .next
            .as_mut()
            .and_then(|current| split_chunk(current.info(), layout.clone()));
    }
}
