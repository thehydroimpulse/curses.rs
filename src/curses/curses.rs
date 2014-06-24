#![allow(dead_code)]
extern crate libc;

#[link(name = "curses")]
mod ffi {
    use libc::{c_int, c_void, c_char};

    extern {
        fn getch() -> c_int;
        fn wgetch(win: *c_void) -> c_int;
        fn newterm(ty: *c_char, outfd: *c_void, infd: *c_void) -> *c_void;
        fn set_term(screen: *c_void) -> *c_void;
        fn initscr() -> *c_void;
        fn endwin() -> c_int;
        fn isendwin() -> c_int;
        fn delscreen(sp: *c_void);
    }
}
