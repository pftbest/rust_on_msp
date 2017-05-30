# `rust_on_msp`

> Simple blinking LED example that runs on MSP430.

## Compiling

This project can be compiled using nightly rust and [xargo](https://github.com/japaric/xargo).

Tested using version `rustc 1.19.0-nightly (5de00925b 2017-05-29)`

Steps:
* First, install `msp430-elf-gcc` compiler, and make sure it is in your `$PATH`.
 You can get it from [here](http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/latest/index_FDS.html).
* Install nightly rust: `$ rustup default nightly`
* Install xargo: `$ cargo install xargo`
* Build the project: `$ make`
* or you can build it using xargo directly (if you don't like `make`)

  `$ xargo build --release --target msp430`
* Flash the firmware using mspdebug: `$ make prog`

## How it works

This project is does not use default startup code from gcc, so a reset handler should be defined like this:
```rust
#[used]
#[link_section = "__interrupt_vector_reset"]
static RESET_VECTOR: unsafe extern "msp430-interrupt" fn() = reset_handler;
```
RESET_VECTOR is just a function pointer that gets placed inside a special section
called `__interrupt_vector_reset` and is pointing to `reset_handler` function,
so it will be called on reset.

`reset_handler` function is defined in `global_asm!` block because it needs to
set stack pointer as it's first instruction during startup. Handlers for other
interrupts doesn't need this, so they can be written in rust. For example
handler for timer_a0 interrupt can be defined like this:
```Rust
#[used]
#[link_section = "__interrupt_vector_timer0_a0"]
static TIM0_VECTOR: unsafe extern "msp430-interrupt" fn() = timer0_handler;

unsafe extern "msp430-interrupt" fn timer0_handler() {
    // you can do something here
}
```

## Porting to other boards and MCUs

To run this code on the other boards and MCUs, you need to change it in few places:
* Get a linker script for your MCU from msp430-elf-gcc include directory, and place it
  inside `ldscripts` folder. (Don't forget to get `*_symbols.ld` one as well).
* Modify `.cargo/config` file so it would point to your ld script from step 1 and change
  linker flags to match your MCU name.
* Modify `build.rs` script so it would copy the right ld script from step 1.

## Board

I am using TI LaunchPad G2 board with `msp430g2553` MCU, but it should be easy to port this code for any other board or MCU.

![board](https://github.com/pftbest/rust_on_msp/raw/master/board.jpg "TI LaunchPad G2")
