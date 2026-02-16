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

// The block bitmap.
pub struct BlockBitmap<const N: usize>(&'static mut [u8; N]);

impl<const N: usize> Bitmap for BlockBitmap<N> {
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

impl<const N: usize> BlockBitmap<N> {
    pub fn from_slice(slice: &'static mut [u8]) -> Result<Self, &'static str> {
        unsafe {
            if slice.len() != N {
                return Err("The slice length is not equal to the block bitmap size.");
            }
            let array = &mut *slice.as_mut_ptr().cast::<[u8; N]>();
            Ok(Self(array))
        }
    }
}

impl<const N: usize> crate::GenericFsData for BlockBitmap<N> {
    fn as_bytes(&self) -> &[u8] {
        self.0
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self.0
    }

    fn from_bytes(bytes: &[u8]) -> Option<&Self>
        where
            Self: Sized {
        if bytes.len() != N {
            return None;
        }
        let bitmap = unsafe { &*(bytes[0] as *const u8 as *const Self) };
        Some(bitmap)
    }

    fn from_mut_bytes(bytes: &mut [u8]) -> Option<&mut Self>
        where
            Self: Sized {
        if bytes.len() != N {
            return None;
        }
        let bitmap = unsafe { &mut *(bytes[0] as *mut u8 as *mut Self) };
        Some(bitmap)
    }
}
