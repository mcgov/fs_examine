use std::env;
use xfat::headers::exfat::boot_sector::BootSector;
use xfat::headers::exfat::extended_boot_sector::ExtendedBootSector;
use xfat::headers::gpt::Gpt;
use xfat::headers::mbr::Mbr;
use xfat::headers::reader::read_header_from_file;
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
*/

fn main() {
	let file_arg = env::args().nth(1).unwrap();
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
	let _a: Gpt; // make one to enable code checks
}
