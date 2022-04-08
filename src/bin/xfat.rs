use std::env;
use xfat::headers::ext4::superblock::Superblock;
use xfat::headers::gpt::partitions::PartitionEntry;
use xfat::headers::gpt::Gpt;
use xfat::headers::mbr::Mbr;
use xfat::headers::reader::read_header_from_offset;
//use xfat::headers::xfs::ondiskhdr::XfsOndiskHeader;
/*
	let processed_header = read_header_from_file_unsafe::<BootSector, BootSectorRaw>(&file_arg);
	println!("{:x}", processed_header.volume_length);
	// disassemble the boot sector
	disassemble(
		&processed_header.boot_code,
		16,
		0x78,
		processed_header.boot_code.len(),
	);

	let mbr = read_header_from_file::<Mbr>(&file_arg, 0);
	println!("{:?}", mbr);
	let main_exfat = read_header_from_file::<BootSector>(
		&file_arg,
		mbr.partitions[0].lba_of_partition_start as u64 * 512,
	);
	main_exfat.print_header();
	main_exfat.validate_header();
	let extended_boot_sector = read_header_from_file::<ExtendedBootSector>(&file_arg, 512);
	println!(
		"extended boot sector is valid: {:x?}",
		extended_boot_sector.section_is_valid(main_exfat.bytes_per_sector_shift)
	);
*/
const BLOCK_SIZE: u64 = 512;

fn main() {
	let file_arg = env::args().nth(1).unwrap();
	let mbr = read_header_from_offset::<Mbr>(&file_arg, 0);
	println!("{:?}", mbr);
	let gpt = read_header_from_offset::<Gpt>(&file_arg, 1 * BLOCK_SIZE); // make one to enable code checks
	println!("{:x?}", gpt);
	for i in 0..7 as u64 {
		let entry = read_header_from_offset::<PartitionEntry>(
			&file_arg,
			gpt.gpe_table_start * BLOCK_SIZE + i * gpt.gpe_table_entry_size as u64,
		);
		println!("{}", entry.name());
		println!("{:x?}", entry);
		println!("{:?}", entry.type_to_str());
	}
	let ext4 =
		read_header_from_offset::<PartitionEntry>(&file_arg, gpt.gpe_table_start * BLOCK_SIZE);

	let superblock =
		read_header_from_offset::<Superblock>(&file_arg, 1024 + ext4.first_lba * BLOCK_SIZE); //ext4 pads 1024 bytes ahead of block0
	println!("{:x?}", superblock);
	println!("volume name: {}", superblock.volume_name());
	println!("volume name: {}", superblock.mount_opts());
	println!("volume name: {}", superblock.last_mounted());
	println!("first_error: {}", superblock.first_error_func());
	println!("last_error : {}", superblock.last_error_func());
}
