#! bin/bash

MNT=./mnt
DISK=./disk.img
TARGET=$1

# make_image 部分
qemu-img create -f raw $DISK 200M
mkfs.fat -n 'OS' -s 2 -f 2 -R 32 -F 32 $DISK

## mount_image 部分
mkdir -p $MNT
sudo mount -o loop $DISK $MNT

sudo mkdir -p $MNT/EFI/BOOT
sudo cp $TARGET $MNT/EFI/BOOT/BOOTX64.EFI

sleep 0.5
sudo umount $MNT

# run_image 部分
qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=$DISK # -s