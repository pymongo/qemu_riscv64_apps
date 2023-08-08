#![no_std]
#![feature(panic_info_message)]

/*
```
# Exit system call number
li a7, 93

# Exit status code
li a0, 0

# Perform the exit system call
ecall
```
*/
pub fn syscall(a7_syscall_id: usize, a0: usize, a1: usize, a2: usize) -> isize {
    let mut ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            // a0 function argument or return value register
            // inlateout means use reg input and later receive output
            inlateout("x10") a0 => ret,
            // a1 function argument or return value register
            in("x11") a1,
            // a2
            in("x12") a2,
            // a7
            in("x17") a7_syscall_id
        );
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn exit(exit_code: i32) {
    syscall(SYSCALL_EXIT, exit_code as usize, 0, 0);
}

// rcore lab1 use sbi::put_char to print
pub fn print<T: AsRef<[u8]>>(buf: T) {
    let buf = buf.as_ref();
    let fd = 1i32;
    syscall(
        SYSCALL_WRITE,
        fd as usize, buf.as_ptr() as usize, buf.len(),
    );
}

#[panic_handler]
fn panic_handler(p: &core::panic::PanicInfo) -> ! {
    if let Some(loc) = p.location() {
        print("panic `");
        let msg = p
            .message()
            .map(|x| x.as_str().unwrap_or_default())
            .unwrap_or_default();
        print(msg);
        print("` at ");
        print(loc.file());
        print(":");
        let mut line = loc.line();
        let mut num = [0u8; 8];
        let mut idx = num.len() - 1;
        while line != 0 {
            num[idx] = b'0' + (line % 10) as u8;
            idx -= 1;
            line /= 10;
        }
        for num in num {
            if num > 0 {
                print(&[num]);
            }
        }
        print("\n");
    } else {
        print("no panic location\n");
    };
    exit(1);
    print("unreachable\n");
    loop {}
}
