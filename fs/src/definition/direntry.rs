/// The entry point of directory.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DirEntry {
    /// The inode number of the directory.
    pub inode: u32,

    /// The name of the directory, which contains up to 255 characters.
    pub name: [u8; 252],
}

impl DirEntry {
    pub const fn empty() -> Self {
        Self {
            inode: 0,
            name: [0; 252],
        }
    }
}

impl crate::GenericFsData for DirEntry {
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

    fn from_bytes(bytes: &[u8]) -> Option<&Self>
        where
            Self: Sized {
        if bytes.len() < core::mem::size_of::<Self>() {
            return None;
        }
        let dir_entry = unsafe { &*(bytes[0] as *const u8 as *const Self) };
        Some(dir_entry)
    }

    fn from_mut_bytes(buf: &mut [u8]) -> Option<&mut Self> {
        if buf.len() < core::mem::size_of::<Self>() {
            return None;
        }
        let dir_entry = unsafe { &mut *(buf[0] as *mut u8 as *mut Self) };
        Some(dir_entry)
    }
}
