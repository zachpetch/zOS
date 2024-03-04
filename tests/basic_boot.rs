#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zos::println;

#[no_mangle]    // don't mangle the name of this function, you compiler you!
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_prinln output");
}

