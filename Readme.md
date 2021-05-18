# Operating System

Operating system development following the _Writing an OS in Rust_ series at
[os.phil-opp.com](https://os.phil-opp.com).

## Building

### Requirements

- Nightly [rust](https://www.rust-lang.org/tools/install) toolchain: `rustup toolchain install nightly`
- Binary [bootimage](https://crates.io/crates/bootimage) crate: `cargo install bootimage`
- [Qemu](https://www.qemu.org/)

To build use `cargo build`.

To run use `cargo run`.

To run all tests use `cargo test`.

## License

This project is licensed under the terms of both the MIT license and the
Apache license (Version 2.0).

See [License-MIT](License-MIT) and [License-Apache](License-Apache) for details.
