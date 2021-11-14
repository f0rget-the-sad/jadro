#![no_std]
#![no_main]

use core::panic::PanicInfo;
use jadro::{QemuExitCode, exit_qemu, serial_print};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print!("[ok]\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_print!("[test did not panic]\n");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
