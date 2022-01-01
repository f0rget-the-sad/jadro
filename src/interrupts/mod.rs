mod idt;

use bitflags::bitflags;
use lazy_static::lazy_static;
use crate::{serial_print, QemuExitCode, exit_qemu};

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(
                    "mov rdi, rsp",
                    "sub rsp, 8", // align the stack pointer
                    "call {}",
                    sym $name);
                asm!(
                    "add rsp, 8", // undo stack pointer alignment
                    "iretq",
                    options(noreturn)
                );
            }
        }
        wrapper
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(
                    "pop rsi", // pop error code into rsi(second arg)
                    "mov rdi, rsp",
                    "sub rsp, 8", // align the stack pointer
                    "call {}",
                    sym $name,
                    options(noreturn)
                );
            }
        }
        wrapper
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(3, handler!(breakpoint_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));
        idt
    };
}


extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) {
    serial_print!("EXCEPTION: DIVIDE BY ZERO\n{:#?}\n", stack_frame);
    exit_qemu(QemuExitCode::Success);
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) {
    serial_print!("EXCEPTION: INVALID OPCODE at {:#x}\n {:#?}\n",
        stack_frame.instruction_pointer, stack_frame);
    exit_qemu(QemuExitCode::Success);
    loop {}
}

extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    serial_print!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}\n",
        stack_frame.instruction_pointer, stack_frame);
    exit_qemu(QemuExitCode::Success);
    //loop {}
}

bitflags! {
    struct PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0;
        const CAUSED_BY_WRITE = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

extern "C" fn page_fault_handler(
    stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    use x86_64::registers::control;
    serial_print!(
        "\nEXCEPTION: PAGE FAULT while accessing {:#x}\
        \nerror code: {:?}\n{:#?}\n",
        control::Cr2::read(),
        PageFaultErrorCode::from_bits(error_code).unwrap(),
        stack_frame);
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn init() {
    IDT.load();
    serial_print!("IDT LOADED\n");
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    // error code (optional)
    // This error code only exists for exceptions such as page faults or general
    // protection faults and provides additional information.

    /// Istruction pointer and its code segment descriptor onto the stack. This
    /// tells us the address of the last executed instruction, which caused the
    /// exception.
    instruction_pointer: u64,
    code_segment:        u64,
    /// This register contains various state information of the interrupted
    /// program. For example, it indicates if interrupts were enabled and
    /// whether the last executed instruction returned zero.
    cpu_flags:           u64,
    /// stack segment descriptor (SS) and the old stack pointer (from before the
    /// alignment) onto the stack. This allows us to restore the previous stack
    /// pointer when we want to resume the interrupted progra
    stack_pointer:       u64,
    stack_segment:       u64,
    // the CPU aligns the stack pointer on a 16-byte boundary.  This allows the
    // handler function to use SSE instructions, which partly expect such an
    // alignment
}
