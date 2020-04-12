#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use ggos::serial_print;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use ggos::{exit_qemu, QemuExitCode, serial_println};
use x86_64::structures::idt::InterruptStackFrame;

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_falut_handler)
                .set_stack_index(ggos::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow...");
    ggos::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_resursion)]
fn stack_overflow() {
    stack_overflow();
}

extern "x86-interrupt" fn test_double_falut_handler(
    _stack_frame: &mut InterruptStackFrame,
    _err_code: u64,
) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop{};
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ggos::test_panic_handler(info)
}

