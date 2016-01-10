extern crate gcc;

use std::path::Path;

fn main() {

    let files = vec!("xz_crc32.c", "xz_crc64.c", "xz_dec_stream.c", "xz_dec_lzma2.c", "xz_dec_bcj.c");
    let src_dir = Path::new("xz-embedded/linux/lib/xz/");
    let inc_dir = Path::new("xz-embedded/linux/include/linux");

    let mut cfg = gcc::Config::new();

    for file in files {
        cfg.file(src_dir.join(file));
    }
    cfg.include(inc_dir);
    cfg.include("xz-embedded/userspace/");

    cfg.define("XZ_USE_CRC64", None)
       .define("XZ_DEC_ANY_CHECK", None)
       .flag("-std=gnu89")
       .flag("-ggdb3")
       .flag("-pedantic")
       .flag("-Wall")
       .flag("-Wextra")
       .opt_level(2);

    cfg.compile("libxzembedded.a");



}
