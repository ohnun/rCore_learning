#![no_main]
#![no_std]
mod lang_items;
mod sbi;

use crate::sbi::console_putchar;
use crate::sbi::shutdown;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// rust_main()
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    // test code.
    console_putchar('O' as usize);
    console_putchar('k' as usize);
    console_putchar('\n' as usize);
    shutdown();
    // loop{}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
