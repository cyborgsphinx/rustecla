extern crate libc;
use libc::{c_char, c_int, size_t};
use libc::types::common::c95::FILE;

use std::ffi::CString;
use std::string::as_string;
use std::str::from_utf8_unchecked;

#[repr(C)]
pub struct GetLine;
pub struct ExpandFile;
pub struct FileExpansion {
    exists: c_int,
    nfile: c_int,
    files: *mut char,
}
pub struct WordCompletion;

#[link(name = "tecla")]
extern {
    fn libtecla_version(major: *mut c_int, minor: *mut c_int, micro: *mut c_int);
    fn new_GetLine(linelen: size_t, histlen: size_t) -> *mut GetLine;
    fn del_GetLine(gl: *mut GetLine) -> *mut GetLine;
    fn gl_get_line(gl: *mut GetLine,
                   prompt: *const c_char,
                   start_line: *const c_char,
                   start_pos: c_int) -> *const c_char;
    fn gl_query_char(gl: *mut GetLine, prompt: *const c_char, defchar: c_char) -> c_int;
    fn gl_read_char(gl: *mut GetLine) -> c_int;
    fn gl_configure_getline(gl: *mut GetLine, app_string: *const c_char,
                            app_file: *const c_char, user_file: *const c_char) -> c_int;
    // fn gl_bind_keyseq() -> c_int;
    fn new_ExpandFile() -> *mut ExpandFile;
    fn del_ExpandFile(ef: *mut ExpandFile) -> *mut ExpandFile;
    fn ef_expand_file(ef: *mut ExpandFile,
                      path: *const c_char,
                      pathlen: c_int) -> *mut FileExpansion;
    fn el_list_expansions(result: *mut FileExpansion,
                          fp: *mut FILE,
                          term_width: c_int) -> c_int;
    fn ef_last_error(ef: *mut ExpandFile) -> *const c_char;
    fn new_WordCompletion() -> *mut WordCompletion;
    fn del_WordCompletion(cpl: *mut WordCompletion) -> WordCompletion;
}

pub fn new_gl(linelen: usize, histlen: usize) -> *mut GetLine {
    let line = linelen as size_t;
    let hist = histlen as size_t;
    let mut res: *mut GetLine;
    unsafe {
        res = new_GetLine(line, hist);
    }
    res
}

pub fn get_line(gl: *mut GetLine, prompt: &str) -> String {
    let c_prompt = CString::from_slice(prompt.as_bytes());
    let mut out: *const c_char;
    let start = 0 as i8;
    let mut res: &str;
    unsafe {
        out = gl_get_line(gl, c_prompt.as_ptr(), &start, -1);
        res = from_utf8_unchecked(std::ffi::c_str_to_bytes(&out));
    }
    as_string(res).clone()
}

#[test]
fn it_works() {
}
