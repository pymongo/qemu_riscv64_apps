#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(p: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() {
    
}