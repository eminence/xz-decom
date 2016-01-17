//! xz-decom
//!
//! XZ Decompression using xz-embedded
//!
//! This crate provides XZ decompression using the xz-embedded library. 
//! This means that compression and perhaps some advanced features are not supported.
//!

extern crate xz_embedded_sys as raw;

use std::error::Error;
use std::fmt;

/// Error type for problems during decompression
#[derive(Debug)]
pub struct XZError {
    msg: &'static str,
    code: Option<raw::XZRawError>
}

impl Error for XZError {
    fn description(&self) -> &str { self.msg }
    fn cause<'a>(&'a self) -> Option<&'a Error> {
        if let Some(ref e) = self.code {
            Some(e)
        } else {
            None
        }
    }
}

impl fmt::Display for XZError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Decompress some data
///
/// The input slice should contain the full chunk of data to decompress.  There is no support for
/// partial decompression
///
/// #  Example
///
/// Pretty simple:
///
/// ```ignore
/// let data = include_bytes!("data/hello.xz");
///
/// let result = decompress(data).unwrap();
/// assert_eq!(result, "hello".as_bytes());
/// ```
///
pub fn decompress(compressed_data: &[u8]) -> Result<Vec<u8>, XZError> {
    unsafe {
        // Note that these return void, and can't fail
        raw::xz_crc32_init();
        raw::xz_crc64_init();
    }
    let state = unsafe { 
        raw::xz_dec_init(raw::xz_mode::XZ_DYNALLOC, 1 << 26)
    };
    if state.is_null() {
        return Err(XZError{msg: "Failed to initialize", code: None});
    }

    let mut out_vec = Vec::new();

    let out_size = 4096;
    let mut out_buf = Vec::with_capacity(out_size);
    out_buf.resize(out_size, 0);

    let mut buf = raw::xz_buf {
        _in: compressed_data.as_ptr(),
        in_size: compressed_data.len() as u64,
        in_pos:0,

        out: out_buf.as_mut_ptr(),
        out_pos: 0,
        out_size: out_size as u64,
    };

    loop {
        let ret = unsafe { raw::xz_dec_run(state, &mut buf) };
        //println!("Decomp returned {:?}", ret);
        if ret == raw::xz_ret::XZ_OK {
            out_vec.extend(&out_buf[0..(buf.out_pos as usize)]);
            buf.out_pos = 0;
        } else if ret == raw::xz_ret::XZ_STREAM_END {
            out_vec.extend(&out_buf[0..(buf.out_pos as usize)]);
            break;
        } else {
            return Err(XZError{msg: "Decompressing error", code: Some(raw::XZRawError::from(ret))})
        }
        if buf.in_pos == buf.in_size {
            // if we're reached the end of out input buffer, but we didn't hit
            // XZ_STREAM_END, i think this is an error
            return Err(XZError{msg: "Reached end of input buffer", code: None})
        }
    }

    unsafe { raw::xz_dec_end(state) };

    Ok(out_vec)
    

}


