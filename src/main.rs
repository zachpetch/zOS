#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(zos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zos::println;

/// This function is the entry point.
/// The linker looks for a function named `_start` by default, which is why the
/// `no_mangle` tag is used (so the compiler won't touch/"mangle" the name).
#[no_mangle]
pub extern "C" fn _start() -> ! {
//    for i in ["World", "Handsom", "Zachariah"] {
//        println!("Hello, {}!", i);
//    }

    zos::init();

    let ptr = 0x2031b2 as *mut u8;

    // read
    unsafe { let x = *ptr; }
    println!("read worked");

    // write
    unsafe { *ptr = 42; }
    println!("write worked");

    #[cfg(test)]
    test_main();

//    println!("It did not crash, which is good.");

    zos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    zos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zos::test_panic_handler(info)
}

