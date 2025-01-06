use std::ptr;
use libheif_sys as lh;

fn main() {
    unsafe {
        lh::heif_init(ptr::null_mut());
    }

    println!("Hello, world!");
}
