#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(zos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zos::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

/// Main entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use zos::memory::BootInfoFrameAllocator;
    use zos::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

//    for i in ["World", "Handsom", "Zachariah"] {
//        println!("Hello, {}!", i);
//    }
    println!("Welcome!");
    zos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
//    let mut frame_allocator = unsafe {
//        BootInfoFrameAllocator::init(&boot_info.memory_map)
//    };

    let addresses = [
        0xb8000,            // the identity-mapped vga buffer page
        0x201008,           // some code page
        0x0100_0020_1a10,   // some stack page
        boot_info.physical_memory_offset, // virt addr mapped to phys address 0
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("The system is running (i.e. it has not crashed).");

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

