# xz-decom
XZ Decompression using xz-embedded

This crate provides XZ decompression using the [xz-embedded](http://tukaani.org/xz/embedded.html) library.
This means that compression and perhaps some advanced features are not supported.

Tested on rust stable (1.5), beta (1.6) and nightly (1.7).  
Tested on OSX and Linux

# Usage

Add the following to your Cargo.toml file:

```toml
[dependencies]
xz-decom = "0.2"
```

Example:

```rust
extern crate xz_decom;
use xz_decom::decompress;

let data = include_bytes!("data/hello.xz");

let result = decompress(data).unwrap();
assert_eq!(result, "hello".as_bytes());

```

# Documentation

Available here : https://eminence.github.io/xz-decom/doc/xz_decom/index.html


# License

Licensed under either of Apache License, Version 2.0 or MIT, at your option

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
