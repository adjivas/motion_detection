#![feature(test)]

extern crate test;

mod data;

use data::{PATH_DATA, RAW_DATA};

use image::io::Reader;

use std::path::Path;
use std::io::Cursor;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    fn load_from_memory() {
        assert!(image::load_from_memory(RAW_DATA).is_ok());
    }

    fn open() {
        let path = Path::new(PATH_DATA);

        assert!(image::open(path).is_ok());
    }

    fn reader() {
        let reader = Reader::new(Cursor::new(RAW_DATA)).with_guessed_format().expect("REASON");

        assert!(reader.decode().is_ok());
    }

    #[bench]
    fn bench_load_from_memory(bench: &mut Bencher) {
        bench.iter(|| load_from_memory())
    }

    #[bench]
    fn bench_reader(bench: &mut Bencher) {
        bench.iter(|| reader());
    }

    #[bench]
    fn bench_open(bench: &mut Bencher) {
        bench.iter(|| open());
    }
}
