#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
pub mod bitmap;
pub mod definition;

pub use bitmap::Bitmap;

use crate::definition::Inode;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use {
    std::fs::{File, OpenOptions},
    std::io::{Read, Seek, SeekFrom, Write},
};

pub const BLOCK_SIZE: usize = 1024;

/// The block device driver.
pub trait BlockDevice {
    /// Read a block from the block device.
    ///
    /// # Parameters
    ///
    /// * `block_num` - The block number to read.
    /// * `buf` - The buffer to store the data.
    fn read_block(
        &mut self,
        block_num: u32,
        offset: u32,
        buf: &mut [u8],
    ) -> Result<(), &'static str>;

    /// Write a block to the block device.
    ///
    /// # Parameters
    ///
    /// * `block_num` - The block number to write.
    /// * `buf` - The data to write.
    fn write_block(&mut self, block_num: u32, offset: u32, buf: &[u8]) -> Result<(), &'static str>;
}

#[cfg(feature = "std")]
// Implement the block device for the file.
pub struct FileBlockDevice(File);

#[cfg(feature = "std")]
impl BlockDevice for FileBlockDevice {
    fn read_block(
        &mut self,
        block_num: u32,
        offset: u32,
        buf: &mut [u8],
    ) -> Result<(), &'static str> {
        // Read from file
        self.0
            .seek(SeekFrom::Start(
                (block_num as u64 * BLOCK_SIZE as u64) + offset as u64,
            ))
            .map_err(|_| "Failed to seek to block")?;
        self.0.read_exact(buf).map_err(|_| "Failed to read block")
    }

    fn write_block(&mut self, block_num: u32, offset: u32, buf: &[u8]) -> Result<(), &'static str> {
        // Write to file
        self.0
            .seek(SeekFrom::Start(
                (block_num as u64 * BLOCK_SIZE as u64) + offset as u64,
            ))
            .map_err(|_| "Failed to seek to block")?;
        self.0.write_all(buf).map_err(|_| "Failed to write block")?;
        self.0.sync_all().map_err(|_| "Failed to sync block")
    }
}

/// Initialize the block device driver for the file system.
///
/// # Parameters
///
/// * `file_path` - The path of the file to be used as the block device.
///
/// # Returns
///
/// * `Result<FileBlockDevice, String>` - The block device driver.
#[cfg(feature = "std")]
pub fn init_block_device(file_path: &str) -> Result<FileBlockDevice, String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    // Return the block device driver.
    Ok(FileBlockDevice(file))
}

/// The basic structure of the whole file system.
#[repr(C)]
pub struct FileSystem<B: BlockDevice> {
    /// The block device driver.
    pub block_device: B,

    /// The super block of the file system.
    pub super_block: definition::SuperBlock,

    /// The data start block number.
    pub data_start_block: u32,
}

impl<B: BlockDevice> FileSystem<B> {
    /// Mount the file system.
    ///
    /// # Parameters
    ///
    /// * `bd` - The block device driver.
    ///
    /// # Returns
    ///
    /// * `Self` - The mounted file system.
    pub fn mount(mut bd: B) -> Self {
        let mut super_block_buf = [0u8; core::mem::size_of::<definition::SuperBlock>()];
        bd.read_block(0, 0, &mut super_block_buf).unwrap();
        let super_block = definition::SuperBlock::from_bytes(&super_block_buf).unwrap();
        let data_start_block = if super_block.fs_type == definition::FsType::Standard {
            65536
        } else {
            1024
        };

        Self {
            block_device: bd,
            super_block: *super_block,
            data_start_block,
        }
    }

