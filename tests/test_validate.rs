use xfat::headers::boot_sector::read_boot_sector_header;

#[test]
fn test_validate_known_mbr() {
    let processed_header = read_boot_sector_header("tests/mbr.bin");
    assert_eq!(processed_header.validate_header(), true);
}
