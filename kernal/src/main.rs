#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![allow(unused_imports)]
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use core::panic::{PanicInfo, PanicMessage};
use luner_os::{error_logs, println, qemu, serial_println, testing::Testable, hlt_loop};
use luner_os::memory::{PT::init, allocs::framalloc::{EmptyFrameAllocator, BootInfoFrameAllocator}};
use luner_os::memory::{mappr::create_example_mapping, allocs::allocator};
use luner_os::task::{Task, executor::Executor, keyboard, scheduler, threed::Thread, state::MultitaskingState};
use x86_64::{structures::paging::{PageTable, Page}, VirtAddr};
use bootloader::{BootInfo, entry_point};

extern crate alloc;

/// Thread 1: Fully Preemptive
/// The timer interrupt will forcibly context-switch this thread every tick.
extern "C" fn preemptive_thread() -> ! {
    let mut count = 0;
    loop {
        count += 1;
        if count % 100_000 == 0 {
            println!("[Preemptive Thread] Running... count: {}", count);
        }
    }
}

/// Thread 2: User Cooperative (Hybrid)
/// Runs for a bit and voluntarily yields, but if it runs past 100ms, the timer forces a switch.
extern "C" fn cooperative_thread() -> ! {
    let mut count = 0;
    loop {
        count += 1;
        if count % 100_000 == 0 {
            println!("[UserCoop Thread] Running... yielding!");
            scheduler::yield_now(); // Voluntary yield
        }
    }
}

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! { 
    // Initialization
    luner_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { luner_os::memory::PT::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { 
        BootInfoFrameAllocator::init(&boot_info.memory_map) 
    };
    luner_os::memory::allocs::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    scheduler::init();

    // Spawning Test Threads with different Multitasking States
    let thread1 = Thread::new(preemptive_thread, MultitaskingState::Preemptive);
    let thread2 = Thread::new(
        cooperative_thread,
        MultitaskingState::UserCooperative {
            ticks_used: 0,
            max_ticks: 10, // 10 ticks = ~100ms at 100Hz
        },
    );

    scheduler::spawn_thread(thread1);
    scheduler::spawn_thread(thread2);

    println!("Scheduler initialized. Starting preemption test...\n");

    // Kick off the first thread!
    scheduler::schedule_next_task();

    hlt_loop();
}

#[allow(unreachable_code)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(test)]
    {
        serial_println!("[test failed]");
        serial_println!("Error: {}\n", info);
        luner_os::qemu::exit_qemu(luner_os::qemu::QemuExitCode::Success);
    }

    #[cfg(not(test))]
    {
        error_logs::MENTALOSBREAKDOWN(info);
    }

    hlt_loop();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    crate::qemu::exit_qemu(crate::qemu::QemuExitCode::Success);
}
