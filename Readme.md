# MyCore
## 生成内核
`cargo build --release`
## 裁剪并获取内核镜像
`rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os \
    -O binary target/riscv64gc-unknown-none-elf/release/os.bin`  
## 启动qemu加载镜像
`qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S`  
若无需GDB调试, 则删除最后一行`-s -S`
## 启动GDB调试工具
`riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'`  
