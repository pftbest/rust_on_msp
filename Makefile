DEVICE = msp430g2553
TARGET = target/$(DEVICE)/release/msp

all:
	xargo build --release --target $(DEVICE)
	msp430-elf-objdump -Cd $(TARGET) > $(TARGET).lst
	msp430-elf-size $(TARGET)

clean:
	cargo clean

prog:
	mspdebug rf2500 "prog $(TARGET)"
