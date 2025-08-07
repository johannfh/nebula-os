help:
    @echo -e "\e[1;94mNebulaOS Justfile for building and running the Bootloader and Operating System using QEMU\e[0m"
    @echo -e "\e[92mAvailable commands:\e[0m"
    @echo -e "  \e[95mjust\e[0m \e[93mclean\e[0m   \e[3;92m- Clean the Efi System Partition (ESP)\e[0m"
    @echo -e "  \e[95mjust\e[0m \e[93mbuild\e[0m   \e[3;92m- Build the Nebuload EFI application and set up the ESP\e[0m"
    @echo -e "  \e[95mjust\e[0m \e[93mrun\e[0m     \e[3;92m- Run QEMU with OVMF for UEFI boot\e[0m"
    @echo -e "  \e[95mjust\e[0m \e[93mtest\e[0m    \e[3;92m- Run tests for all of NebulaOS\e[0m"

clean:
    @echo -e "\e[1;94m--- Cleaning old ESP ---\e[0m"
    rm -rf esp

build: clean
    @echo -e "\e[1;94m--- Building project ---\e[0m"
    @echo -e "\e[92mCreating ESP directory structure\e[0m"
    mkdir -p esp/efi/boot
    @echo -e "\e[92mCopying Font files to ESP\e[0m"
    cp -r fonts esp/fonts
    @echo -e "\e[92mBuilding Nebuload (Bootloader) EFI application\e[0m"
    cargo build --manifest-path nebuload/Cargo.toml --target x86_64-unknown-uefi
    @echo -e "\e[92mCopying Nebuload EFI application to ESP\e[0m"
    cp target/x86_64-unknown-uefi/debug/nebuload.efi esp/efi/boot/bootx64.efi

run: build
    @echo -e "\e[1;94m--- Running QEMU with OVMF for UEFI boot ---\e[0m"
    qemu-system-x86_64 -enable-kvm \
        -m 2048 \
        -bios OVMF_X64.fd \
        -drive format=raw,file=fat:rw:esp

test:
    @echo -e "\e[1;94m--- Running tests for NebulaOS ---\e[0m"
    @echo -e "\e[92mRunning tests for core components of NebulaOS\e[0m"
    cargo groups test core
    @echo -e "\e[1;94m--- All tests passed successfully! ---\e[0m"

