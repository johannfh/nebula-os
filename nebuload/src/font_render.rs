use uefi::{
    CStr16,
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    proto::media::{
        file::{File, FileAttribute, FileInfo, FileMode, RegularFile},
        fs::SimpleFileSystem,
    },
};

pub fn load_font(filepath: &str) {
    // Convert the filepath to a CStr16 format
    assert!(
        filepath.len() < 256,
        "Path length exceeds maximum allowed length"
    );
    let mut cstr_buffer = [0u16; 256];
    let filepath = CStr16::from_str_with_buf(filepath, &mut cstr_buffer)
        .expect("Failed to convert filepath to CStr16");

    log::info!("Loading font from path: {}", filepath);

    // Load the font file from the specified path
    let sfs_handle =
        get_handle_for_protocol::<SimpleFileSystem>().expect("Failed to read font file");

    let mut sfs_proto = open_protocol_exclusive::<SimpleFileSystem>(sfs_handle)
        .expect("Failed to open Simple File System protocol");

    let mut root = sfs_proto.open_volume().expect("Failed to open ESP volume");
    log::info!("Opened ESP volume successfully");

    let file_handle = root
        .open(filepath, FileMode::Read, FileAttribute::READ_ONLY)
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

    let info = file
        .get_info::<FileInfo>(&mut file_info_buf)
        .expect("Failed to get file info");

    log::info!("File Size of {}: {}", info.file_name(), info.file_size());
}
