#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use log::info;
use uefi::{
    Handle, Result as UefiResult, Status,
    boot::{self, get_handle_for_protocol, open_protocol_exclusive, stall},
    helpers,
    proto::console::{
        gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
        text::Input,
    },
};

struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        let pixels = vec![BltPixel::new(0, 0, 0); width * height];
        Buffer {
            width,
            height,
            pixels,
        }
    }

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    fn blit(&mut self, gop: &mut GraphicsOutput) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    fn blit_pixel(&mut self, gop: &mut GraphicsOutput, coords: (usize, usize)) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::SubRectangle {
                coords: coords,
                px_stride: self.width,
            },
            dest: coords,
            dims: (1, 1),
        })
    }
}

#[uefi::entry]
fn main() -> Status {
    // Initialize UEFI services
    helpers::init().unwrap();

    // Test printing something to the UEFI console
    info!("Hello, UEFI World!");

    let stdin_handle: Handle =
        get_handle_for_protocol::<Input>().expect("Failed to get handle for Input protocol");

    let mut stdin =
        open_protocol_exclusive::<Input>(stdin_handle).expect("Failed to open Input protocol");

    let gop_handle: Handle = get_handle_for_protocol::<GraphicsOutput>()
        .expect("Failed to get handle for Graphics Output protocol");

    let mut gop = open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .expect("Failed to open Graphics Output protocol");

    // Stall for 10 seconds
    stall(10_000_000);

    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width as usize, height as usize);
    buffer
        .blit(&mut gop)
        .expect("Failed to blit buffer to video");

    let tl_pixel = buffer.pixel(0, 0).expect("Failed to get top-left pixel");
    tl_pixel.red = 255; // Set red channel to 255
    buffer
        .blit_pixel(&mut gop, (0, 0))
        .expect("Failed to blit pixel at (0, 0)");

    // Stall for 10 seconds
    stall(10_000_000);

    let mut keys_remaining = 5;
    info!("Reading {} key events...", keys_remaining);
    while keys_remaining > 0 {
        let input = stdin.read_key().expect("Failed to read key");
        if let Some(key) = input {
            info!("Key pressed: {:?}", key);
            keys_remaining -= 1;
            info!("Reading {} more key event(s)...", keys_remaining);
        }
    }

    // create a key event to wait for
    let key_event = stdin
        .wait_for_key_event()
        .expect("Failed to create key event");

    info!("Waiting for a key event...");

    // here we get a `usize` but I dont know what it means yet
    let _ = boot::wait_for_event(&mut [key_event])
        .expect("Error occured in waiting for an input event");

    info!("Key event received!!! WE ARE WINNING BOYS");

    // Stall for 30 seconds
    info!("Waiting for 30 seconds before exiting...");
    uefi::boot::stall(30_000_000);

    // Exit the application successfully!
    Status::SUCCESS
}
