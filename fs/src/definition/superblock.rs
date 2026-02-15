/// The definition of the super block.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuperBlock {
    /// The magic number to identify the file system.
    pub magic: u32,

    /// The size of each block in bytes.
    pub block_size: u32,

    /// The block number where the inode start.
    pub inode_start_block: u32,

    /// The block number where the data starts.
    pub data_start_block: u32,

    /// The total block number in the partition.
    pub total_block_num: u32,
}

impl SuperBlock {
    /// Get the super block as a byte slice.
    ///
    /// # Returns
    ///
    /// * `&[u8]` - The super block as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    /// Get the super block as a mutable byte slice.
    ///
    /// # Returns
    ///
    /// * `&mut [u8]` - The super block as a mutable byte slice.
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self as *mut Self as *mut u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    /// Create an inode object by a slice.
    ///
    /// # Parameters
    ///
    /// * `buf` - The slice of bytes.
    ///
    /// # Returns
    ///
    /// * `Self` - The inode object.
    pub fn from_bytes(buf: &[u8]) -> Option<&Self> {
        if buf.len() < core::mem::size_of::<Self>() {
            return None;
        }
        let ptr = buf.as_ptr() as *const Self;
        let inode: &Self = unsafe { &*ptr };
        Some(inode)
    }

    /// Create this inode object by a mutable slice.
    ///
    /// # Parameters
    ///
    /// * `buf` - The slice of bytes.
    ///
    /// # Returns
    ///
    /// * `Self` - The inode object.
    pub fn from_bytes_mut(buf: &mut [u8]) -> Option<&mut Self> {
        if buf.len() < core::mem::size_of::<Self>() {
            return None;
        }
        let inode = unsafe { &mut *(buf[0] as *mut u8 as *mut Self) };
        Some(inode)
    }
}

impl SuperBlock {
    /// Init a superblock object.
    ///
    /// # Parameters
    ///
    /// * `fs_type` - The file system type.
    /// * `partition_size` - The partition size in bytes.
    ///
    /// # Returns
    ///
    /// * `Self` - The superblock object.
    pub fn new(partition_size: u64) -> Self {
        Self {
            magic: 0x504B4653,
            block_size: 1024,
            inode_start_block: 257,
            data_start_block: 65536+256,
            total_block_num: (partition_size / 1024) as u32,
        }
    }
}
