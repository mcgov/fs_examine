use xfat::headers::exfat;
use xfat::headers::ext4::superblock::Superblock;
use xfat::headers::gpt::partitions::PartitionEntry;
use xfat::headers::gpt::*;
use xfat::headers::mbr::*;
use xfat::headers::reader;
#[test]
fn test_validate_known_boot_sector() {
    let mut reader = reader::new("tests/main_boot_sector.bin");
    let processed_header = reader.read_header_from_offset::<exfat::boot_sector::BootSector>(0);
    assert_eq!(processed_header.validate_header(), true);
}

#[test]
fn test_read_known_mbr() {
    let mut reader = reader::new("tests/mbr.bin");
    let _processed_header = reader.read_header_from_offset::<exfat::boot_sector::BootSector>(0);
}

#[test]
fn test_read_mbr_and_boot_sector() {
    let mut reader = reader::new("tests/mbr_and_mbs.bin");

    let mbr = reader.read_header_from_offset::<Mbr>(0);
    let main_boot_sector = reader.read_header_from_offset::<exfat::boot_sector::BootSector>(
        mbr.partitions[0].lba_of_partition_start as u64 * 512,
    );
    assert_eq!(main_boot_sector.validate_header(), true);
}

// NOTE: test for extended boot sector is missing, I need an example of one.

#[test]
fn test_read_gpt_and_ext4_partition_entries() {
    let test_file = "tests/multipart.bin";
    let mut reader = reader::new(test_file);
    let mbr = reader.read_header_from_offset::<Mbr>(0);
    println!("{:?}", mbr);
    let gpt = reader.read_header_from_offset::<Gpt>(512);
    println!("{:x?}", gpt);
    let ext4 = reader.read_header_from_offset::<PartitionEntry>(gpt.gpe_table_start * 512);
    let superblock = reader.read_header_from_offset::<Superblock>(1024 + ext4.first_lba * 512);
    //ext4 pads 1024 bytes ahead of block0
    assert_eq!(superblock.magic, 0xef53) //ext4 magic
}
