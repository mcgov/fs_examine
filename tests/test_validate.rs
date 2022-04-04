use xfat::headers::exfat;
use xfat::headers::mbr::*;
use xfat::headers::reader::read_header_from_file;

#[test]
fn test_validate_known_boot_sector() {
    let processed_header =
        read_header_from_file::<exfat::boot_sector::BootSector>("tests/main_boot_sector.bin", 0);
    assert_eq!(processed_header.validate_header(), true);
}

#[test]
fn test_read_known_mbr() {
    let processed_header = read_header_from_file::<Mbr>("tests/mbr.bin", 0);
    println!("{:?}", processed_header);
}

#[test]
fn test_read_mbr_and_boot_sector() {
    let file = "tests/mbr_and_mbs.bin";
    let mbr = read_header_from_file::<Mbr>(&file, 0);
    let main_boot_sector = read_header_from_file::<exfat::boot_sector::BootSector>(
        &file,
        mbr.partitions[0].lba_of_partition_start as u64 * 512,
    );
    main_boot_sector.validate_header();
}

// NOTE: test for extended boot sector is missing, I need an example of one.
