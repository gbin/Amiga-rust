
pub type CStr= *const u8;

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

// 68000 Word and Longs
pub type UWord= u16;
pub type ULong= u32;

// recreated from NDK_1.3/Includes1.3/include.h/exec/nodes.h
#[repr(C)]
struct Node {
    ln_succ: *const usize, // dummy because gccrs is buggy
    ln_prec: *const usize,
    ln_type: u8,
    ln_pri: i8,
    ln_name: CStr
}

// recreated from NDK_1.3/Includes1.3/include.h/exec/libraries.h
#[repr(C)]
pub struct Library {
    lib_node: Node,
    lib_flags: u8,
    lib_pad: u8,
    lib_negsize: UWord,	    /* number of bytes before library */
    lib_possize: UWord,	    /* number of bytes after library */
    lib_version: UWord,	    /* major */
    lib_revision: UWord,	    /* minor */
    lib_idstring: CStr,	    /* ASCII identification */
    lib_sum: ULong,		    /* the checksum itself */
    lib_opencnt: UWord	    /* number of current opens */
}

extern "C" {
    fn _openlib<T>(name: CStr, version: ULong) -> &T;
    fn _closelib(lib: &Library);
    fn _call<D1T, D2T, D3T, R>(lib: *const Library, lvo:i32, d1:D1T, d2:D2T, d3:D3T) -> R;
}

impl Library {
    fn call<D1T, D2T, D3T, R>(&self, lvo:i32, d1:D1T, d2:D2T, d3:D3T) -> R {
       unsafe {_call::<D1T, D2T, D3T, R>(self, lvo, d1, d2, d3)}
    }
}

pub struct File {}
pub struct DosLibrary(Library);

const LVO_OUTPUT:i32 = -60;
const LVO_WRITE:i32 = -48;

impl DosLibrary {
    pub fn output(&self) -> &'static File {
        self.0.call::<(), (), (), &'static File>(LVO_OUTPUT, (),(),())
    }
    pub fn write(&self, output_stream: *const File, msg: &str) -> usize {
        let length =12;
        self.0.call::<*const File, CStr, usize, usize>(LVO_WRITE, output_stream,msg as *const str as CStr,length)
    }
}

fn open_library<T>(name: CStr, version: u32) -> &'static T {
    unsafe {
        // TODO: implement a panic if the result if NULL
        _openlib::<T>(name, version) as &'static T
    }
}

fn close_library(library: &Library) {
    unsafe {
        _closelib(library)
    }
}
