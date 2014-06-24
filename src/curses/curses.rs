#![allow(dead_code)]
#![feature(link_args)]

extern crate libc;

use libc::Null;
use libc::{c_int, c_void, c_char, STDIN_FILENO, STDOUT_FILENO, FILE, Nullable};
use std::ptr;

pub mod ffi {
    use libc::{c_int, c_void, c_char, FILE};

    #[link(name = "curses")]
    extern {
        pub fn getch() -> c_int;
        pub fn wgetch(win: *mut c_void) -> c_int;
        pub fn newterm(ty: *c_char, outfd: *FILE, infd: *FILE) -> *mut c_void;
        pub fn set_term(screen: *mut c_void) -> *mut c_void;
        pub fn initscr() -> *mut c_void;
        pub fn endwin() -> c_int;
        pub fn isendwin() -> c_int;
        pub fn delscreen(sp: *mut c_void);
    }
}

/// A new curses terminal instance.
pub struct Term {
    screen: Nullable<c_void>
}

impl Term {

    /// Initializes a new curses terminal with the default out/in file
    /// descriptors set to stdout and stdin, respectively. Also sets the type
    /// to NULL.
    pub fn new() -> Term {
        let stdin: *FILE = unsafe {
            libc::fopen("/dev/stdin".to_c_str().unwrap() as *c_char, "r+".to_c_str().unwrap() as *c_char)
        };
        let stdout: *FILE = unsafe {
            libc::fopen("/dev/stdout".to_c_str().unwrap() as *c_char, "r+".to_c_str().unwrap() as *c_char)
        };

        let ty: *c_char = ptr::null();
        unsafe { ffi::newterm(ty, stdin, stdout) };
        Term {
            screen: Null
        }
    }
}
