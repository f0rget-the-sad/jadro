#![no_std]
#![no_main]

use core::panic::PanicInfo;
use jadro::{QemuExitCode, exit_qemu, divide_by_zero, serial_print, interrupts};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print!("[ok]\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {

    interrupts::init();
    divide_by_zero();
    serial_print!("[test did not panic]\n");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
