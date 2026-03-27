#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![allow(unused_imports)]
use core::panic::PanicInfo;
use luner_os::serial_println;
use luner_os::qemu;
use luner_os::testing::Testable;
use luner_os::error_logs;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    crate::qemu::exit_qemu(crate::qemu::QemuExitCode::Success);
}

use luner_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Booting LunerOS{}", "!");
    
    luner_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    luner_os::hlt_loop();   
}

#[allow(unreachable_code)]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error_logs::MENTALOSBREAKDOWN(info);
    luner_os::hlt_loop();   
}