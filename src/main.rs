#![no_std] //do not link rust std liberary
#![no_main] // disable entry point
use core::panic::PanicInfo;

// fn called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static OUTPUT: &[u8] = b"Ayush24k's Os Built in Rust";

#[no_mangle] // to disable name mangling
pub extern "C" fn _start() -> ! {
    //entry point, since linker looks for a fn
    //named '_start' by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in OUTPUT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xD;
        }
    }
    loop {}
}
