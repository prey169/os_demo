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

    let ptr = 0x2046a1 as *mut u8;

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {level_4_page_table:?}");

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    os_demo::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_demo::hlt_loop();
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
