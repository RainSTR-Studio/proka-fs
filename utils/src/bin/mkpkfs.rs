//! The tool to create the proka file system.
use clap::Parser;
use colored::Colorize;
use proka_fs::bitmap::BlockBitmap;
use proka_fs::definition::{DirEntry, Inode, SuperBlock};
use proka_fs::{
    Bitmap, BlockDevice, FileBlockDevice, GenericFsData, convert_name, get_device_size,
    init_block_device,
};

// Define CLI args
#[derive(Parser)]
#[command(about = "The ProkaFS creater")]
struct Args {
    /// The path to the file to create.
    #[arg(required = true)]
    path: String,
}

fn main() {
    let result = || {
        println!(
            "{}: The file system of {}",
            "ProkaFS (PKFS)".bold(),
            "ProkaOS".bold()
        );
        println!("mkpkfs {}", "v0.1.0".cyan().bold());
        println!(
            "Copyright (C) {} {year}, All rights reserved.",
            "RainSTR Studio".cyan().bold(),
            year = "2025-2026".bold()
        );
        println!();

        /* Prework: Initialize the program */
        // Parse the CLI args.
        let args = Args::parse();
        // Open the file.
        let mut bd = init_block_device(&args.path)?;
        /* Stage 1: Initialize the super block */
        println!("mkpkfs: [INFO] Initialize the super block...");
        let mut super_block = SuperBlock::new(get_device_size(&args.path)?);
        sync(&mut bd, &mut super_block)?;
        let data_start_block = 65792; // 65536 + 256

        // Check is the size is > 64MB
        if get_device_size(&args.path)? < 64 * 1024 * 1024 {
            return Err("The partition size must be larger than 64MB".to_string());
        }

        /* Stage 2: Initialize the root inode */
        println!("mkpkfs: [INFO] Initialize the root inode...");
        let root_inode = Inode {
            is_used: true,
            inode_id: 0,
            file_type: proka_fs::definition::FileType::Directory,
            head_block: data_start_block,
            file_length: 2 * core::mem::size_of::<DirEntry>() as u64,
            _reserved: [0; 7],
        };
        bd.write_block(1, 0, root_inode.as_bytes())?;
        sync(&mut bd, &mut super_block)?;

        /* Stage 3: Init the block bitmap */
        println!("mkpkfs: [INFO] Initialize the block bitmap...");

        // 3.1: Initialize the block bitmap
        // This bitmap is 0 for all, but except 2 places:
        //
        // 1. Super Block (const, 0)
        // 2. Block bitmap itself (From `total_block - size_of::<BlockBitmap>()` to `total_block`)
        let mut block_bitmap: Vec<u8> = Vec::new();

        // 3.2: Get the bitmap start block and total block
        let bitmap_start_block = super_block.bitmap_start_block;
        let total_block = super_block.total_block;

        for _ in 0..total_block {
            block_bitmap.push(0);   // Extend vec storage
        }

        // 3.3: Set the index 0 to 1 (Superblock used)
        block_bitmap.set(0, true);

        // 3.4: Set each bitmap block's block to 1
        for i in bitmap_start_block..total_block {
            block_bitmap.set(i as usize, true);
        }

        // 3.5: Convert to &mut [u8]
        let block_bitmap = BlockBitmap::new(&mut block_bitmap);


        // 3.6: Write it to the block device
        bd.write_block(bitmap_start_block, 0, block_bitmap.as_bytes())?;

        /* Stage 4: Initialize the root directory's basic information */
        // In this stage, we will create the root directory's basic information.
        // There are 2 dir entry we MUST define:
        // - "."
        // - ".."
        // These entries are pointed at the same directory: root.
        println!("mkpkfs: [INFO] Initialize the root directory's basic information...");

        // 4.1: Define the name of the "." and ".." entries.
        let name_dot = convert_name(b".");
        let name_parent = convert_name(b"..");

        // 4.2: Define the dir entry of the "." and ".." entries.
        let entry_dot = DirEntry {
            inode: 0,
            name: name_dot,
        };
        let entry_parent = DirEntry {
            inode: 0,
            name: name_parent,
        };

        // 4.3: Write the "." and ".." entries to the root directory.
        //
        // # Note:
        // - The data block starts at block 1024, which is a constant currently.
        bd.write_block(data_start_block, 0, entry_dot.as_bytes())?;
        bd.write_block(
            data_start_block,
            core::mem::size_of::<DirEntry>() as u32,
            entry_parent.as_bytes(),
        )?;
        Ok(())
    };

    if let Err(e) = result() {
        eprintln!("mkpkfs: [ERROR] {}", e);
        eprintln!("mkpkfs: [ERROR] Terminated.");
        std::process::exit(1);
    }
}

fn sync(bd: &mut FileBlockDevice, superblock: &mut SuperBlock) -> Result<(), String> {
    bd.write_block(0, 0, superblock.as_bytes())?;
    Ok(())
}
