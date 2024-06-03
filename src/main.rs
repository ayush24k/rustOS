#![no_std] //do not link rust std liberary
#![no_main] // disable entry point
use core::panic::PanicInfo;

// fn called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // to disable name mangling
pub extern "C" fn _start() -> ! {
    //entry point, since linker looks for a fn
    //named '_start' by default 
    loop {}
}


