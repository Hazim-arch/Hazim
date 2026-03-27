use crate::println;
use core::panic::PanicInfo;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn MENTALOSBREAKDOWN(info: &PanicInfo) -> ! {
    println!("error code: 666");
    println!("       Mental OS Breakdown \n");
    println!("  a kernal panic or blue screen");
    println!("     of death has occered \n");
    println!(" you have three options:");
    println!(" give this version of the OS to");
    println!(" the devs at luneros.org \n");
    println!(" run onbored system repair\n");
    println!(" chroot into the OS\n");
    println!(" panic at: {}\n", info);
    loop {};
}