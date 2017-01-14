# `rust_on_msp`

> Simple blinking LED example that runs on MSP430.

## Compiling

This project can be compiled using nightly rust and [xargo](https://github.com/japaric/xargo).

Tested using version `rustc 1.16.0-nightly (4ecc85beb 2016-12-28)`

Steps:
* First, install `msp430-elf-gcc` compiler, and make sure it is in your `$PATH`.
 You can get it from [here](http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/latest/index_FDS.html). 
* ~~Install nightly rust: `$ rustup default nightly`~~
* Until issue [#38824](https://github.com/rust-lang/rust/issues/38824) is fixed, we need to use older version of nightly compiler. You can get it using this commands:
```
rustup default nightly-2016-12-29
rustup override set nightly-2016-12-29
rustup component add rust-src
```
* Install xargo: `$ cargo install xargo`
* Build the project: `$ make`
* or you can build it using xargo directly (if you don't like `make`)

  `$ xargo build --release --target msp430`
* Flash the firmware using mspdebug: `$ make prog`

## How it works

* This project is does not use default startup code from gcc, so a reset handler should be defined like this:
```rust
#[no_mangle]
#[link_section = "__interrupt_vector_reset"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;
```
  RESET_VECTOR is just a function pointer that gets placed inside a special section called
  `__interrupt_vector_reset` and is pointing to `main` function, so it will be called on reset.
  Other interrupts may be defined the same way for example `__interrupt_vector_timer0_a0`, but it
  is not possible to use interrupts yet (other than reset), because rust doesn't have a special 
  calling convention for MSP430 interrupts. A workaround would be to use `#[naked]` functions.
  You can follow the discussion here: [#38465](https://github.com/rust-lang/rust/pull/38465)

## Porting to other boards and MCUs

To run this code on the other boards and MCUs, you need to change it in few places:
* Get a linker script for your MCU from msp430-elf-gcc include directory, and place it
  inside `ldscripts` folder. (Don't forget to get `*_symbols.ld` one as well).
* Modify `.cargo/config` file so it would point to your ld script from step 1 and change
  linker flags to match your MCU name.
* Modify `build.rs` script so it would copy the right ld script from step 1.

## Board

I am using TI LaunchPad G2 board with `msp430g2553` MCU, but it is easy to port this code for any other board or MCU.

![board](https://github.com/pftbest/rust_on_msp/raw/master/board.jpg "TI LaunchPad G2")
