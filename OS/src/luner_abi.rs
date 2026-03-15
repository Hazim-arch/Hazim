use core::arch::naked_asm;

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn luner_function_example() {
    naked_asm!(
        "add rax, rbx", 
        "mov r15, rax", 
        "ret"
    );
}

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn check_luner_abi() {
    naked_asm!(
        "mov rax, rdi",
        "add rax, 0x1337",
        "ret"
    );
}
