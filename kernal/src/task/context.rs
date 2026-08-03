use core::arch::global_asm;
use super::state;
use crate::task::threed::{Thread, ThreadId};
use spin::Mutex;

unsafe extern "C" {
    pub unsafe fn switch_context(old_rsp: *mut usize, new_rsp: *const usize);
}

global_asm!(
    r#"
    .global switch_context
    .type switch_context, @function
    switch_context:
        // 1. Save callee-saved registers of CURRENT thread onto its stack
        push rbp
        push rbx
        push r12
        push r13
        push r14
        push r15

        // 2. Save current RSP to old_rsp (*rdi = rsp)
        mov [rdi], rsp

        // 3. Load next thread's RSP from new_rsp (rsp = *rsi)
        mov rsp, [rsi]

        // 4. Restore callee-saved registers of NEXT thread from its stack
        pop r15
        pop r14
        pop r13
        pop r12
        pop rbx
        pop rbp

        // 5. Return into NEXT thread's saved RIP
        ret
    "#
);

#[repr(C, packed)]
struct InitStackFrame {
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    rbx: usize,
    rbp: usize,
    rip: usize, // Entry function address
}

impl Thread {
    pub fn new(entry_point: extern "C" fn() -> !, ms: state::MultitaskingState) -> Self {
        const STACK_SIZE: usize = 4096 * 4; // 16 KB
        let stack = alloc::vec![0u8; STACK_SIZE];
        let stack_top = stack.as_ptr() as usize + STACK_SIZE;

        // Calculate initial RSP location (leave room for InitStackFrame)
        let frame_size = core::mem::size_of::<InitStackFrame>();
        let initial_rsp = stack_top - frame_size;

        unsafe {
            let frame_ptr = initial_rsp as *mut InitStackFrame;
            frame_ptr.write(InitStackFrame {
                r15: 0,
                r14: 0,
                r13: 0,
                r12: 0,
                rbx: 0,
                rbp: 0,
                rip: entry_point as usize,
            });
        }

        core::mem::forget(stack); // Prevent Rust drop from freeing the stack

        Thread {
            id: ThreadId::new(),
            stack_pointer: initial_rsp,
            stack_bottom: stack_top - STACK_SIZE,
            stack_size: STACK_SIZE,
            ms: Mutex::new(ms),
        }
    }
}

pub fn yield_to(current: &mut Thread, next: &Thread) {
    unsafe {
        switch_context(
            &mut current.stack_pointer as *mut usize,
            &next.stack_pointer as *const usize,
        );
    }
}