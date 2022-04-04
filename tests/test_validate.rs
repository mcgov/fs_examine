use xfat::headers::boot_sector::*;
use xfat::headers::mbr::*;
use xfat::headers::reader::read_header_from_file_unsafe;

#[test]
fn test_validate_known_boot_sector() {
    let processed_header =
        read_header_from_file_unsafe::<BootSector, BootSectorRaw>("tests/main_boot_sector.bin");
    assert_eq!(processed_header.validate_header(), true);
}

#[test]
fn test_read_known_mbr() {
    let processed_header = read_header_from_file_unsafe::<Mbr, MbrRaw>("tests/mbr.bin");
    println!("{:?}", processed_header);
}
