use xfat::headers::exfat;
use xfat::headers::ext4::superblock::Superblock;
use xfat::headers::gpt::partitions::PartitionEntry;
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
    let _processed_header = read_header_from_offset::<Mbr>("tests/mbr.bin", 0);
}

#[test]
fn test_read_mbr_and_boot_sector() {
    let file = "tests/mbr_and_mbs.bin";
    let mbr = read_header_from_offset::<Mbr>(&file, 0);
    let main_boot_sector = read_header_from_offset::<exfat::boot_sector::BootSector>(
        &file,
        mbr.partitions[0].lba_of_partition_start as u64 * 512,
    );
    assert_eq!(main_boot_sector.validate_header(), true);
}

// NOTE: test for extended boot sector is missing, I need an example of one.

#[test]
fn test_read_gpt_and_ext4_partition_entries() {
    let test_file = "tests/multipart.bin";
    let mbr = read_header_from_offset::<Mbr>(test_file, 0);
    println!("{:?}", mbr);
    let gpt = read_header_from_offset::<Gpt>(test_file, 512);
    println!("{:x?}", gpt);
    let ext4 = read_header_from_offset::<PartitionEntry>(test_file, gpt.gpe_table_start * 512);
    let superblock = read_header_from_offset::<Superblock>(test_file, 1024 + ext4.first_lba * 512);
    //ext4 pads 1024 bytes ahead of block0
    assert_eq!(superblock.magic, 0xef53) //ext4 magic
}
