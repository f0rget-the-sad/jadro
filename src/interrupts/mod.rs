mod idt;

use lazy_static::lazy_static;
use crate::{serial_print, QemuExitCode, exit_qemu};

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt
    };
}

extern "C" fn divide_by_zero_handler() -> ! {
    serial_print!("EXCEPTION: DIVIDE BY ZERO\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn init() {
    IDT.load();
    serial_print!("IDT LOADED\n");
}
