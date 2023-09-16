# Motion Detection
![badge](https://framagit.org/adjivas/motion_detection/badges/master/pipeline.svg)

This service will captures a MJPEG video and produces a MQTT's motion messages.

It's was designed for the [BazzDoorbell](https://github.com/guino/BazzDoorbell) project.

## What is in this crate?
This service is a Rust axiomatic solution based on the image-compare/image crates.

## Play with it!
How to configure the environment:
```shell
cp env.example.sh env.sh
$EDITOR env.sh
source env.sh
```

How to compile:
```shell
cargo build --release
```

How to run the service:
```shell
cargo run
```

How to cross compile:
```shell
cross build --target x86_64-unknown-linux-gnu --release
cross build --target armv7-unknown-linux-gnueabihf --release
```

## Test the performences
How to run the benchmarks:
```shell
cargo bench
```

How to run FlameGraph:
```shell
cargo bench --all-features --bench image -- --profile-time=5
```
