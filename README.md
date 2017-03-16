# `rust_on_msp`

> Simple blinking LED example that runs on MSP430.

## Compiling

This project can be compiled using nightly rust and [xargo](https://github.com/japaric/xargo).

Tested using version `rustc 1.17.0-nightly (0aeb9c129 2017-03-15)`

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
#[no_mangle]
#[link_section = "__interrupt_vector_reset"]
pub static RESET_VECTOR: unsafe extern "msp430-interrupt" fn() = reset_handler;
```
RESET_VECTOR is just a function pointer that gets placed inside a special section
called `__interrupt_vector_reset` and is pointing to `reset_handler` function,
so it will be called on reset.

Instead of writing all this code by hand, you may want to use `define_interrupt!`
macro. It takes 3 arguments, first - some unique name, second - the name of a section for the interrupt you want (you can look up the names in your linker
script), and the third argument is a block of code that will be executed when
interrupt fires. For example:
```rust
define_interrupt!(TIM0_A0_VECTOR, "__interrupt_vector_timer0_a0", {
    // do something here
});
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
