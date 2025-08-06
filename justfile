help:
    @echo "NebulaOS Justfile for building and running the Bootloader and Operating System using QEMU"
    @echo "Available commands:"
    @echo "  just clean     - Clean the Efi System Partition (ESP)"
    @echo "  just build     - Build the Nebuload EFI application and set up the ESP"
    @echo "  just run       - Run QEMU with OVMF for UEFI boot"

clean:
    @echo "Cleaning old Efi System Partition (ESP)"
    rm -rf esp

build: clean
    @echo "Setup Efi System Partition (ESP)"
    mkdir -p esp/efi/boot
    @echo "Building all EFI applications"
    cargo build --workspace --target x86_64-unknown-uefi
    @echo "Copying Nebuload (Bootloader) EFI application to ESP"
    cp target/x86_64-unknown-uefi/debug/nebuload.efi esp/efi/boot/bootx64.efi

run: build
    @echo "Running QEMU with OVMF for UEFI boot"
    qemu-system-x86_64 -enable-kvm \
        -m 2048 \
        -bios OVMF_X64.fd \
        -drive format=raw,file=fat:rw:esp

