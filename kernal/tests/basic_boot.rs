#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(luner_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use luner_os::println;

#[unsafe(no_mangle)] 
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
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