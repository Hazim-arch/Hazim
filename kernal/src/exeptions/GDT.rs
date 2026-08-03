use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;
use crate::exeptions::TSS::TSS;
use crate::serial_println;

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    
    serial_println!("GDT: Loading table...");
    GDT.0.load();
    unsafe {
        serial_println!("GDT: Reloading CS...");
        CS::set_reg(GDT.1.code_selector);
        serial_println!("GDT: Loading TSS...");
        load_tss(GDT.1.tss_selector);
        serial_println!("GDT: Success!");
    }
}