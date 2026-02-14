//! The bitmap which describes is the block bitmap and inode bitmap used.

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

impl<const N: usize> Bitmap for &mut [u8; N] {
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
