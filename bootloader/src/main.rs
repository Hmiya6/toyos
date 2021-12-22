#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use core::fmt::Write;
use uefi::{
    prelude::*,
    proto::{
        loaded_image::LoadedImage,
        media::fs::SimpleFileSystem,
        media::file::Directory,
    }
};
use uefi:: {
    proto::media::file::{ File, FileAttribute, FileInfo, FileMode, FileType},
    table::boot::MemoryType,
};
extern crate alloc;

type EntryFn = extern "sysv64" fn();

fn open_root(image: Handle, system_table: &SystemTable<Boot>) -> Directory {
    let loaded_image = system_table
        .boot_services()
        .handle_protocol::<LoadedImage>(image)
        .unwrap_success()
        .get();
    let device = unsafe { (*loaded_image).device() };
    let file_system = system_table
        .boot_services()
        .handle_protocol::<SimpleFileSystem>(device)
        .unwrap_success()
        .get();
    unsafe { (*file_system).open_volume().unwrap_success() }   
}


#[no_mangle]
pub extern "efiapi" fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> ! {

    unsafe {
        uefi::alloc::init(system_table.boot_services());
    }

    system_table.stdout().clear().unwrap_success();
    writeln!(system_table.stdout(), "Hello, UEFI!").unwrap();

    let mut kernel_file = {
        let mut root = open_root(image, &system_table);
        let file_handle = root
            .open("kernel.elf", FileMode::Read, FileAttribute::READ_ONLY)
            .unwrap_success();
        match file_handle.into_type().expect_success("Failed into_type") {
            FileType::Regular(file) => Some(file),
            _ => None,
        }.expect("kernel file is not regular file")
    };
    let kernel_file_size = {
        let info_buf = &mut [0u8; 4000];
        let info: &mut FileInfo = kernel_file.get_info(info_buf).unwrap_success();
        info.file_size() as usize
    };
    let kernel_file_buf = {
        let addr = system_table.boot_services()
            .allocate_pool(MemoryType::LOADER_DATA, kernel_file_size)
            .unwrap_success();
        unsafe {
            core::slice::from_raw_parts_mut(addr, kernel_file_size)
        }
    };
    let read_size = kernel_file.read(kernel_file_buf).unwrap_success();
    kernel_file.close();
    assert_eq!(read_size, kernel_file_size);

    use goblin::elf;
    let kernel_elf = elf::Elf::parse(kernel_file_buf)
        .expect("Failed parse kernel file");

    use alloc::vec::Vec;
    let pt_load_headers = kernel_elf
            .program_headers
            .iter()
            .filter(|ph| ph.p_type == elf::program_header::PT_LOAD)
            .collect::<Vec<_>>();

    let (kernel_start, kernel_end) = {
        use core::cmp;
        let (mut start, mut end) = (usize::MAX, usize::MIN);
        for &pheader in &pt_load_headers {
            start = cmp::min(start, pheader.p_vaddr as usize);
            end = cmp::max(end, (pheader.p_vaddr + pheader.p_memsz) as usize);
        }
        (start, end)
    };

    writeln!(
        system_table.stdout(), "Kernel: {:#x} - {:#x}", kernel_start, kernel_end
    ).unwrap();

    system_table.boot_services().allocate_pages(
        uefi::table::boot::AllocateType::Address(kernel_start),
        MemoryType::LOADER_DATA,
        (kernel_end - kernel_start + 0xfff) / 0x1000,
    ).unwrap_success();

    for &pheader in &pt_load_headers {
        let offset = pheader.p_offset as usize; // offset in file
        let file_size = pheader.p_filesz as usize; // LOAD segment file size
        let mem_size = pheader.p_memsz as usize; // LOAD segment memory size
        let load_dest =
            unsafe { core::slice::from_raw_parts_mut(pheader.p_vaddr as *mut u8, mem_size) };
        // maybe optimized out?
        load_dest[..file_size].copy_from_slice(&kernel_file_buf[offset..offset + file_size]);
        load_dest[file_size..].fill(0);
    }

    let entry_point = {
        let addr = kernel_elf.entry;
        unsafe { core::mem::transmute::<u64, EntryFn>(addr) }
    };

    entry_point();

    loop {}
}

use core::panic::PanicInfo;
use core::alloc::Layout;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("alloc error")
}