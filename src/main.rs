#![no_std]
#![no_main]
//#![cfg(not(test))]
use core::panic::PanicInfo;

static HELLO: &[u8]=b"Hello World!";

mod vga_buffer;
#[no_mangle]
pub extern "C" fn _start() -> ! {
use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
///this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
