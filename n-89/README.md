# n-89
Welcome to your Nspire project!

## Prerequisites
- Ndless toolchain installed and added to path
- Rustup installed
- Latest Rust Nightly installed (nightly-2019-04-20 works)
- `jq`
- Unix-like (tested on Linux, most likely Mac and Cygwin will work as well)

Complete install script:
```bash
curl https://sh.rustup.rs -sSf | sh # skip if rustup already installed
rustup install nightly # skip if nightly already installed
cargo install cargo-make
```

Get started by running

```bash
cargo +nightly make dev
```

to start development. Your .tns file will be available in
`target/armv5te-nspire-eabi/debug/n-89.tns`.

When you're ready to release your application,
**don't forget to compile in release mode** with

```bash
cargo +nightly make release
```

Your .tns file will be available in
`target/armv5te-nspire-eabi/release/n-89.tns`

You may skip `+nightly` if you set nightly as your default compiler
(`rustup default nightly`), or
[set a directory override](https://github.com/rust-lang/rustup.rs#directory-overrides).

Check out the [ndless examples](https://github.com/lights0123/example-nspire)
for more info.
