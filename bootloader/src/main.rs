#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> ! {
    // println!("Hello, world!");
    // use uefi::prelude::ResultExt;
    use core::fmt::Write;
    system_table.stdout().clear().unwrap();
    writeln!(system_table.stdout(), "Hello bootloader").unwrap();
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}