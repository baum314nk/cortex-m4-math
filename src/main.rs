#![feature(allocator_api)]
#![feature(const_float_bits_conv)]
#![feature(asm_const)]
#![no_std]
#![no_main]
#![allow(unused)]

extern crate alloc;
mod matrix;

use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::{borrow::BorrowMut, mem::size_of, ops::Add};
use cortex_m::peripheral::SYST;
use cortex_m_rt::entry;
use cortex_m_semihosting::nr::{CLOCK, ELAPSED};
use cortex_m_semihosting::{debug, hprintln};
use critical_section::Mutex;
use embedded_alloc::Heap;
use matrix::Matrix;
use panic_halt as _;
use panic_halt as _;

const FIRST_ADDRESS: *const i32 = 0x2000_0000 as *const i32;
const LAST_ADDRESS: *const i32 = 0x2001_FFFC as *const i32;
const HEAP_SIZE: usize = 0x1_0000; // 64KB , Heap can take up half of the total RAM

#[global_allocator]
static HEAP: Heap = Heap::empty();
static mut HEAP_PTRS: Mutex<(u32, u32)> = Mutex::new((0, 0));

macro_rules! println_ptr {
    ($ptr:expr) => {
        let ptr = $ptr as *const i32;
        let ptr_raw = ptr as u32;
        let value = ptr.read() as u32;
        cortex_m_semihosting::export::hstdout_fmt(format_args!(
            "Value at 0x{:04X} {:04X}: 0x{:04X} {:04X}\n",
            (ptr_raw >> 16) as u16,
            ptr_raw as u16,
            (value >> 16) as u16,
            value as u16,
        ))
    };
}
macro_rules! println_byte_ptr {
    ($ptr:expr) => {
        let ptr = $ptr as *const u8;
        let ptr_raw = ptr as u32;
        let value = ptr.read();
        cortex_m_semihosting::export::hstdout_fmt(format_args!(
            "Value at 0x{:04X} {:04X}: 0x{:02X}\n",
            (ptr_raw >> 16) as u16,
            ptr_raw as u16,
            value,
        ))
    };
}

fn get_heap_ptrs() -> (*const u8, *const u8) {
    let result = unsafe { *HEAP_PTRS.get_mut() };
    return (result.0 as *const u8, result.1 as *const u8);
}

unsafe fn init_heap(print_info: bool) {
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
    // Store heap start and end addresses in static pointers
    let raw_start_ptr = HEAP_MEM.as_ptr() as u32;
    let raw_end_ptr = HEAP_MEM.as_ptr().add(HEAP_SIZE) as u32;
    *HEAP_PTRS.get_mut() = (raw_start_ptr, raw_end_ptr);

    if !print_info {
        return;
    }
    // Print heap info
    hprintln!("### Linked-list Heap initialized ###");
    hprintln!(
        "Start: 0x{:04X} {:04X}",
        (raw_start_ptr >> 16) as u16,
        raw_start_ptr as u16
    );
    hprintln!(
        "End:   0x{:04X} {:04X}",
        (raw_end_ptr >> 16) as u16,
        raw_end_ptr as u16
    );
    hprintln!(
        "Length: {:} B / {:} KB / {:} #u32\n",
        HEAP_SIZE,
        HEAP_SIZE / 1024,
        HEAP_SIZE / 4
    );
}

fn get_sys_timer() -> SYST {
    // Set up the system timer (SysTick) to count in milliseconds
    let mut core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut syst = core_peripherals.SYST;

    // Configure the system timer to count in milliseconds
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    syst.set_reload(SYST::get_ticks_per_10ms() * 10); // Wraps every 100ms
    syst.clear_current();
    syst.enable_counter();

    // TODO: Test this delay
    for _ in 0..1_000 {
        cortex_m::asm::nop();
    }

    return syst;
}

#[entry]
unsafe fn main() -> ! {
    // // Initialize the allocator BEFORE you use it
    init_heap(true);
    let heap_start = get_heap_ptrs().0 as *const i32;

    // now the allocator is ready types like Box, Vec can be used.

    let v1 = Matrix::new([[1.0], [2.0], [3.0]]);
    let v2 = Matrix::new([[1.0], [2.0], [3.0]]).T();

    let m1 = v1.dyadic(&v2);
    hprintln!("{}\n", m1);

    let m2: Matrix = v1.transform_householder();
    hprintln!("{}\n", m2);

    hprintln!(
        "Remaining Heap space: {} KB\n",
        (HEAP.free() as f32) / 1024.0
    );

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
