#![no_std]
#![no_main]

#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::testframwork::test_runner)]
#![feature(custom_test_frameworks)]

use core::panic::PanicInfo;
use core::arch::{asm, naked_asm};
mod lelite;
mod vga_buffer;
mod testframwork;

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    // 1. Manually get the lock
    {
        let mut writer = vga_buffer::WRITER.lock();
        
        // 2. Clear everything first!
        writer.clear_screen(); 
        
        // 3. Print manually to the middle of the screen
        writer.column_position = 0;
        writer.write_string("LunerOS Ceres 0.1 Beta\n");
        writer.write_string("System Status: OK\n");
    }

    // 4. Now try the macro
    println!("Macro Test: Success!");

    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        // 1. Setup the stack
        "and rsp, -16",
        "mov rsp, {stack_top}",
        "sub rsp, 16",
          
        // 2. JUMP TO RUST (This is the missing link!)
        "jmp {main}",
            
        // 3. Fallback hang
        "2:",
        "hlt",
        "jmp 2b",
            
        stack_top = const 0x90000,
        main = sym kernel_main,
    );
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_luner_abi() {
    naked_asm!(
        "mov rax, rdi",
        "add rax, 0x1337",
        "ret"
        );
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