# Motion detection

How to compile:
```bash
cargo build --release --features nomagick # only with pure axiomatic Rust crates
cargo build --release --features magick # with ImageMagick C library
```

How to cross compile:
```bash
cross build --target x86_64-unknown-linux-gnu --release --features nomagick
cross build --target arm-unknown-linux-gnueabihf --release --features nomagick
cross build --target armv7-unknown-linux-gnueabihf --release --features nomagick
```

How to run the benchmarks:
```bash
cargo bench --features nomagick
cargo flamegraph --features nomagick --bench image -- --bench
```
