# `rust_on_msp`

> Simple blinking LED example that runs on MSP430.

## Compiling

This project can be compiled using nightly rust and [xargo](https://github.com/japaric/xargo).

Tested using version `1.15.0-nightly (8f02c429a 2016-12-15)`.

Steps:
* First, install msp430-elf-gcc compiler, and make sure it is in your $PATH.
 You can get it from [here](http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/latest/index_FDS.html). 
* Install nightly rust

  `$ rustup default nightly`
* Install xargo

  `$ cargo install xargo`
* Build the project using

  `$ make`

* or you can build it using xargo directly (if you don't like `make`)

  `$ xargo build --release --target msp430g2553`

* Flash the firmware using `mspdebug`

  `$ make prog`

## How it works

* One of the issues you may run into is incompatibility between MSP430 and
  MSP430X. When `rustc` compiles code it does not pass `-mmcu` flag to the gcc,
  so gcc choses default ISA. For `msp430-elf-gcc` it is MSP430X, and it is not 
  what you want for `msp430g2553` MCU. To work around this, you should create 
  a *shim* compiler like [`msp-gcc.sh`](https://github.com/pftbest/rust_on_msp/blob/master/msp-gcc.sh)
  that will pass the necessary flags to gcc.

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
  calling convention for MSP430 interrupts.

## Proting to other boards and MCUs

To run this code on the other boards and MCUs, you need to change it in few places:
* Get a linker script for your MCU from msp430-elf-gcc include directory, and place it
  inside `ldscripts` folder.
* Rename `msp430g2553.json` and modify it to point to the right liker script from step 1
* Modify `msp-gcc.sh` so it would pass the right mcu name to the gcc.
* Modify `DEVICE` variable inside a Makefile.

## Board

I am using TI LaunchPad G2 board with `msp430g2553` MCU, but it is easy to port this code for any other board or MCU.

![board](https://github.com/pftbest/rust_on_msp/raw/master/board.jpg "TI LaunchPad G2")
