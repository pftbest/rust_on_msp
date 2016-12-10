# `rust_on_msp`

> Simple blinking LED example that runs on MSP430.

## Compiling

Unfortunately, it is not possible to build this project without patches
form [#38286](https://github.com/rust-lang/rust/pull/38286) and [#38240](https://github.com/rust-lang/rust/pull/38240), so we need to wait until they land in nightly.

## Board

I am using TI LaunchPad G2 board with `msp430g2553` MCU, but it is easy to port this code for any other board or MCU.

![board](https://github.com/pftbest/rust_on_msp/raw/master/board.jpg "TI LaunchPad G2")
