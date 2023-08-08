#![no_std]
#![no_main]

#[no_mangle]
fn _start() -> ! {
    // import panic_handler
    extern crate qemu_riscv64_apps;

    // 2877394 Illegal instruction     (core dumped)
    unsafe { core::arch::asm!("wfi") };
    panic!("unreachable");
}
