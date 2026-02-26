# Proka FS Library

This crate is the library of ProkaFS, which is the filesystem of ProkaOS.

It defined lots of generic filesystem type, and provided some APIs to operate the 
files/directories of ProkaOS.

Note that this library is `no_std` as default, unless you enable the `std` feature. 

Also, you must implement `global_allocator` feature to use this library in `no_std` environment.

# Available Features

- `std`: Enable the standard library features (Uses in Windows/macOS/Linux)

# Contributing

Contributions are welcome, and we will thank you very much!

If you want to do some contributions for us, perhaps you can see
[The contribution docs](https://github.com/RainSTR-Studio/proka-fs/blob/main/CONTRIBUTING.md) and 
follow the contribution guide to do some contributions.

Note that your contribution should be useful, and obey `Keep It Simple, Stupid` (KISS) principle.

# Examples

```rust
use std::fs::{self, OpenOptions};
use proka_fs::{FileSystem, init_block_device};


fn main() -> Result<(), &'static str> {
    // Set the file you want to operate is `disk.img`.
    let file_path = "disk.img";

    // Initialize file system block device
    let bd = init_block_device(file_path).unwrap();

    // Mount the file system
    let mut fs = FileSystem::mount(bd);

    // Then enjoy operating it! :)
    fs.mkdir(0, "new_dir")?; // parent_inode=0 is the root directory
    fs.mkfile(0, "new_file.txt")?;

    Ok(())
}

```

# License

This project is licensed under the GNU General Public License v3.0 - see the 
[LICENSE](https://github.com/RainSTR-Studio/proka-fs/tree/main/LICENSE) file for more details.