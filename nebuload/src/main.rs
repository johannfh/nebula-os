#![no_std]
#![no_main]

use log::{error, info};
use uefi::{
    Handle as UefiHandle,
    boot::{self, get_handle_for_protocol, open_protocol_exclusive},
    helpers,
    proto::console::text::Input as InputProtocol,
};

#[uefi::entry]
fn main() -> uefi::Status {
    // Initialize UEFI services
    helpers::init().unwrap();

    // Test printing something to the UEFI console
    info!("Hello, UEFI World!");

    let handle: UefiHandle = match get_handle_for_protocol::<InputProtocol>() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to get handle for Input protocol: {}", e);
            return e.status();
        }
    };

    let mut stdin = match open_protocol_exclusive::<InputProtocol>(handle) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to open Input protocol exclusively: {}", e);
            return e.status();
        }
    };

    // create a key event to wait for
    let key_event = stdin
        .wait_for_key_event()
        .expect("Failed to create key event");

    for i in 0..5 {
        info!("Reading {} more key event(s)...", 5 - i);
        let input = stdin.read_key().expect("Failed to read key");
        if let Some(key) = input {
            info!("Key pressed: {:?}", key);
        }
    }

    info!("Waiting for a key event...");

    // here we get a `usize` but I dont know what it means yet
    let _ = boot::wait_for_event(&mut [key_event])
        .expect("Error occured in waiting for an input event");

    info!("Key event received!!! WE ARE WINNING BOYS");

    // Stall for 30 seconds
    info!("Waiting for 30 seconds before exiting...");
    uefi::boot::stall(30_000_000);

    // Exit the application successfully!
    uefi::Status::SUCCESS
}
