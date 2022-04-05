use std::env;
use xfat::headers::exfat::boot_sector::BootSector;
use xfat::headers::exfat::extended_boot_sector::ExtendedBootSector;
use xfat::headers::gpt::{Gpt, PartitionEntry};
use xfat::headers::mbr::Mbr;
use xfat::headers::reader::read_header_from_sector;
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
const BLOCK_SIZE: u16 = 512;

// TODO: reading by blocks was dumb, need to swap back to byte offset,
// headers aren't all 512 bytes

fn main() {
	let file_arg = env::args().nth(1).unwrap();
	let mbr = read_header_from_sector::<Mbr>(&file_arg, 0, BLOCK_SIZE);
	println!("{:?}", mbr);
	let gpt = read_header_from_sector::<Gpt>(&file_arg, 1, BLOCK_SIZE); // make one to enable code checks
	println!("{:x?}", gpt);
	for i in 0..gpt.gpe_table_entries {
		let entry = read_header_from_sector::<PartitionEntry>(
			&file_arg,
			gpt.gpe_table_start + i as u64,
			BLOCK_SIZE,
		);
		println!("{}", entry.name());
		println!("{:?}", entry);
	}
}
