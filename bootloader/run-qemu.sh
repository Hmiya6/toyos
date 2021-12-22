#! bin/bash

MNT=./target/mnt
DISK=./target/disk.img
OVMF_PATH=/usr/share/OVMF/x64/OVMF.fd
BOOTLOADER_EFI_PATH=$1

qemu-img create -f raw $DISK 200M
mkfs.fat -n 'OS' -s 2 -f 2 -R 32 -F 32 $DISK

mkdir -p $MNT
sudo mount -o loop $DISK $MNT

sudo mkdir -p $MNT/EFI/BOOT
sudo cp $BOOTLOADER_EFI_PATH $MNT/EFI/BOOT/BOOTX64.EFI

sleep 0.5
sudo umount $MNT

qemu-system-x86_64 -bios $OVMF_PATH -drive format=raw,file=$DISK