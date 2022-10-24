clear
qemu-system-x86_64 -drive format=raw,file=target/x86_64-target/debug/bootimage-titanium.bin -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -display none
