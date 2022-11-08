#![feature(intrinsics)]

mod amiga;
mod string;

use amiga::*;

// gccrs doesn't support macro imports so you need to dup them here.
// expose the concat builtin macro
#[rustc_builtin_macro]
macro_rules! concat {
    () => {{}};
}

// helper to create c strings (zero terminated) from rust str
macro_rules! cstr{
    ($a:expr)=>{
        {
            let s = concat!($a, "\0");
            s as *const str as CStr
        }
    }
}
#[no_mangle]
pub fn main() {
    let name = {cstr!("dos.library")};
    let lib = open_library::<DosLibrary>(name, 0);
    let stdout = lib.output();
    lib.write(stdout, "Hello World!");
}