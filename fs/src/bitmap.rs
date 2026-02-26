//! The bitmap which describes is the block bitmap and inode bitmap used.

use crate::{GenericFsData, Vec};

pub trait Bitmap {
    /// Set up a bit's status
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the block or inode.
    /// * `used` - Whether the block or inode is used.
    fn set(&mut self, index: usize, used: bool);

    /// Check if the block or inode is used.
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the block or inode.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the block or inode is used.
    fn is_used(&self, index: usize) -> bool;

    /// Allocate a free bit.
    ///
    /// # Parameters
    ///
    /// * `max` - The maximum index to search.
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - The index of the free bit, or None if no free bit is found.
    fn alloc(&mut self, max: usize) -> Option<usize>;

    /// Free the bit.
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the block or inode.
    fn free(&mut self, index: usize);

    /// Clear all bits.
    fn clear(&mut self);
}

// Implement this trait for all &mut [u8; N]
impl Bitmap for &mut [u8] {
    fn set(&mut self, index: usize, used: bool) {
        self[index] = if used { 1 } else { 0 };
    }

    fn is_used(&self, index: usize) -> bool {
        self[index] != 0
    }

    fn alloc(&mut self, max: usize) -> Option<usize> {
        for i in 0..max {
            if !self.is_used(i) {
                self[i] = 1;
                return Some(i);
            }
        }
        None
    }

    fn free(&mut self, index: usize) {
        self[index] = 0;
    }

    fn clear(&mut self) {
        for i in 0..self.len() {
            self[i] = 0;
        }
    }
}

impl Bitmap for Vec<u8> {
    fn set(&mut self, index: usize, used: bool) {
        self[index] = if used { 1 } else { 0 };
    }

    fn is_used(&self, index: usize) -> bool {
        self[index] != 0
    }

    fn alloc(&mut self, max: usize) -> Option<usize> {
        for i in 0..max {
            if !self.is_used(i) {
                self[i] = 1;
                return Some(i);
            }
        }
        None
    }

    fn free(&mut self, index: usize) {
        self[index] = 0;
    }

    fn clear(&mut self) {
        for i in 0..self.len() {
            self[i] = 0;
        }
    }
}

/* ==========<Block Bitmap Definition>========== */

/// The block bitmap.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockBitmap<'a>(&'a mut [u8]);

// Let the [`BlockBitmap`] implement the [`Bitmap`] trait, so that it can use the bitmap methods.
impl Bitmap for BlockBitmap<'_> {
    fn set(&mut self, index: usize, used: bool) {
        self.0.set(index, used);
    }

    fn is_used(&self, index: usize) -> bool {
        self.0.is_used(index)
    }

    fn alloc(&mut self, max: usize) -> Option<usize> {
        self.0.alloc(max)
    }

    fn free(&mut self, index: usize) {
        self.0.free(index);
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

// This struct will written into the disk as well, so it's a generic fs type.
// That's why it implements the GenericFsData trait.
impl GenericFsData for BlockBitmap<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.0
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self.0
    }

    fn from_bytes(bytes: &[u8]) -> Option<&Self>
    where
        Self: Sized,
    {
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }

    fn from_mut_bytes(bytes: &mut [u8]) -> Option<&mut Self>
    where
        Self: Sized,
    {
        Some(unsafe { &mut *(bytes.as_mut_ptr() as *mut Self) })
    }
}

impl<'a> BlockBitmap<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self(bytes)
    }
}