#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ggos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ggos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ggos::memory;
    use ggos::memory::BoolInfoFrameAllocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    ggos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let _mapper = unsafe { memory::init(phys_mem_offset) };
    let _frame_allocator = unsafe { BoolInfoFrameAllocator::init(&boot_info.memory_map) };

    let _x = Box::new(41);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ggos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ggos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ggos::test_panic_handler(info);
}
