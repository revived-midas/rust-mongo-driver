extern crate libc;

use libc::c_char;
use libc::size_t;
use std::ffi::CStr;
use std::str;

#[repr(C)]
pub struct BSONDocument {
    flags: u32,
    pub len: u32,
    padding: [u8;120],
}

#[link(name = "bson-1.0")]
extern {
    fn bson_get_major_version() -> i32;
    fn bson_new() -> *mut BSONDocument;
    fn bson_append_int32(bson: *const BSONDocument, key: *const u8, key_length: i32, value: i32) -> bool;
    fn bson_as_json(bson: *const BSONDocument, length: *mut size_t) -> *mut c_char;
}

pub fn get_sample_bson_doc() -> *mut BSONDocument {
    unsafe {
        let y = bson_new();
        bson_append_int32(y, (b"foo").as_ptr(), 3, 42 as i32);
        y
    }
}

pub fn bson_to_json(doc: *const BSONDocument) -> String {
    let mut size = 0 as size_t;
    let s: *mut c_char;

    let c_buf: *const c_char = unsafe { bson_as_json(doc, &mut size) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = std::str::from_utf8(buf).unwrap();
    str_slice.to_owned()
}
