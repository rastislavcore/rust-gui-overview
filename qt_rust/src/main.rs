use std::{
    ffi::CString,
    env,
    os::raw::{c_char, c_int},
};

mod interface;
mod implementation;

#[link(name = "qgrep", kind="static")]
extern "C" {
    fn main_cpp(app: *const c_char) -> c_int;
}

fn main() {
    let mut args = env::args();
    let app = CString::new(args.next().unwrap()).unwrap();
    unsafe {
        main_cpp(app.as_ptr());
    }
}
