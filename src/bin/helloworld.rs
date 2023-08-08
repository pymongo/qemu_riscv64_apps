#![no_std]

#[panic_handler]
fn foo(p: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn main() {
    
}