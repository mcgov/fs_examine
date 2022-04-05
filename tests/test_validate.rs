use xfat::headers::exfat;
use xfat::headers::gpt::*;
use xfat::headers::mbr::*;
use xfat::headers::reader::read_header_from_offset;

#[test]
fn test_validate_known_boot_sector() {
    let processed_header =
        read_header_from_offset::<exfat::boot_sector::BootSector>("tests/main_boot_sector.bin", 0);
    assert_eq!(processed_header.validate_header(), true);
}

#[test]
fn test_read_known_mbr() {
    let processed_header = read_header_from_offset::<Mbr>("tests/mbr.bin", 0);
    println!("{:?}", processed_header);
}

#[test]
fn test_read_mbr_and_boot_sector() {
    let file = "tests/mbr_and_mbs.bin";
    let mbr = read_header_from_offset::<Mbr>(&file, 0);
    let main_boot_sector = read_header_from_offset::<exfat::boot_sector::BootSector>(
        &file,
        mbr.partitions[0].lba_of_partition_start as u64 * 512,
    );
    main_boot_sector.validate_header();
}

// NOTE: test for extended boot sector is missing, I need an example of one.

#[test]
fn test_read_gpt_and_ext4_partition_entries() {
    let test_file = "tests/gpt_ext4_ntfs.bin";
    let mbr = read_header_from_offset::<Mbr>(test_file, 0);
    println!("{:?}", mbr);
    let gpt = read_header_from_offset::<Gpt>(test_file, 1 * 512); // make one to enable code checks
    println!("{:x?}", gpt);
    for i in 0..gpt.gpe_table_entries as u64 {
        let entry = read_header_from_offset::<PartitionEntry>(
            test_file,
            gpt.gpe_table_start * 512 + i * gpt.gpe_table_entry_size as u64,
        );
        println!("{}", entry.name());
        println!("{:?}", entry);
    }
}
