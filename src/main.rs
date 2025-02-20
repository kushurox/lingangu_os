#![no_std]
#![no_main]

mod utils;

use utils::{get_hartid, print, sadge_delay, UART};

use core::panic::PanicInfo;


#[panic_handler]
fn oh_no(_info: &PanicInfo) -> !{

    loop {}
}


#[unsafe(link_section = ".entry")]
#[unsafe(no_mangle)]
extern "C" fn kmain() -> !{

    let id = get_hartid();
    if get_hartid() == 1 {
        sadge_delay();
        print("\nCore 1 exists!");
    } else {
        print("Other Core exists!");
    }

    loop {}
}