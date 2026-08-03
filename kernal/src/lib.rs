#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod error_logs;
pub mod luner_abi;
pub mod exeptions; 
pub mod testing;
pub mod lelite;
pub mod task;
pub mod qemu;
pub mod memory;
#[allow(non_snake_case)]
pub mod IO;     

#[cfg(test)]
use bootloader::{entry_point, BootInfo}; 

pub fn test_runner(tests: &[&dyn testing::Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

pub fn init() {
    exeptions::GDT::init();
    exeptions::IDT::init_idt();
    unsafe { exeptions::PIC::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
entry_point!(test_kernel_main);
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(); // GDT, IDT, PIC
    
    // 1. Initialize Memory (Same as main.rs)
    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::PT::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::allocs::framalloc::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // 2. Initialize Heap (So tests can use Box/Vec)
    memory::allocs::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed in test_kernel_main");

    test_main();
    hlt_loop();
}

// needed for basic_boot.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
