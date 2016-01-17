extern crate xz_embedded_sys as raw;


pub fn decompress(compressed_data: &[u8]) -> Vec<u8> {
    unsafe {
        raw::xz_crc32_init();
        raw::xz_crc64_init();
    }
    let state = unsafe { 
        raw::xz_dec_init(raw::xz_mode::XZ_DYNALLOC, 1 << 26)
    };
    if state.is_null() { panic!("failed to init") }

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
            out_vec.extend_from_slice(&out_buf[0..(buf.out_pos as usize)]);
            buf.out_pos = 0;
        } else if ret == raw::xz_ret::XZ_STREAM_END {
            out_vec.extend_from_slice(&out_buf[0..(buf.out_pos as usize)]);
            break;
        } else {
            panic!("Decompressing error {:?}", ret);
        }
        if buf.in_pos == buf.in_size {
            println!("Reached end of in buffer");
            break;
        }
    }

    unsafe { raw::xz_dec_end(state) };

    out_vec
    

}


#[test]
fn it_works() {

    unsafe {
        raw::xz_crc32_init();
        raw::xz_crc64_init();

        let state = raw::xz_dec_init(raw::xz_mode::XZ_DYNALLOC, 1 << 26);
  
        let mut out_buf: [u8; 32] = [0; 32];
        let in_buf = include_bytes!("/storage/home/achin/devel/xz-embedded/userspace/hello.xz");

        let mut buf = raw::xz_buf {
            _in: in_buf.as_ptr(),
            in_size: in_buf.len() as u64,
            in_pos:0,

            out: out_buf.as_mut_ptr(),
            out_pos: 0,
            out_size: 2,
            
        };

        let ret = raw::xz_dec_run(state, &mut buf);
        println!("ret={:?}", ret);
        println!("out_pos: {}", buf.out_pos);
        println!("out_size: {}", buf.out_size);
        let mut v = Vec::from(&out_buf[..]);
        v.truncate(buf.out_pos as usize);
        println!("{}", String::from_utf8(v).unwrap());
        println!("in_pos: {}", buf.in_pos);


    }

}
