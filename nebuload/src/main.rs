#![no_std]
#![no_main]

mod fs;

use uefi::{Status, boot::get_handle_for_protocol, helpers, proto::console::text::Output};

use crate::fs::read_file_from_esp;

#[macro_use]
extern crate alloc;

#[uefi::entry]
fn main() -> Status {
    // Initialize UEFI services
    helpers::init().unwrap();

    let stdout_handle =
        get_handle_for_protocol::<Output>().expect("Failed to get handle for Output protocol");
    let mut stdout = uefi::boot::open_protocol_exclusive::<Output>(stdout_handle)
        .expect("Failed to open Output protocol");

    stdout.clear().expect("Failed to clear console");

    // Test printing something to the UEFI console
    log::info!("Hello, UEFI World!");

    let otf_data = read_file_from_esp("\\fonts\\ajn-tanmatsuki\\ajn-tanmatsuki.otf");
    assert!(!otf_data.is_empty(), "Font file is empty");



    // Stall for 30 seconds
    uefi::boot::stall(30_000_000);

    // Exit the application successfully!
    Status::SUCCESS
}
