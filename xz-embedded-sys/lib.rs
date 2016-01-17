#![allow(non_camel_case_types)]

//! xz-embedded-sys
//!
//! FFI Bindings to the xz-embedded library, which is a simple xz decompression library
//!
//! The documentation in this crate is copied almost verbatim from the xz-embedded header file, and
//! so there might be some C-isms that aren't applicable to this rust crate.  Please read
//! carefully.
//!

extern crate libc;
use libc::{
    size_t
};


/// A wrapper around xz_ret
#[derive(Debug)]
pub struct XZRawError {
    pub code: xz_ret
}

impl std::fmt::Display for XZRawError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XZRawError: {:?}", self.code)
    }
}

impl std::convert::From<xz_ret> for XZRawError {
    fn from(e: xz_ret) -> XZRawError {
        XZRawError{code: e}
    }
}

impl std::error::Error for XZRawError {
    fn description(&self) -> &str { 
        match self.code {
            xz_ret::XZ_OK => "Everything is OK so far",
            xz_ret::XZ_STREAM_END => "Operation finished successfully",
            xz_ret::XZ_UNSUPPORTED_CHECK => "Integrity check type is not supported",
            xz_ret::XZ_MEM_ERROR => "Allocating memory failed",
            xz_ret::XZ_MEMLIMIT_ERROR => "A bigger LZMA2 dictionary is needed than allowed by dict_max",
            xz_ret::XZ_FORMAT_ERROR => "File format was not recognized",
            xz_ret::XZ_OPTIONS_ERROR => "This implementation doesn't support the requested compression options",
            xz_ret::XZ_DATA_ERROR => "Compressed data is corrupt",
            xz_ret::XZ_BUF_ERROR => "Cannot make any progress"
        }
    }
}

/// Operation mode
///
/// It is possible to enable support only for a subset of the above
/// modes at compile time by defining XZ_DEC_SINGLE, XZ_DEC_PREALLOC,
/// or XZ_DEC_DYNALLOC. The xz_dec kernel module is always compiled
/// with support for all operation modes, but the preboot code may
/// be built with fewer features to minimize code size.
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

/// Return codes
///
/// In multi-call mode, XZ_BUF_ERROR is returned when two consecutive calls
/// to XZ code cannot consume any input and cannot produce any new output.
/// This happens when there is no new input available, or the output buffer
/// is full while at least one output byte is still pending. Assuming your
/// code is not buggy, you can get this error only when decoding a compressed
/// stream that is truncated or otherwise corrupt.
///                                                                           
/// In single-call mode, XZ_BUF_ERROR is returned only when the output buffer
/// is too small or the compressed input is corrupt in a way that makes the
/// decoder produce more output than the caller expected. When it is
/// (relatively) clear that the compressed input is truncated, XZ_DATA_ERROR
/// is used instead of XZ_BUF_ERROR.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum xz_ret {

    /// Everything is OK so far. More input or more
    /// output space is required to continue. This
    /// return code is possible only in multi-call mode
    /// (XZ_PREALLOC or XZ_DYNALLOC).
	XZ_OK,

    /// Operation finished successfully.
	XZ_STREAM_END,

    /// Integrity check type is not supported. Decoding
    /// is still possible in multi-call mode by simply
    /// calling xz_dec_run() again.
    /// Note that this return value is used only if
    /// XZ_DEC_ANY_CHECK was defined at build time,
    /// which is not used in the kernel. Unsupported
    /// check types return XZ_OPTIONS_ERROR if
    /// XZ_DEC_ANY_CHECK was not defined at build time.
	XZ_UNSUPPORTED_CHECK,

    /// Allocating memory failed. This return code is
    /// possible only if the decoder was initialized
    /// with XZ_DYNALLOC. The amount of memory that was
    /// tried to be allocated was no more than the
    /// dict_max argument given to xz_dec_init().
	XZ_MEM_ERROR,

    /// A bigger LZMA2 dictionary would be needed than
    /// allowed by the dict_max argument given to
    /// xz_dec_init(). This return value is possible
    /// only in multi-call mode (XZ_PREALLOC or
    /// XZ_DYNALLOC); the single-call mode (XZ_SINGLE)
    /// ignores the dict_max argument.
	XZ_MEMLIMIT_ERROR,

    /// File format was not recognized (wrong magic
    /// bytes).
	XZ_FORMAT_ERROR,

    /// This implementation doesn't support the requested
    /// compression options. In the decoder this means
    /// that the header CRC32 matches, but the header
    /// itself specifies something that we don't support.
	XZ_OPTIONS_ERROR,

    /// Compressed data is corrupt.
	XZ_DATA_ERROR,

    /// Cannot make any progress. Details are slightly
    /// different between multi-call and single-call
    /// mode; more information below.
	XZ_BUF_ERROR
}

///  Passing input and output buffers to XZ code
///
///
#[repr(C)]
pub struct xz_buf {
    /// Beginning of the input buffer. This may be NULL if and only
    /// if in_pos is equal to in_size.
    pub _in: *const u8,
    /// Current position in the input buffer. This must not exceed
    /// in_size.
    pub in_pos: size_t,
    /// Size of the input buffer
    ///
    pub in_size: size_t,

