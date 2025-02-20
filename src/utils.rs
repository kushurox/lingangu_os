use core::arch::asm;

pub const UART: *mut u64 = 0x10000000 as *mut u64;


pub fn print(msg: &'static str) {
    for i in msg.bytes() {
        unsafe {
            UART.write_volatile(i as u64);
        }
    }
}

pub fn get_hartid() -> u64 {
    unsafe {
        let mut res: u64;
        asm!("csrr {0}, mhartid", out(reg) res);
        res
    }
}

pub fn get_time() -> u64 {  // something's wrong with this i think, will debug later
    unsafe {
        let mut res: u64;
        asm!("csrr {0}, time ", out(reg) res);
        res
    }
}

pub fn delay(ticks: u64) {
    let t1 = get_time();
    while (get_time() - t1) < ticks {
    }
}

pub fn sadge_delay(){
    for _ in 0..1000000{

    }
}