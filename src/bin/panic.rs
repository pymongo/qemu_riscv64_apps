#![no_std]
#![no_main]

// _start function is not expected to return because there is no concept of program termination or returning to an operating system.
// bare-metal or embedded system where no operating system or runtime is available
#[no_mangle]
fn _start() -> ! {
    // import panic_handler
    extern crate qemu_riscv64_apps;

    panic!("oh no");
}
