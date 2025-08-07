#![no_std]
#![no_main]

mod font_render;

use uefi::{Status, helpers};

use crate::font_render::load_font;

#[uefi::entry]
fn main() -> Status {
    // Initialize UEFI services
    helpers::init().unwrap();

    // Test printing something to the UEFI console
    log::info!("Hello, UEFI World!");

    load_font("\\fonts\\ajn-tanmatsuki\\ajn-tanmatsuki.otf");

    // Stall for 30 seconds
    uefi::boot::stall(30_000_000);

    // Exit the application successfully!
    Status::SUCCESS
}
