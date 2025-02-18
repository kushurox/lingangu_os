#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

const UART: *mut u64 = 0x10000000 as *mut u64;

#[panic_handler]
fn oh_no(_info: &PanicInfo) -> !{

    loop {}
}

#[unsafe(link_section = ".entry")]
#[unsafe(no_mangle)]
fn kmain() -> !{
    print("kruthart noob");
    loop {}
}

fn print(msg: &'static str) {
    for i in msg.bytes() {
        unsafe {
            UART.write_volatile(i as u64);
        }
    }
}
