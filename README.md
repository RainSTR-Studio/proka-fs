# ProkaFS - The filesystem of ProkaOS

[![Rust Stable](https://img.shields.io/badge/rust-stable-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/) [![License: GPL v3.0](https://img.shields.io/badge/License-GPL%203.0-yellow.svg?style=flat-square)](https://opensource.org/licenses/GPL-3.0) [![GitHub Stars](https://img.shields.io/github/stars/RainSTR-Studio/proka-fs?style=flat-square)](https://github.com/RainSTR-Studio/proka-fs/stargazers) [![GitHub Issues](https://img.shields.io/github/issues/RainSTR-Studio/proka-kernel?style=flat-square)](https://github.com/RainSTR-Studio/proka-kernel/issues) [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RainSTR-Studio/proka-kernel?style=flat-square)](https://github.com/RainSTR-Studio/proka-kernel/pulls)

**Copyright (C) RainSTR Studio 2026, All rights reserved.**

---

Welcome to use ProkaFS, which is prepared for ProkaOS. 
Primarily for practice, and we hope this can be supported in more operating systems.

## Project Highlights
 - **Memory safe**: Written in **Rust**, which is very safe in memory;
 - **Lightweight**: The filesystem is very lightweight, and it can be used in embedded systems;
 - **Multiple Filesystem Type**: Supports 2 file system type, in order to adapt different disk space.
 - **Separated modules**: The filesystem is separated into 2 modules, which are `proka-fs` and `pkfs-utils`.

## How-to-build

### Requirements
 - **Rust**: ProkaFS is written in Rust, so you need a Rust compiler.

### Steps

1. **Install necessary components**

We suggest you to use **rustup** to install Rust.

```bash
# This command is from https://rustup.rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. **Clone the repository**
After installing these tools, clone this repository.

```bash
# Origin address
git clone https://github.com/RainSTR-Studio/proka-fs.git

# If you are in China, this address might faster than original one.
git clone https://ghfast.top/https://github.com/RainSTR-Studio/proka-fs.git
```

3. **Build the project**

Execute the following commands to build the project.

```bash
cd proka-fs
cargo build --release
```

This will generate these executable files in `target/release`:

 - `mkpkfs`: The ProkaFS creator;
 - `ckpkfs`: The checker and fixer of ProkaFS *(todo)*

For more usages, please type `<command> --help` in the terminal.

## Contributors

Thanks to all contributors who have helped improve ProkaFS:

 - zhangxuan2011 <zx20110412@outlook.com>

If you want to contribute to this project, you can follow the guide in [CONTRIBUTING.md](CONTRIBUTING.md).

### Leave your name here

By the way, don't forget to add your name in these positions:

 - [Contributors List](#contributors);
 - [Cargo metadata in `fs/Cargo.toml`](fs/Cargo.toml);
 - [Cargo metadata in `utils/Cargo.toml`](utils/Cargo.toml);

## Open Source License

This project is currently using [**GPL-3.0**](LICENSE) license.
See [`LICENSE`](LICENSE) for more details.