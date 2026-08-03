#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(luner_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;
use luner_os::{println, memory, memory::allocs::allocator};

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    luner_os::init(); // Load GDT/IDT
    
    // TESTS NEED THE HEAP TOO!
    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::PT::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::allocs::framalloc::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap init failed in tests");

    test_main();
    luner_os::hlt_loop();
}

#[allow(dead_code)]
fn test_runner(_tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    luner_os::testing::test_panic_handler(info)
}