    /// Synchronize the file system to the block device.
    pub fn sync(&mut self) -> Result<(), &'static str> {
        self.block_device
            .write_block(0, 0, &self.super_block.as_bytes())
    }

    /// Get the max inode (which means the file we can store in this fs)
    ///
    /// # Returns
    ///
    /// * `usize` - The max inode number.
    ///
    /// # Note
    ///
    /// If you want to get the max inode number, you should call this method and minus 1.
    ///
    /// # Example
    ///
    /// ```
    /// let fs = FileSystem::mount(bd);
    /// let max_inode = fs.get_max_inode();
    /// let max_inode_id = max_inode - 1;
    /// ```
    pub fn get_max_inode(&self) -> usize {
        ((self.data_start_block - 1) as usize * self.super_block.block_size as usize)
            / core::mem::size_of::<definition::Inode>()
    }

    /// Allocate an inode.
    ///
    /// # Returns
    ///
    /// * `(Inode, u32)` - The inode and the block number.
    fn alloc_inode(
        &mut self,
        file_type: definition::FileType,
    ) -> Result<(Inode, u32), &'static str> {
        // Alloc which bitmap has been used.
        let mut block_bitmap = &mut self.super_block.block_bitmap;
        let block_num = if let Some(i) = block_bitmap.alloc(65536).map(|i| i as u32) {
            i
        } else {
            return Err("No block available");
        };
        self.sync()?;

        // Alloc which inode has been used.
        let mut inode_bitmap = &mut self.super_block.inode_bitmap;
        let inode_num = if let Some(i) = inode_bitmap.alloc(128) {
            i as u32
        } else {
            return Err("No inode available");
        };
        self.sync()?;

        // Define that inode
        let inode = Inode {
            inode_id: inode_num,
            file_type,
            head_block: block_num, // Problem: Can't sure that the behind block is free, being optimized.
            file_length: 0,
            _reserved: [0; 8],
        };
        Ok((inode, block_num))
    }

    fn get_inode(&mut self, inode_id: u32) -> Option<Inode> {
        // First, check is the inode exists.
        let inode_bitmap = &mut self.super_block.inode_bitmap;
        if !inode_bitmap.is_used(inode_id as usize) {
            return None;
        }

        // Second, read the inode from the block device.
        let mut buf = [0u8; core::mem::size_of::<Inode>()];
        let (block_idx, offset) = Inode::locate(inode_id, &self.super_block);
        if self
            .block_device
            .read_block(block_idx as u32, offset as u32, &mut buf)
            .is_err()
        {
            return None;
        }
        let inode = Inode::from_bytes(&buf)?;
        Some(*inode)
    }

    fn add_dir_entry(
        &mut self,
        parent_inode_id: u32,
        name: &str,
        inode_id: u32,
    ) -> Result<(), &'static str> {
        // 1. Check is the parent directory exists.
        let mut parent_inode = if let Some(inode) = self.get_inode(parent_inode_id) {
            inode
        } else {
            return Err("Parent inode not found");
        };

        // 2. Calculate which block and offset the dir entry should be written.
        let entry_size = core::mem::size_of::<definition::DirEntry>();
        let data_offset = parent_inode.file_length as usize;
        parent_inode.file_length += entry_size as u64;

        let data_block_head_idx = parent_inode.head_block as usize;

        // 3. Create a dir entry.
        let name = convert_name(name.as_bytes());
        let dir_entry = definition::DirEntry {
            inode: inode_id,
            name,
        };

        // 4. Write the dir entry to the block device.
        self.block_device.write_block(
            data_block_head_idx as u32,
            data_offset as u32,
            &dir_entry.as_bytes(),
        )?;

        // 5. Update the parent inode.
        let (block_idx, offset) = Inode::locate(parent_inode_id, &self.super_block);
        self.block_device
            .write_block(block_idx as u32, offset as u32, &parent_inode.as_bytes())?;
        Ok(())
    }

    /// Create a file.
    pub fn mkfile(&mut self, parent_inode_id: u32, name: &str) -> Result<(), &'static str> {
        /* Stage 1: Allocate an inode. */
        // 1.1: Allocate an inode.
        let inode_num = self.alloc_inode(definition::FileType::Regular).unwrap();

        // 1.2: Write the inode to the block device.
        let (block_idx, offset) = Inode::locate(inode_num.0.inode_id, &self.super_block);
        self.block_device
            .write_block(block_idx as u32, offset as u32, &inode_num.0.as_bytes())?;

        /* Stage 2: Create the dir entry for its parent directory. */
        // Write the dir entry to the block device.
        self.add_dir_entry(parent_inode_id, name, inode_num.0.inode_id)?;
        Ok(())
    }

    /// Create a directory.
    pub fn mkdir(&mut self, parent_inode_id: u32, name: &str) -> Result<(), &'static str> {
        // 1. Allocate an inode.
        let inode_num = self.alloc_inode(definition::FileType::Directory).unwrap();

        // 2. Write the inode to the block device.
        let offset = inode_num.0.inode_id as usize * core::mem::size_of::<Inode>();
        self.block_device
            .write_block(inode_num.1, offset as u32, &inode_num.0.as_bytes())?;

        // 3. Create a '.' and '..' entry in the directory.
        // 3.1 Create a '.' entry.
        let dot_name = convert_name(b".");
        let dot_dir_entry = definition::DirEntry {
            inode: inode_num.0.inode_id,
            name: dot_name,
        };

        // 3.2 Create a '..' entry.
        let parent_name = convert_name(b"..");
        let dot_dot_dir_entry = definition::DirEntry {
            inode: parent_inode_id,
            name: parent_name,
        };

        // 3.3 Write the '.' and '..' entry to the block device.
        let offset = inode_num.0.inode_id as usize * core::mem::size_of::<definition::DirEntry>();
        self.block_device
            .write_block(inode_num.1, offset as u32, &dot_dir_entry.as_bytes())?;
        self.block_device.write_block(
            inode_num.1,
            (offset + core::mem::size_of::<definition::DirEntry>()) as u32,
            &dot_dot_dir_entry.as_bytes(),
        )?;

        /* Stage 4: Add dir entry to parent direcotry */
        // 4.1: Get the parent inode
        self.add_dir_entry(parent_inode_id, name, inode_num.0.inode_id)?;

        Ok(())
    }

    /// List a directory.
    pub fn ls(&mut self, inode_id: u32) -> Result<Vec<definition::DirEntry>, &'static str> {
        // 1. Check is the directory exists.
        let inode = if let Some(inode) = self.get_inode(inode_id) {
            inode
        } else {
            return Err("Inode not found");
        };

        // 2. Read the directory entries from the block device.
        let mut buf = [0u8; 1024];
        self.block_device
            .read_block(inode.head_block, 0, &mut buf)?;
        let dir_entries = unsafe {
            core::slice::from_raw_parts(buf.as_ptr() as *const definition::DirEntry, 128)
        };
        Ok(dir_entries.to_vec())
    }
}

