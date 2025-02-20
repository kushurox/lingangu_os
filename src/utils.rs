use core::{arch::asm, ops::Rem, sync::atomic::{AtomicBool, Ordering}};

pub const UART: *mut u64 = 0x10000000 as *mut u64;


pub fn print(msg: &'static str) {
    // print_num(0);
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

#[allow(dead_code)]
pub fn get_time() -> u64 {  // something's wrong with this i think, will debug later
    unsafe {
        let mut res: u64;
        asm!("csrr {0}, time ", out(reg) res);
        res
    }
}

#[allow(dead_code)]
pub fn delay(ticks: u64) {
    let t1 = get_time();
    loop {
        let t2 = get_time();
        if t2 - t1 > ticks {
            return;
        }
    }
}

#[allow(dead_code)]
pub fn sadge_delay(){
    for _ in 0..1000000{

    }
}

pub fn print_num(mut num: u64){
    let mut digs = [0u64; 10];
    let mut i = 0;
    while num != 0 {
        let dig = num.rem(10);
        num = num/10;
        digs[i] = dig;
        i += 1;
    }
    for i in (0..10).rev() {
        unsafe {
            UART.write_volatile(digs[i] + 48);
        }
    }
    unsafe {UART.write_volatile(10);}
}