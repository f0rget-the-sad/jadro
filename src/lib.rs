#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;

use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_print!("[ok]\n");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_print!("[failed]\n");
    serial_print!("{}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    // the iobase of the isa-debug-exit device
    const ISA_DEBUG_EXIT_IOBASE : u16 = 0xf4;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_IOBASE);
        port.write(code as u32);
    }
}

pub fn divide_by_zero() {
    unsafe {
        asm!(
            "mov eax, 0x69",
            "mov ecx, 0x0",
            "div ecx",
        );
    }
}
