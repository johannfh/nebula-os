#![no_std]
#![no_main]

use fontdue::Font;
use uefi::{Status, boot::get_handle_for_protocol, helpers, proto::console::text::Output};

use crate::utils::read_file_from_esp;

#[macro_use]
extern crate alloc;

mod utils;

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
    log::info!(
        "Successfully read font file from ESP, size: {} bytes",
        otf_data.len()
    );

    let font = Font::from_bytes(otf_data.as_slice(), fontdue::FontSettings::default())
        .expect("Failed to parse font data");
    log::info!(
        "Font loaded successfully! {}",
        font.name().expect("Font has no name")
    );

    // Stall for 10 seconds
    uefi::boot::stall(10_000_000);
    log::info!("Shutting down...");
    uefi::boot::stall(1_000_000);

    // Exit the application successfully!
    Status::SUCCESS
}
