#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

extern crate cc;

fn main() {
    cc::Build::new()
        .file("depend/JSON-c/JSON_checker.c")
        .include("depend/JSON-c")
        .flag("-Wno-sign-compare") //warning: comparison of integers of different signs: 'int' and 'unsigned int'
        .cpp_link_stdlib("c++") //clang: warning: treating 'c' input as 'c++' when in C++ mode, this behavior is deprecated [-Wdeprecated]
        .compile("libjsonchecker.a");
}
