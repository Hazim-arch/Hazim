use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use crate::{println, serial_println, hlt_loop};
use crate::exeptions::{PIC::{PIC_1_OFFSET, PICS}, TSS};
use crate::IO::Input::keybord::keyboard_interrupt_handler;
use crate::task::{state::MultitaskingState, scheduler::{get_current_running_task, schedule_next_task}};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, Keyboard, 
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
               .set_stack_index(TSS::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}


pub fn init_idt() {
    IDT.load();
}

// exeprion handlers:
extern "x86-interrupt" fn breakpoint_handler(
stack_frame: InterruptStackFrame)
{
    serial_println!("the breakpoint worked");
    println!("An exeption has occered");
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    serial_println!("or did it");
}

extern "x86-interrupt" fn double_fault_handler( 
stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    println!("A fatal exeption has occered");
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    if let Some(current_task) = get_current_running_task() {
        // Lock the Mutex to safely mutate the state inside Arc<Thread>
        let mut ms_guard = current_task.ms.lock();

        match &mut *ms_guard {
            MultitaskingState::Preemptive => {
                drop(ms_guard); // Always drop the lock before switching context!
                schedule_next_task();
            }
            MultitaskingState::UserCooperative { ticks_used, max_ticks } => {
                *ticks_used += 1;
                if *ticks_used >= *max_ticks {
                    *ticks_used = 0;
                    drop(ms_guard); // Drop lock before context switch!
                    schedule_next_task();
                }
            }
            MultitaskingState::Cooperative | MultitaskingState::Blocked => {}
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}
