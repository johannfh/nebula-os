#![no_std]
#![no_main]

use alloc::string::String;
use fontdue::{
    Font,
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
};
use nebula_uefi_graphics::screen::Screen;
use uefi::{
    Status,
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    helpers, print,
    proto::console::{gop::GraphicsOutput, text::Output},
};

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
    let mut stdout =
        open_protocol_exclusive::<Output>(stdout_handle).expect("Failed to open Output protocol");

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

    log::info!("Switching to custom rendering...");
    uefi::boot::stall(1_000_000);

    let gop_handle = get_handle_for_protocol::<GraphicsOutput>()
        .expect("Failed to get handle for GraphicsOutput protocol");

    let mut gop = open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .expect("Failed to open GraphicsOutput protocol");

    let (width, height) = gop.current_mode_info().resolution();

    let mut screen = Screen::new(width, height);
    screen.blit(&mut gop).expect("Failed to blit screen");

    let text = "Hello, UEFI World!";

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

    layout.reset(&LayoutSettings {
        line_height: 32.0,
        ..LayoutSettings::default()
    });

    layout.append(&[font.clone()], &TextStyle::new(text, 32.0, 0));

    for (i, char) in text.chars().enumerate() {
        let (metrics, bitmap) = font.rasterize(char, 32.0);
        screen
            .draw_char(
                &mut gop,
                bitmap,
                (i * 32 + metrics.xmin as usize, (50 + metrics.ymin) as usize),
                (metrics.width, metrics.height),
            )
            .expect("Failed to draw character");
        log::info!(
            "Rendered character '{}' at position ({}, 0)\n",
            char,
            i * metrics.width
        );
    }

    screen.blit(&mut gop).expect("Failed to blit screen");

    // Stall for 10 seconds
    uefi::boot::stall(30_000_000);
    log::info!("Shutting down...");
    uefi::boot::stall(1_000_000);

    // Exit the application successfully!
    Status::SUCCESS
}
