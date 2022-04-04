use xfat::headers::boot_sector::*;
use xfat::headers::mbr::*;
use xfat::headers::reader::read_header_from_file_unsafe;

#[test]
fn test_validate_known_boot_sector() {
    let processed_header =
        read_header_from_file_unsafe::<BootSector, BootSectorRaw>("tests/main_boot_sector.bin", 0);
    assert_eq!(processed_header.validate_header(), true);
}

#[test]
fn test_read_known_mbr() {
    let processed_header = read_header_from_file_unsafe::<Mbr, MbrRaw>("tests/mbr.bin", 0);
    println!("{:?}", processed_header);
}

#[test]
fn test_read_mbr_and_boot_sector() {
    let file = "tests/mbr_and_mbs.bin";
    let mbr = read_header_from_file_unsafe::<Mbr, MbrRaw>(&file, 0);
    let main_boot_sector = read_header_from_file_unsafe::<BootSector, BootSectorRaw>(
        &file,
        mbr.partitions[0].lba_of_partition_start as u64 * 512,
    );
    main_boot_sector.validate_header();
}
