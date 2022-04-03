use std::env;
use xfat::headers::boot_sector::read_boot_sector_header;
use xfat::headers::disx86::disassemble;

fn main() {
	let file_arg = env::args().nth(1).unwrap();
	// creates and validates
	let processed_header = read_boot_sector_header(&file_arg);
	//processed_header.print_header();

	println!("{:x}", processed_header.volume_length);
	// disassemble the boot sector
	disassemble(
		&processed_header.boot_code,
		16,
		0x78,
		processed_header.boot_code.len(),
	)
}
