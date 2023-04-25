# Motion detection
This project will captures a MJPEG video and produces a MQTT's motion messages.
It's was designed for the [BazzDoorbell](https://github.com/guino/BazzDoorbell) project.

## What is in this crate?
This project has two exclusive modes:
- The nomagick feature is a Rust axiomatic solution based on the image-compare/image crates.
- The magick feature is a C dependency based on ImageMagick, isn't yet cross compilable with the arm architecture (See [magick-rust!38](https://github.com/nlfiedler/magick-rust/issues/38))

## Play with it!
How to configure the environment:
```shell
cp env.example.sh env.sh # you have to adapt the environment variables
source env.sh
```

How to compile:
```shell
cargo build --release --features nomagick # only with pure axiomatic Rust crates
cargo build --release --features magick # with ImageMagick C library
```

How to cross compile:
```shell
cross build --target x86_64-unknown-linux-gnu --release --features nomagick
cross build --target x86_64-unknown-linux-gnu --release --features magick
cross build --target armv7-unknown-linux-gnueabihf --release --features nomagick
```

How to run the benchmarks:
```shell
cargo bench --features nomagick
cargo flamegraph --features nomagick --bench image -- --bench
cargo bench --features magick
cargo flamegraph --features magick --bench image -- --bench
```
