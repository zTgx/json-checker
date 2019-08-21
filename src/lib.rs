pub mod ffi;

pub struct JsonChecker {
    pub checker: *mut ffi::JsonCheckerRaw,
}
impl JsonChecker {
    pub fn new(depth: i32) -> Self {
        JsonChecker {
            checker: unsafe { ffi::new_JSON_checker(depth) },
        }
    }

    pub fn check_char(&mut self, next_char: i32) -> i32 {
        unsafe {
            ffi::JSON_checker_char(self.checker, next_char)
        }
    }

    pub fn done(&mut self) -> i32 {
        unsafe {
            ffi::JSON_checker_done(self.checker)
        }
    }
}
