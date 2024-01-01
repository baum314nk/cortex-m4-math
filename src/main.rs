#![no_std]
#![no_main]
#![allow(unused)]
// use matrix::Matrix;

use core::mem::size_of;

// mod matrix;
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

macro_rules! println_u32 {
    ($name:expr, $addr:expr) => {
        let addr = $addr as u32;
        let addr_high = (addr >> 16) as u16;
        let addr_low = addr as u16;
        cortex_m_semihosting::export::hstdout_fmt(format_args!(
            concat!($name, ": 0x{:04X} {:04X}\n"),
            addr_high, addr_low
        ))
    };
}

const FIRST_ADDRESS: *const i32 = 0x2000_0000 as *const i32;
const LAST_ADDRESS: *const i32 = 0x2001_FFFC as *const i32;

#[entry]
unsafe fn main() -> ! {
    let mut a: u32 = 3;
    let ptr_a = core::ptr::from_mut(&mut a);
    hprintln!("a = {}", a);
    println_u32!("ptr", ptr_a);
    // let b = 19;
    core::ptr::write(ptr_a, (ptr_a.read() as u32) << 2);
    hprintln!("a = {}", ptr_a.read());
    println_u32!("ptr", ptr_a);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
