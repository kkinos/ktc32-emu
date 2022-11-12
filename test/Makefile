SRCS=$(wildcard *.asm)
TARGET:=$(patsubst %.asm,%.mem,$(SRCS))

.PHONEY: all
all: $(TARGET)

%.mem: %.asm
	ktc32-asm $< -o $@

.PHONEY: clean
clean:
	rm -rf *.mem