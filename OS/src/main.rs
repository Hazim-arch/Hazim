#![no_std]
#![no_main]

#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::testframwork::test_runner)]
#![feature(custom_test_frameworks)]

#![allow(unused_imports)]
extern crate volatile;
extern crate spin;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

use crate::vga_buffer::Writer;
mod testframwork;
mod vga_buffer;
mod luner_abi;
mod lelite;

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    unsafe { luner_abi::check_luner_abi(); }

    #[cfg(test)]
    testframwork::test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        vga_buffer::WRITER.force_unlock();
    }

    // Switch to "Emergency Red"
    vga_buffer::WRITER.lock().color_code = vga_buffer::ColorCode::new(
        vga_buffer::Color::White, 
        vga_buffer::Color::Red
    );


    println!("\n[ MENTAL OS BREAKDOWN ]");
    println!("------------------------");
    println!("{}", info);
    println!("------------------------");
    println!("LunerOS has given up on life or did it.");
    println!("you have three options.");
    println!("options:");
    println!("  1: give this version of LunerOS to the devs at LunerOS.org with the user folder removerd");
    println!("  2: reinstall the OS");
    println!("  3: edit the sorce code of the kernal");

    loop {}
}