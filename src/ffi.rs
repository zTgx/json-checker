#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

#[repr(C)]
pub struct JsonCheckerRaw {
    pub valid: libc::c_int,
    pub state: libc::c_int,
    pub depth: libc::c_int,
    pub top: libc::c_int,

    pub stack: *mut libc::c_void,
}

#[link(name = "jsonchecker")]
extern "C" {
    pub fn new_JSON_checker(depth: libc::c_int) -> *mut JsonCheckerRaw;
    pub fn JSON_checker_char(checker: *mut JsonCheckerRaw, next_char: libc::c_int) -> libc::c_int;
    pub fn JSON_checker_done(jc: *mut JsonCheckerRaw) -> libc::c_int;
}
