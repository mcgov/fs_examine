use std::env;
use xfat::headers::boot_sector::{BootSector, BootSectorRaw};
use xfat::headers::disx86::disassemble;
use xfat::headers::reader::read_header_from_file_unsafe;

use xfat::headers::mbr::{Mbr, MbrRaw};

fn main() {
	let file_arg = env::args().nth(1).unwrap();
	// creates and validates
	let processed_header = read_header_from_file_unsafe::<BootSector, BootSectorRaw>(&file_arg);
	//processed_header.print_header();

	let _mbr: Mbr;
	println!("{:x}", processed_header.volume_length);
	// disassemble the boot sector
	disassemble(
		&processed_header.boot_code,
		16,
		0x78,
		processed_header.boot_code.len(),
	);

	let processed_header = read_header_from_file_unsafe::<Mbr, MbrRaw>("tests/mbr.bin");
	println!("{:?}", processed_header);
}
