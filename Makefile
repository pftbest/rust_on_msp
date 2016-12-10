TARGET = target/msp430g2553/release/msp

all:
	RUSTFLAGS="--sysroot sysroot" cargo build --release --target=msp430g2553
	msp430-elf-objdump -Cd $(TARGET) > $(TARGET).lst
	msp430-elf-size $(TARGET)

clean:
	cargo clean

prog:
	mspdebug rf2500 "prog $(TARGET)"
