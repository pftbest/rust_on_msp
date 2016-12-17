#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate volatile_register;
use volatile_register::RW;

extern "C" {
    static mut WDTCTL: RW<u16>;
    static mut P1DIR: RW<u8>;
    static mut P1OUT: RW<u8>;
}

#[no_mangle]
#[link_section = "__interrupt_vector_reset"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;

pub unsafe extern "C" fn main() -> ! {
    WDTCTL.write(0x5A00 + 0x80);
    P1DIR.write(0b0100_0001);
    P1OUT.write(0x01);
    loop {
        P1OUT.modify(|x| !x);
        delay(40000);
    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn delay(mut n: u16) {
    unsafe {
        asm! {
            "1: \n dec $0 \n jne 1b" : "+r" (n) ::: "volatile"
        }
    }
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
