#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> ! {
    // println!("Hello, world!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}