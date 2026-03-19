use crate::println;
use core::panic::PanicInfo;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn MENTALOSBREAKDOWN(info: &PanicInfo) -> ! {
    println!("╔════════════════════════════════╗");
    println!("║      Mental OS Breakdown       ║");
    println!("╠════════════════════════════════╣");
    println!("║ a kernal panic or blue screen  ║");
    println!("║     of death has occered       ║");
    println!("║ you have three options:        ║");
    println!("╠════════════════════════════════╣");
    println!("║ give this version of the OS to ║");
    println!("║ the devs at luneros.org        ║");
    println!("║ run onbored system repair      ║");
    println!("║ chroot into the OS             ║");
    println!("║ panic at: {}                   ║", info);
    println!("╚════════════════════════════════╝");
    loop {};
}