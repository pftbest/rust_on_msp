#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(abi_msp430_interrupt)]

extern crate volatile_register;
use volatile_register::RW;

macro_rules! define_interrupt {
    ($name:ident, $section:tt, $code:block) => (
        #[no_mangle]
        #[link_section = $section]
        pub static $name: unsafe extern "msp430-interrupt" fn() = {
            unsafe extern "msp430-interrupt" fn handler() $code;
            handler
        };
    )
}

define_interrupt!(RESET_VECTOR, "__interrupt_vector_reset", {
    main();
});

define_interrupt!(TIM0_VECTOR, "__interrupt_vector_timer0_a0", {
    // do nothing
});

extern "C" {
    static mut WDTCTL: RW<u16>;
    static mut P1DIR: RW<u8>;
    static mut P1OUT: RW<u8>;
}

unsafe fn main() -> ! {
    WDTCTL.write(0x5A00 + 0x80);
    P1DIR.write(0b0100_0001);
    P1OUT.write(0x01);
    loop {
        P1OUT.modify(|x| !x);
        delay(40000);
    }
}

fn delay(mut n: u16) -> u16 {
    unsafe {
        asm! {
            "1: \n dec $0 \n jne 1b" : "+r" (n) ::: "volatile"
        }
    }
    n
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