    /// Beginning of the output buffer. This may be NULL if and only
    /// if out_pos is equal to out_size.
    pub out: *mut u8,
    /// Current position in the output buffer. This must not exceed
    /// out_size.
    pub out_pos: size_t,
    /// Size of the output buffer
    pub out_size: size_t
}

/// Opaque type to hold the XZ decoder state
pub enum xz_dec {}

extern "C" {
    /// Allocate and initialize a XZ decoder state
    ///
    /// @mode: Operation mode
    ///
    /// @dict_max: Maximum size of the LZMA2 dictionary (history buffer) for
    /// multi-call decoding. This is ignored in single-call mode
    /// (mode == XZ_SINGLE). LZMA2 dictionary is always 2^n bytes
    /// or 2^n + 2^(n-1) bytes (the latter sizes are less common
    /// in practice), so other values for dict_max don't make sense.
    /// In the kernel, dictionary sizes of 64 KiB, 128 KiB, 256 KiB,
    /// 512 KiB, and 1 MiB are probably the only reasonable values,
    /// except for kernel and initramfs images where a bigger
    /// dictionary can be fine and useful.
    ///
    /// Single-call mode (XZ_SINGLE): xz_dec_run() decodes the whole stream at
    /// once. The caller must provide enough output space or the decoding will
    /// fail. The output space is used as the dictionary buffer, which is why
    /// there is no need to allocate the dictionary as part of the decoder's
    /// internal state.
    ///
    /// Because the output buffer is used as the workspace, streams encoded using
    /// a big dictionary are not a problem in single-call mode. It is enough that
    /// the output buffer is big enough to hold the actual uncompressed data; it
    /// can be smaller than the dictionary size stored in the stream headers.
    ///
    /// Multi-call mode with preallocated dictionary (XZ_PREALLOC): dict_max bytes
    /// of memory is preallocated for the LZMA2 dictionary. This way there is no
    /// risk that xz_dec_run() could run out of memory, since xz_dec_run() will
    /// never allocate any memory. Instead, if the preallocated dictionary is too
    /// small for decoding the given input stream, xz_dec_run() will return
    /// XZ_MEMLIMIT_ERROR. Thus, it is important to know what kind of data will be
    /// decoded to avoid allocating excessive amount of memory for the dictionary.
    ///
    /// Multi-call mode with dynamically allocated dictionary (XZ_DYNALLOC):
    /// dict_max specifies the maximum allowed dictionary size that xz_dec_run()
    /// may allocate once it has parsed the dictionary size from the stream
    /// headers. This way excessive allocations can be avoided while still
    /// limiting the maximum memory usage to a sane value to prevent running the
    /// system out of memory when decompressing streams from untrusted sources.
    ///
    /// On success, xz_dec_init() returns a pointer to struct xz_dec, which is
    /// ready to be used with xz_dec_run(). If memory allocation fails,
    /// xz_dec_init() returns NULL.
    pub fn xz_dec_init(mode: xz_mode, dict_max: u32) -> *mut xz_dec;

    /// Run the XZ decoder
    ///
    /// @s:          Decoder state allocated using xz_dec_init()
    ///
    /// @b:          Input and output buffers
    ///
    ///
    /// The possible return values depend on build options and operation mode.
    /// See enum xz_ret for details.
    ///
    /// Note that if an error occurs in single-call mode (return value is not
    /// XZ_STREAM_END), b->in_pos and b->out_pos are not modified and the
    /// contents of the output buffer from b->out[b->out_pos] onward are
    /// undefined. This is true even after XZ_BUF_ERROR, because with some filter
    /// chains, there may be a second pass over the output buffer, and this pass
    /// cannot be properly done if the output buffer is truncated. Thus, you
    /// cannot give the single-call decoder a too small buffer and then expect to
    /// get that amount valid data from the beginning of the stream. You must use
    /// the multi-call decoder if you don't want to uncompress the whole stream.
    pub fn xz_dec_run(s: *mut xz_dec, b: *mut xz_buf) -> xz_ret;

    /// Reset an already allocated decoder state
    ///
    /// @s:          Decoder state allocated using xz_dec_init()
    ///
    /// This function can be used to reset the multi-call decoder state without
    /// freeing and reallocating memory with xz_dec_end() and xz_dec_init().
    ///                                                                          
    /// In single-call mode, xz_dec_reset() is always called in the beginning of
    /// xz_dec_run().  Thus, explicit call to xz_dec_reset() is useful only in
    /// multi-call mode.
    pub fn xz_dec_reset(s: *mut xz_dec);

    /// Free the memory allocated for the decoder state
    ///
    /// @s: Decoder state allocated using xz_dec_init(). If s is NULL, this function does nothing.
    pub fn xz_dec_end(s: *mut xz_dec);


    /// Initialize the CRC32 lookup table
    ///
    /// This must be called before any other xz_* function to initialize
    /// the CRC32 lookup table.
    pub fn xz_crc32_init();

    /// Update CRC32 value using the polynomial from IEEE-802.3.
    ///
    /// To start a new calculation, the third argument must be zero. To continue the calculation,
    /// the previously returned value is passed as the third argument.
    pub fn xz_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;


    /// Initialize the CRC64 lookup table
    ///
    /// This must be called before any other xz_* function (except xz_crc32_init())
    /// to initialize the CRC64 lookup table.
    pub fn xz_crc64_init();

    /// Update CRC64 value using the polynomial from ECMA-182.
    ///
    /// To start a new calculation, the third argument must be zero. To continue the calculation,
    /// the previously returned value is passed as the third argument.
    pub fn xz_crc64(buf: *const u8, size: size_t, crc: u64) -> u64;

}



