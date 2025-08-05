#![no_std]
#![no_main]

use log::info;
use uefi::prelude::*;

fn setup_uefi() {
    uefi::helpers::init().unwrap();
}

#[uefi::entry]
fn main() -> Status {
    info!("Hello, UEFI World!");
    boot::stall(10_000_000);
    
    Status::SUCCESS
}
