# Motion detection
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
```