#[test]
fn test_full_hello_decompress() {
    let data: Vec<u8> = vec!(
        0xfd,0x37,0x7a,0x58,0x5a,0x00,0x00,0x04,0xe6,0xd6,0xb4,0x46,0x02,0x00,0x21,0x01,
        0x16,0x00,0x00,0x00,0x74,0x2f,0xe5,0xa3,0x01,0x00,0x04,0x68,0x65,0x6c,0x6c,0x6f,
        0x00,0x00,0x00,0x00,0xb1,0x37,0xb9,0xdb,0xe5,0xda,0x1e,0x9b,0x00,0x01,0x1d,0x05,
        0xb8,0x2d,0x80,0xaf,0x1f,0xb6,0xf3,0x7d,0x01,0x00,0x00,0x00,0x00,0x04,0x59,0x5a
    );
    unsafe {
        xz_crc32_init();
        xz_crc64_init();

        let state = xz_dec_init(xz_mode::XZ_DYNALLOC, 1 << 26);
  
        let mut out_buf: [u8; 32] = [0; 32];
        let in_buf = data;

        let mut buf = xz_buf {
            _in: in_buf.as_ptr(),
            in_size: in_buf.len() as u64,
            in_pos:0,

            out: out_buf.as_mut_ptr(),
            out_pos: 0,
            out_size: 32,
            
        };

        let ret = xz_dec_run(state, &mut buf);
        println!("ret={:?}", ret);
        println!("out_pos: {}", buf.out_pos);
        println!("out_size: {}", buf.out_size);
        let mut v = Vec::from(&out_buf[..]);
        v.truncate(buf.out_pos as usize);
        println!("in_pos: {}", buf.in_pos);
        xz_dec_end(state);
        
        assert_eq!(ret, xz_ret::XZ_STREAM_END);
        assert_eq!(buf.out_pos, 5);
        assert_eq!(buf.in_size, buf.in_pos);
        assert_eq!(v, "hello".as_bytes());

    }
}



#[test]
fn test_partial_hello_decompress() {
    let data: Vec<u8> = vec!(
        0xfd,0x37,0x7a,0x58,0x5a,0x00,0x00,0x04,0xe6,0xd6,0xb4,0x46,0x02,0x00,0x21,0x01,
        0x16,0x00,0x00,0x00,0x74,0x2f,0xe5,0xa3,0x01,0x00,0x04,0x68,0x65,0x6c,0x6c,0x6f,
        0x00,0x00,0x00,0x00,0xb1,0x37,0xb9,0xdb,0xe5,0xda,0x1e,0x9b,0x00,0x01,0x1d,0x05,
        0xb8,0x2d,0x80,0xaf,0x1f,0xb6,0xf3,0x7d,0x01,0x00,0x00,0x00,0x00,0x04,0x59,0x5a
    );
    unsafe {
        xz_crc32_init();
        xz_crc64_init();

        let state = xz_dec_init(xz_mode::XZ_DYNALLOC, 1 << 26);
  
        let mut out_buf: [u8; 32] = [0; 32];
        let in_buf = data;

        let mut buf = xz_buf {
            _in: in_buf.as_ptr(),
            in_size: in_buf.len() as u64,
            in_pos:0,

            out: out_buf.as_mut_ptr(),
            out_pos: 0,
            out_size: 2,
            // set out_size to be smaller than "hello", so that two calls to xz_dec_run are needed 
        };

        let ret = xz_dec_run(state, &mut buf);
        println!("ret={:?}", ret);
        println!("out_pos: {}", buf.out_pos);
        println!("out_size: {}", buf.out_size);
        let mut v = Vec::from(&out_buf[..]);
        v.truncate(buf.out_pos as usize);
        println!("in_pos: {}", buf.in_pos);
        
        assert_eq!(ret, xz_ret::XZ_OK);
        assert_eq!(buf.out_pos, 2);
        assert_eq!(v, "he".as_bytes());

        buf.out_size = 5;
        let ret = xz_dec_run(state, &mut buf);
        println!("ret={:?}", ret);
        assert_eq!(ret, xz_ret::XZ_STREAM_END);
        let mut v = Vec::from(&out_buf[..]);
        v.truncate(buf.out_pos as usize);
        assert_eq!(v, "hello".as_bytes());



    }
}
