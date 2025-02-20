#![no_std]
#![no_main]

mod utils;

use utils::{delay, get_hartid, get_time, print, print_num, sadge_delay, UART};

use core::{panic::PanicInfo, sync::atomic::{AtomicU64, Ordering}};

static CORE_COUNT: AtomicU64 = AtomicU64::new(0);


#[panic_handler]
fn oh_no(_info: &PanicInfo) -> !{

    loop {}
}


#[unsafe(link_section = ".entry")]
#[unsafe(no_mangle)]
extern "C" fn kmain() -> ! {
    print("Core 0\n");
    loop {print("Doing sth\n"); delay(100000);};
}

#[unsafe(no_mangle)]
extern "C" fn do_nothing() -> ! {
    delay(100000);
    print("Core 1\n");
    loop {};
}