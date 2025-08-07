#![no_std]
#![no_main]

use log::info;
use uefi::prelude::*;

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello, UEFI World!");
    boot::stall(10_000_000);
    Status::SUCCESS
}
