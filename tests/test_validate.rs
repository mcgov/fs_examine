use fs_examine::headers::exfat;
use fs_examine::headers::ext4::superblock::Superblock;
use fs_examine::headers::gpt::partitions::PartitionEntry;
use fs_examine::headers::gpt::*;
use fs_examine::headers::mbr::*;
use fs_examine::headers::reader;

/* low-quality semi-useless tests. */
#[test]
fn test_md4() {
    /*
            MD4 test suite:
        MD4 ("") = 31d6cfe0d16ae931b73c59d7e0c089c0
        MD4 ("a") = bde52cb31de33e46245e05fbdbd6fb24
        MD4 ("abc") = a448017aaf21d8525fc10ae87aa6729d
        MD4 ("message digest") = d9130a8164549fe818874806e1c7014b
        MD4 ("abcdefghijklmnopqrstuvwxyz") = d79e1c308aa5bbcdeea8ed63df412da9
        MD4 ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789") =
        043f8582f241db351ce627e153e7f0e4
        MD4 ("123456789012345678901234567890123456789012345678901234567890123456
        78901234567890") = e33b4ddc9c38f2199c3e7b164fcc0536
        *//*
        let mut md4 = md4::init();
        md4.update(&[]);
        let mut hash = md4.finalize();
    */
    // I grabbed these validation strings from the RFC but
    // the ext4 half-md4 implementation isn't the same
    // as the full version in the RFC. I leave these as a monument
    // to the time I lost figuring that out.
}

#[test]
fn test_validate_known_boot_sector() {
    let mut reader = reader::new("tests/main_boot_sector.bin");
    let processed_header = reader
        .read_header_from_offset::<exfat::boot_sector::BootSector>(0);
    assert_eq!(processed_header.validate_header(), true);
}

#[test]
fn test_read_known_mbr() {
    let mut reader = reader::new("tests/mbr.bin");
    let _processed_header = reader
        .read_header_from_offset::<exfat::boot_sector::BootSector>(0);
}

#[test]
fn test_read_mbr_and_boot_sector() {
    let mut reader = reader::new("tests/mbr_and_mbs.bin");

    let mbr = reader.read_header_from_offset::<Mbr>(0);
    let main_boot_sector = reader
        .read_header_from_offset::<exfat::boot_sector::BootSector>(
            mbr.partitions[0].lba_of_partition_start as u64 * 512,
        );
    assert_eq!(main_boot_sector.validate_header(), true);
}

// NOTE: test for extended boot sector is missing, I need an example
// of one.

#[test]
fn test_read_gpt_and_ext4_partition_entries() {
    let test_file = "tests/multipart.bin";
    let mut reader = reader::new(test_file);
    let mbr = reader.read_header_from_offset::<Mbr>(0);
    println!("{:?}", mbr);
    let gpt = reader.read_header_from_offset::<Gpt>(512);
    println!("{:x?}", gpt);
    let ext4 = reader.read_header_from_offset::<PartitionEntry>(
        gpt.gpe_table_start * 512,
    );
    let superblock = reader.read_header_from_offset::<Superblock>(
        1024 + ext4.first_lba * 512,
    );
    //ext4 pads 1024 bytes ahead of block0
    assert_eq!(superblock.magic, 0xef53) //ext4 magic
}
