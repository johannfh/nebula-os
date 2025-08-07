use ttf_parser::Face;
use uefi::{
    CStr16,
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    proto::media::{
        file::{File, FileAttribute, FileInfo, FileMode, RegularFile},
        fs::SimpleFileSystem,
    },
};

use alloc::vec::Vec;

/// Reads a font file from the ESP (EFI System Partition) and returns its contents in a buffer.
/// # Arguments
/// * `filepath` - The path to the font file in the ESP, e.g., `\\data\\some.cfg`.
/// # Returns
/// A `Vec<u8>` containing the contents of the font file.
/// # Panics
/// All the time...
pub fn read_file_from_esp(filepath: &str) -> Vec<u8> {
    // Convert the filepath to a CStr16 format
    assert!(
        filepath.len() < 256,
        "Path length exceeds maximum allowed length"
    );
    let mut cstr_buffer = [0u16; 256];
    let filepath = CStr16::from_str_with_buf(filepath, &mut cstr_buffer)
        .expect("Failed to convert filepath to CStr16");

    // Load the font file from the specified path
    let sfs_handle = get_handle_for_protocol::<SimpleFileSystem>()
        .expect("Failed to get handle for SimpleFileSystem protocol");

    let mut sfs_proto = open_protocol_exclusive::<SimpleFileSystem>(sfs_handle)
        .expect("Failed to open SimpleFileSystem protocol");

    let mut root = sfs_proto.open_volume().expect("Failed to open ESP volume");

    let file_handle = root
        .open(filepath, FileMode::Read, FileAttribute::empty())
        .expect("Failed to open font file");

    if file_handle
        .is_directory()
        .expect("File closed or deleted during operation")
    {
        log::error!(
            "Expected a file, but found a directory at the specified path: {}",
            filepath
        );
    }
    let mut file = unsafe { RegularFile::new(file_handle) };

    let mut file_info_buf = [0u8; 1024];

    let file_info = file
        .get_info::<FileInfo>(&mut file_info_buf)
        .expect("Failed to get file info");

    let mut file_buffer = vec![0u8; file_info.file_size() as usize];

    let bytes_read = file
        .read(&mut file_buffer)
        .expect("Failed to read font file into buffer");

    assert_eq!(
        bytes_read,
        file_info.file_size() as usize,
        "Read size does not match file size: {} != {}",
        bytes_read,
        file_info.file_size() as usize
    );

    file_buffer
}
