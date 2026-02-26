/// The definition of the super block.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuperBlock {
    /// The magic number to identify the file system.
    pub magic: u32,

    /// The size of each block in bytes.
    pub block_size: u32,

    /// The block number where the block bitmap starts.
    pub bitmap_start_block: u32,

    /// The block number where the data starts.
    pub data_start_block: u32,

    /// The total block number in the partition.
    pub total_block: u32,
}

impl crate::GenericFsData for SuperBlock {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self as *mut Self as *mut u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    fn from_bytes(buf: &[u8]) -> Option<&Self> {
        if buf.len() < core::mem::size_of::<Self>() {
            return None;
        }
        let ptr = buf.as_ptr() as *const Self;
        let inode: &Self = unsafe { &*ptr };
        Some(inode)
    }

    fn from_mut_bytes(buf: &mut [u8]) -> Option<&mut Self> {
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
        let total_block_num: usize = partition_size as usize / 1024;
        let data_start_block = total_block_num as u32 / 1024 + 65536;
        let bitmap_size = total_block_num as usize / 1024; // 1 bit per block, so 1 byte can represent 8 blocks, so 1024 bytes can represent 8192 blocks
        Self {
            magic: 0x504B4653,
            block_size: 1024,
            bitmap_start_block: (total_block_num - bitmap_size) as u32,
            data_start_block,
            total_block: total_block_num as u32,
        }
    }
}
