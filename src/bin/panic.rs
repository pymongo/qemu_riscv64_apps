#![no_std]
#![no_main]

#[no_mangle]
fn _start() -> ! {
    // import panic_handler
    extern crate qemu_riscv64_apps;

    panic!("oh no");
}
