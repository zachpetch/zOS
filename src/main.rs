#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//static HELLO: &[u8] = b"Hello, Friend!";

/// This function is the entry point.
/// The linker looks for a function named `_start` by default, which is why the
/// `no_mangle` tag is used (so the compiler won't touch/"mangle" the name).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in ["World", "Handsom", "Zachariah"] {
        println!("Hello, {}!", i);
    }

    panic!("A panic message");

//    loop {}
}

mod vga_buffer;
