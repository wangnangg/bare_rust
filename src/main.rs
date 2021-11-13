#![no_std]
#![no_main]
#![feature(global_asm)]

mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn not_main() {
    let mut x = 0;
    loop {
        x = x + 1;
    }
}
