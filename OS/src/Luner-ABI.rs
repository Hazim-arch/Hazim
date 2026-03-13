use core::arch::asm;

#[naked]
pub unsafe extern "C" fn luner_function_example() {
    asm!(
        "add rax, rbx", // Perform our "Luner Math"
        "mov r15, rax", // Store result in R15 (Our custom return register)
        "ret",          // Go back to caller
        options(noreturn)
    );
}

macro_rules! luner_execute {
    ($val1:expr, $val2:expr) => {{
        let result: u64;
        unsafe {
            asm!(
                "mov rax, {0}",
                "mov rbx, {1}",
                "call luner_function_example",
                "mov {2}, r15", // Retrieve from our custom return register
                in(reg) $val1,
                in(reg) $val2,
                out(reg) result,
                out("rax") _, // Tell Rust RAX is now garbage
                out("rbx") _, // Tell Rust RBX is now garbage
            );
        }
        result
    }};
}

