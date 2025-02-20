kernel.elf: src/main.rs boot.o src/utils.rs
	cargo build --release
	cp target/riscv64gc-unknown-none-elf/release/lingangu_os kernel.elf

boot.o: src/boot.s
	riscv64-elf-as -c src/boot.s -o boot.o

sim:
	qemu-system-riscv64 -machine virt -smp 2 -bios kernel.elf -serial mon:stdio