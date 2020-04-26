#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ggos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ggos::util::bit_array::BitArray;
use ggos::{serial_print, serial_println};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use ggos::allocator;
    use ggos::memory::{self, BoolInfoFrameAllocator};
    use x86_64::VirtAddr;

    ggos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BoolInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization faield");

    test_main();
    loop {}
}

#[test_case]
pub fn test_bit_array() {
    serial_print!("test_bit_array...");
    let mut ba = BitArray::new(500);

    assert_eq!(ba.get(138), 0);
    ba.set_on(138);
    assert_eq!(ba.get(138), 1);
    assert_eq!(ba.get(137), 0);
    serial_println!("[OK]");
}

// #[test_case]
// pub fn test_bit_array_error() {
//     serial_print!("test_bit_array_error[should be failed]...");
//     let mut ba = BitArray::new(500);
//     assert_eq!(ba.get(138), 0);
//     ba.get(500);
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ggos::test_panic_handler(info)
}
