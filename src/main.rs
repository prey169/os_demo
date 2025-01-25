#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_demo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_demo::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "World");

    os_demo::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    loop {
        use os_demo::print;
        print!("-");
        for i in 0..10000 {}
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_demo::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
