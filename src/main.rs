#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ggos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ggos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    ggos::init();

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
