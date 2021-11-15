#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jadro::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use jadro::{print, serial_print, interrupts, exit_qemu, QemuExitCode};

/// Setup panic handlers
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jadro::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    print!("Wake UP, {}\n", "NEO!");

    interrupts::init();

    serial_print!("fuck!\n");
    //#[cfg(test)]
    //test_main();
    exit_qemu(QemuExitCode::Success);

    loop {}
}