/// Convert a name to a 256 bytes array.
///
/// # Parameters
///
/// * `name_src` - The name to convert.
///
/// # Returns
///
/// * `[u8; 252]` - The converted name.
///
/// # Example
///
/// ```rust
/// use proka_fs::convert_name;
/// let name = convert_name(b"hello");
/// ```
pub fn convert_name(name_src: &[u8]) -> [u8; 252] {
    let mut name = [0u8; 252];
    let len = name_src.len().min(name.len() - 1);
    name[..len].copy_from_slice(&name_src[..len]);
    name
}

/// Decide the file system type through the file size.
#[cfg(feature = "std")]
pub fn check_fs_type(file_path: &str) -> Result<definition::FsType, String> {
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .map_err(|_| "Failed to open file")?;
    let file_size = file.metadata().map_err(|_| "Failed to get metadata")?.len();
    if file_size > 64 * 1024 * 1024 {
        Ok(definition::FsType::Standard)
    } else {
        Ok(definition::FsType::Minimum)
    }
}

/// Get the device size in bytes.
#[cfg(feature = "std")]
pub fn get_device_size(path: &str) -> Result<u64, String> {
    let file = File::open(path).map_err(|_| "Failed to open file")?;
    let metadata = file.metadata().map_err(|_| "Failed to get metadata")?;
    Ok(metadata.len())
}
