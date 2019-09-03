// Copyright 2019 The Particle Authors
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT

use alloc::alloc::{AllocErr, Layout};
use core::mem::size_of;
use core::ptr::NonNull;

use super::align_up;

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

        allocate_first_fit(&mut self.head, layout).map(|allocation| {
            if let Some(padding) = allocation.front_padding {
                deallocate(&mut self.head, padding.addr, padding.size);
            }
            if let Some(padding) = allocation.back_padding {
                deallocate(&mut self.head, padding.addr, padding.size);
            }
            NonNull::new(allocation.info.addr as *mut u8).unwrap()
        })
    }

    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        deallocate(&mut self.head, ptr.as_ptr() as usize, layout.size())
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
    front_padding: Option<ChunkInfo>,
    back_padding: Option<ChunkInfo>,
}

/// Splits the given chunk into `(front_padding, chunk, back_padding)` if
/// it's big enough to allocate `layout.size()` bytes with the `layout.align()`.
/// Else `None` is returned. Front padding occurs if the required alignment is
/// higher than the chunk's alignment. Back padding occurs if the required size
/// is smaller than the size of the aligned chunk. All padding must be at least
/// `FreeChunkList::min_size()` big or the chunk is unusable.
fn split_chunk(info: ChunkInfo, layout: Layout) -> Option<Allocation> {
    let size = layout.size();
    let align = layout.align();

    let (aligned_addr, front_padding) = if info.addr == align_up(info.addr, align) {
        // chunk has already the required alignment
        (info.addr, None)
    } else {
        // the required alignment causes some padding before the allocation
        let aligned_addr = align_up(info.addr + FreeChunkList::min_size(), align);
        (
            aligned_addr,
            Some(ChunkInfo {
                addr: info.addr,
                size: aligned_addr - info.addr,
            }),
        )
    };

    let aligned_info = {
        if aligned_addr + size > info.addr + info.size {
            // chunk is too small
            return None;
        }
        ChunkInfo {
            addr: aligned_addr,
            size: info.size - (aligned_addr - info.addr),
        }
    };

    let back_padding = if aligned_info.size == size {
        // the aligned chunk has excatly the size the size that's needed,
        // no padding accrues
        None
    } else if aligned_info.size - size < FreeChunkList::min_size() {
        // We can't use this info since its remains would form a new, too
        // small chunk
        return None;
    } else {
        // the chunk is bigger than necessary, so there is some padding
        // behind the allocation
        Some(ChunkInfo {
            addr: aligned_info.addr + size,
            size: aligned_info.size - size,
        })
    };

    Some(Allocation {
        info: ChunkInfo {
            addr: aligned_info.addr,
            size: size,
        },
        front_padding: front_padding,
        back_padding: back_padding,
    })
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
        match allocation {
            Some(allocation) => {
                // chunk is big enough, so remove it from the list by updating
                // the previous pointer
                previous.next = previous.next.as_mut().unwrap().next.take();

                return Ok(allocation);
            }
            None if previous.next.is_some() => {
                // try next chunk
                previous = move_helper(previous).next.as_mut().unwrap();
            }
            None => {
                // this was the last chunk, so no chunk is big enough
                // allocation not possible
                return Err(AllocErr);
            }
        }
    }
}

/// Frees the allocation given by `(addr, size)`. It starts at the given
/// chunk and walks the list to find the correct place (the list is sorted
/// by address).
fn deallocate(mut chunk: &mut Chunk, addr: usize, mut size: usize) {
    loop {
        assert!(size >= FreeChunkList::min_size());

        let chunk_addr = if chunk.size == 0 {
            // It's the dummy chunk, which is the head of the FreeChunkList.
            // It's somewhere on the stack, so it's address is not the address
            // of the chunk. We set the addr to 0 as it's always the first chunk.
            0
        } else {
            // tt's a real chunk in memory and its address is the address of the chunk
            chunk as *mut _ as usize
        };

        // Each freed block must be handled by the previous chunk in memory.
        // Thus the freed address must be always behind the current chunk.
        assert!(
            chunk_addr + chunk.size <= addr,
            "invalid deallocation (probably a double free)"
        );

        // get information about the next block
        let next_chunk_info = chunk.next.as_ref().map(|next| next.info());

        match next_chunk_info {
            Some(next) if chunk_addr + chunk.size == addr && addr + size == next.addr => {
                // block fills the gap between this chunk and the next chunk
                // before:  ___XXX____YYYYY____    where X is this chunk and Y the next chunk
                // after:   ___XXXFFFFYYYYY____    where F is the freed block

                // merge the F and Y blocks to this X block
                chunk.size += size + next.size;
                // remove the Y block
                chunk.next = chunk.next.as_mut().unwrap().next.take();
            }
            _ if chunk_addr + chunk.size == addr => {
                // block is right behind this chunk but there is used memory after it
                // before:  ___XXX______YYYYY____    where X is this chunk and Y the next chunk
                // after:   ___XXXFFFF__YYYYY____    where F is the freed block

                // or: block is right behind this chunk and this is the last chunk
                // before:  ___XXX_______________   where X is this chunk
                // after:   ___XXXFFFF___________    where F is the freed block

                chunk.size += size; // merge the F block to this X block
            }
            Some(next) if addr + size == next.addr => {
                // block is right before the next chunk but there is used memory before it
                // before:  ___XXX______YYYYY____    where X is this chunk and Y the next chunk
                // after:   ___XXX__FFFFYYYYY____    where F is the freed block

                chunk.next = chunk.next.as_mut().unwrap().next.take(); // remove the Y block
                size += next.size; // free the merged F/Y block in next iteration
                continue;
            }
            Some(next) if next.addr <= addr => {
                // block is behind the next chunk, so we delegate it to the next chunk
                // before:  ___XXX__YYYYY________    where X is this chunk and Y the next chunk
                // after:   ___XXX__YYYYY__FFFF__    where F is the freed block

                chunk = move_helper(chunk).next.as_mut().unwrap(); // start next iteration at next chunk
                continue;
            }
            _ => {
                // block is between this and the next hole
                // before:  ___XXX________YYYYY_    where X is this hole and Y the next hole
                // after:   ___XXX__FFFF__YYYYY_    where F is the freed block

                // or: this is the last hole
                // before:  ___XXX_________    where X is this hole
                // after:   ___XXX__FFFF___    where F is the freed block

                let new_chunk = Chunk {
                    size: size,
                    next: chunk.next.take(), // the reference to the Y block (if it exists)
                };
                // write the new hole to the freed memory
                let ptr = addr as *mut Chunk;
                unsafe { ptr.write(new_chunk) };
                // add the F block as the next block of the X block
                chunk.next = Some(unsafe { &mut *ptr });
            }
        }
        break;
    }
}

fn move_helper<T>(x: T) -> T {
    x
}
