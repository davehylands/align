Simple project for determining the alignment for various architectures.

To use, install an appropriate target using `rustup target add TARGETNAME`

You can then build using `cargo build --target=TARGETNAME` and disassemble the lib using:
`llvm-objdump -d target/TARGETNAME/debug/libalign.a`

I tested the following:

* aarch64-apple-ios
* armv5te-unknown-linux-gnueabi
* armv7-linux-androideabi
* i686-linux-android
* x86_64-apple-darwin
* x86_64-apple-ios

and they all produced 1, 2, 4, and 8 for the alignment of u8, u16, u32, and u64
with the exception that i686-linux-android produced an alignment of 4 for u64.

