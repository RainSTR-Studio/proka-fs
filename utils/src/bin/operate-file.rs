use proka_fs::{FileSystem, init_block_device};

fn main() {
    let bd = init_block_device("/Users/zhangxuan/Desktop/proka.img").unwrap();
    let mut fs = FileSystem::mount(bd);

    fs.mkfile(0, "test.txt").unwrap();
}
