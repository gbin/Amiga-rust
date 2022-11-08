#!/bin/zsh
set -e
m68k-elf-gcc -g -O2 -fPIC -march=68000 -c src/ks13.c -o src/ks13.o
m68k-elf-gccrs -g -O2 -fpie -march=68000 -ffunction-sections -fdata-sections --entry main -Wl,--script=link.script -nostdlib src/main.rs src/ks13.o -o hello


## Dump some details about the generated elf
readelf -a hello
m68k-elf-objdump -d hello
readelf -x .got hello
readelf -x .rodata hello
objdump -f hello | grep "start address"

#objcopy -O binary --only-section=.text hello hello.text
#objcopy -O binary --only-section=.data hello hello.data
gcc elf2hunk/elf2hunk.c -o elf2hunk/elf2hunk
# elf2hunk/elf2hunk -v hello hello.amiga
# Check if this is a correctly formatted HUNK amiga format
# hunktool -A -x info hello.amiga
