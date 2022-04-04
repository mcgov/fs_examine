use std::env;
use xfat::headers::boot_sector::{BootSector, BootSectorRaw};
use xfat::headers::disx86::disassemble;
use xfat::headers::reader::read_header_from_file_unsafe;

use xfat::headers::mbr::{Mbr, MbrRaw};

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
	let mbr = read_header_from_file_unsafe::<Mbr, MbrRaw>(&file_arg, 0);
	println!("{:?}", mbr);
	let main_boot_sector = read_header_from_file_unsafe::<BootSector, BootSectorRaw>(
		&file_arg,
		mbr.partitions[0].lba_of_partition_start as u64 * 512,
	);
	main_boot_sector.print_header();
	main_boot_sector.validate_header();
}
