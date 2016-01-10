#![allow(non_camel_case_types)]

extern crate libc;
use libc::{
    size_t
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum xz_mode {
    /// Single-call mode.
    ///
    /// This uses less RAM than than multi-call modes, because the LZMA2 dictionary doesn't need to
    /// be allocated as part of the decoder state. All required data structures are allocated at
    /// initialization, so xz_dec_run() cannot return XZ_MEM_ERROR.
	XZ_SINGLE,

    /// Multi-call mode with preallocated LZMA2 dictionary buffer.
    ///
    /// All data structures are allocated at initialization, so xz_dec_run() cannot return
    /// XZ_MEM_ERROR.
	XZ_PREALLOC,

    /// Multi-call mode.
    ///
    /// The LZMA2 dictionary is allocated once the required size has been parsed from the stream
    /// headers. If the allocation fails, xz_dec_run() will return XZ_MEM_ERROR.
	XZ_DYNALLOC
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum xz_ret {
	XZ_OK,
	XZ_STREAM_END,
	XZ_UNSUPPORTED_CHECK,
	XZ_MEM_ERROR,
	XZ_MEMLIMIT_ERROR,
	XZ_FORMAT_ERROR,
	XZ_OPTIONS_ERROR,
	XZ_DATA_ERROR,
	XZ_BUF_ERROR
}

#[repr(C)]
pub struct xz_buf {
    pub _in: *const u8,
    pub in_pos: size_t,
    pub in_size: size_t,

    pub out: *mut u8,
    pub out_pos: size_t,
    pub out_size: size_t
}

/// Opaque type to hold the XZ decoder state
pub enum xz_dec {}

extern "C" {
    pub fn xz_dec_init(mode: xz_mode, dict_max: u32) -> *mut xz_dec;
    pub fn xz_dec_run(s: *mut xz_dec, b: *mut xz_buf) -> xz_ret;
    pub fn xz_dec_reset(s: *mut xz_dec);
    pub fn xz_dec_end(s: *mut xz_dec);

    pub fn xz_crc32_init();
    pub fn xz_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;

    pub fn xz_crc64_init();
    pub fn xz_crc64(buf: *const u8, size: size_t, crc: u64) -> u64;

}


#[test]
fn it_works() {
}
