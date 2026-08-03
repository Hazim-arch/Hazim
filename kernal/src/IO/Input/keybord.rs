use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use crate::task::keyboard::add_scancode;
use crate::exeptions::PIC::PICS;
use crate::exeptions::IDT::InterruptIndex;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame,
) {
    // Port 0x60 is the PS/2 keyboard controller data port
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    // Send the key code to your async task queue!
    add_scancode(scancode);

    // Tell the PIC we processed the keyboard interrupt
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}