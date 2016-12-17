TARGET = target/msp430/release/msp

all:
	xargo build --release --target msp430
	msp430-elf-objdump -Cd $(TARGET) > $(TARGET).lst
	msp430-elf-size $(TARGET)

clean:
	cargo clean

prog:
	mspdebug rf2500 "prog $(TARGET)"
