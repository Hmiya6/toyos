#! bin/bash

MNT=./target/mnt
DISK=./target/disk.img
OVMF_PATH=/usr/share/OVMF/x64/OVMF.fd
BOOTLOADER_EFI_PATH=./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
KERNEL_ELF_PATH=./target/kernel_target/debug/os.elf

# build bootloader
cd bootloader
cargo build 
cd ../

qemu-img create -f raw $DISK 200M
mkfs.fat -n 'POTATO OS' -s 2 -f 2 -R 32 -F 32 $DISK

mkdir -p $MNT
sudo mount -o loop $DISK $MNT

sudo mkdir -p $MNT/EFI/BOOT

sudo cp $BOOTLOADER_EFI_PATH $MNT/EFI/BOOT/BOOTX64.EFI
sudo cp $KERNEL_ELF_PATH $MNT/kernel.elf

sleep 0.5
sudo umount $MNT

qemu-system-x86_64 -bios $OVMF_PATH -drive format=raw,file=$DISK -s -S