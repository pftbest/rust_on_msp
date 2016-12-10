# `rust_on_msp`

> Simple blinking LED example that runs on MSP430.

## Compiling

Unfortunately, it is not possible to build this project without patches
form [#38286](https://github.com/rust-lang/rust/pull/38286) and [#38240](https://github.com/rust-lang/rust/pull/38240), so we need to wait until they land in nightly.

## How it works

* First issue you may run into is incompatibility between MSP430 and MSP430X. When `rustc` compiles code it does not pass `-mmcu` flag to the gcc, so gcc choses default ISA. For `msp430-elf-gcc` it is MSP430X, and it is not what you want for `msp430g2553` MCU. To work around this, you should create a *shim* compiler like [`msp-gcc.sh`](https://github.com/pftbest/rust_on_msp/blob/master/msp-gcc.sh) that will pass the necessary flags to gcc.

* Second issue is that rust compiler will pass the `-nodefaultlibs` flag to gcc by default, and for some reason (unknown to me) gcc will fail to emit startup code with this flag present. To fix this you should set `"no-default-libraries": false` option in json file for your target.

* Third issue, well it is not an issue, just an inconvinience. I was using a custom rust compiler with patches mentioned above, and found out that `xargo` does not support custom compilers. So I had two options: either add `libcore` crate sources as a dependency in my `Cargo.toml`, or to compile `libcore` separately and put its binary into `sysroot`. I chose the latter case, because it allows me to use other crates like `volatile-register` without changing their source code.

## Board

I am using TI LaunchPad G2 board with `msp430g2553` MCU, but it is easy to port this code for any other board or MCU.

![board](https://github.com/pftbest/rust_on_msp/raw/master/board.jpg "TI LaunchPad G2")
