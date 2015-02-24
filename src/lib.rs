#![feature(libc)]
#![feature(collections)]
#![feature(std_misc)]
extern crate libc;
use libc::{c_char, c_int, size_t};
//use libc::types::common::c95::FILE;

use std::ffi::{CString, CStr};
use std::string::as_string;
use std::str::from_utf8_unchecked;
use std::ptr;

#[repr(C)]
pub struct GetLine;
/*pub struct ExpandFile;
pub struct FileExpansion {
    exists: c_int,
    nfile: c_int,
    files: *mut char,
}
pub struct WordCompletion;
*/
// will add functions as needed
#[link(name = "tecla")]
extern {
    // fn libtecla_version(major: *mut c_int, minor: *mut c_int, micro: *mut c_int);
    fn new_GetLine(linelen: size_t, histlen: size_t) -> *mut GetLine;
    fn del_GetLine(gl: *mut GetLine) -> *mut GetLine;
    fn gl_get_line(gl: *mut GetLine,
                   prompt: *const c_char,
                   start_line: *const c_char,
                   start_pos: c_int) -> *const c_char;
    fn gl_query_char(gl: *mut GetLine, prompt: *const c_char, defchar: c_char) -> c_int;
    // fn gl_read_char(gl: *mut GetLine) -> c_int;
    // fn gl_configure_getline(gl: *mut GetLine, app_string: *const c_char,
    //                        app_file: *const c_char, user_file: *const c_char) -> c_int;
    // fn gl_bind_keyseq() -> c_int;
    // fn new_ExpandFile() -> *mut ExpandFile;
    // fn del_ExpandFile(ef: *mut ExpandFile) -> *mut ExpandFile;
    // fn ef_expand_file(ef: *mut ExpandFile,
    //                  path: *const c_char,
    //                  pathlen: c_int) -> *mut FileExpansion;
    // fn el_list_expansions(result: *mut FileExpansion,
    //                      fp: *mut FILE,
    //                      term_width: c_int) -> c_int;
    // fn ef_last_error(ef: *mut ExpandFile) -> *const c_char;
    // fn new_WordCompletion() -> *mut WordCompletion;
    // fn del_WordCompletion(cpl: *mut WordCompletion) -> WordCompletion;
    /*fn gl_completion_action(gl: *mut GetLine,
                            data: *const void,
                            match_fn: CplMatchFn,
                            list_only: c_int,
                            name: *const c_char,
                            keyseq: *const c_char) -> c_int;*/
    fn gl_save_history(gl: *mut GetLine,
                       filename: *const c_char,
                       comment: *const c_char,
                       max_lines: c_int);
    fn gl_load_history(gl: *mut GetLine,
                       filename: *const c_char,
                       comment: *const c_char);
    fn gl_ignore_signal(gl: *mut GetLine, signo: c_int);
    fn gl_erase_terminal(gl: *mut GetLine) -> c_int;
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

pub fn del_gl(gl: *mut GetLine) -> *mut GetLine {
    unsafe {
        del_GetLine(gl)
    }
}

pub fn get_line(gl: *mut GetLine, prompt: &str) -> String {
    let c_prompt = CString::new(prompt.as_bytes()).unwrap();
    let start: *const i8 = ptr::null();
    let mut res: &str;
    unsafe {
        let out = CStr::from_ptr(gl_get_line(gl, c_prompt.as_ptr(), start, -1));
        res = from_utf8_unchecked(out.to_bytes());
    }
    as_string(res).clone()
}

///Returns the character read, or 0 if unreadable
pub fn query_char(gl: *mut GetLine, prompt: &str, defchar: char) -> char {
    let c_prompt = CString::new(prompt.as_bytes()).unwrap();
    let mut res: u8;
    unsafe {
        let out = gl_query_char(gl, c_prompt.as_ptr(), defchar as c_char);
        if out > 0 {
            res = out as u8;
        } else {
            res = 0;
        }
    }
    res as char
}

pub fn save_history(gl: *mut GetLine, file: &str, comment: &str, max: usize) {
    let c_file = CString::new(file.as_bytes()).unwrap();
    let c_comment = CString::new(comment.as_bytes()).unwrap();
    let c_max = max as c_int;
    unsafe {
        gl_save_history(gl, c_file.as_ptr(), c_comment.as_ptr(), c_max)
    }
}

pub fn load_history(gl: *mut GetLine, file: &str, comment: &str) {
    let c_file = CString::new(file.as_bytes()).unwrap();
    let c_comment = CString::new(comment.as_bytes()).unwrap();
    unsafe {
        gl_load_history(gl, c_file.as_ptr(), c_comment.as_ptr())
    }
}

pub fn ignore(gl: *mut GetLine, sig: isize) {
    unsafe {
        gl_ignore_signal(gl, sig as c_int)
    }
}

pub fn clear(gl: *mut GetLine) -> i32 {
    unsafe {
        gl_erase_terminal(gl)
    }
}

/*#[test]
fn it_works() {
}*/
