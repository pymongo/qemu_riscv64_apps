#![no_std]
#![no_main]

// _start function is not expected to return because there is no concept of program termination or returning to an operating system.
// bare-metal or embedded system where no operating system or runtime is available
#[no_mangle]
fn _start() -> ! {
    const SBI_CONSOLE_PUTCHAR: usize = 1;
    // nothing happen
    qemu_riscv64_apps::syscall(SBI_CONSOLE_PUTCHAR, 'h' as usize, 0, 0);
    qemu_riscv64_apps::syscall(SBI_CONSOLE_PUTCHAR, '\n' as usize, 0, 0);

    qemu_riscv64_apps::print("haha\n");
    qemu_riscv64_apps::exit(0);
    panic!("unreachable");
}
