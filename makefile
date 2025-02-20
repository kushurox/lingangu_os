kernel.elf: src/main.rs src/boot.s src/utils.rs
	cargo build
	cp target/riscv64gc-unknown-none-elf/debug/lingangu_os kernel.elf

boot.o: src/boot.s
	riscv64-elf-as -c src/boot.s -o boot.o

sim:
	qemu-system-riscv64 -machine virt -smp 2 -bios none -kernel kernel.elf -serial mon:stdio