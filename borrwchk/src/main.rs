#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_p: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    let _this = 01;
}
