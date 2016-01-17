extern crate xz_decom;

use xz_decom::decompress;
use std::error::Error;

#[test]
fn test_hello() {
    let data = include_bytes!("data/hello.xz");

    let result = decompress(data).unwrap();
    assert_eq!(result, "hello".as_bytes());
}

#[test]
fn test_error() {
    let data = "not a valid xz file".as_bytes();

    match decompress(data) {
        Err(err) => {
            let cause = err.cause().unwrap();
            println!("{}: {}", err.description(), cause.description());
        },
        Ok(..) => {
            panic!("decompress should have returned error!");
        }
    }
}

#[test]
fn test_partial() {
    let data = include_bytes!("data/hello.xz");

    match decompress(&data[0..30]) {
        Err(err) => {
            assert!(err.cause().is_none());
            println!("{:?}", err);
        },
        Ok(..) => {
            panic!("decompress should have returned error!");
        }
    }
}
