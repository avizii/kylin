qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios /Users/wudj/Workspace/os2022/rCore-Tutorial-v3/bootloader/rustsbi-qemu.bin \
  -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000