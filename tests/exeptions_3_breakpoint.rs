#![no_std]
#![no_main]

//extern crate x86_64;
use x86_64::instructions::interrupts::int3;

use core::panic::PanicInfo;
use jadro::{QemuExitCode, exit_qemu, serial_print, interrupts};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print!("[ok]\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {

    interrupts::init();
    //trigger a breakpoint exception
    int3();
    serial_print!("[test did not panic]\n");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
