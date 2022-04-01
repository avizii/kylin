python3 build.py

#rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/00power_3 -O binary target/riscv64gc-unknown-none-elf/release/00power_3.bin

#rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/01power_5 -O binary target/riscv64gc-unknown-none-elf/release/01power_5.bin

#rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/02power_7 -O binary target/riscv64gc-unknown-none-elf/release/02power_7.bin

#rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/03sleep -O binary target/riscv64gc-unknown-none-elf/release/03sleep.bin

rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/00write_a -O binary target/riscv64gc-unknown-none-elf/release/00write_a.bin

rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/01write_b -O binary target/riscv64gc-unknown-none-elf/release/01write_b.bin

rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/02write_c -O binary target/riscv64gc-unknown-none-elf/release/02write_c.bin
