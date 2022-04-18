use std::env;
use xfat::headers::fs::disk;
use xfat::headers::mbr;
use xfat::headers::reader;

/* =============================================== #
██████╗ ██╗   ██╗██████╗ ████████╗
██╔══██╗██║   ██║██╔══██╗╚══██╔══╝
██████╔╝██║   ██║██████╔╝   ██║
██╔═══╝ ██║   ██║██╔══██╗   ██║
██║     ╚██████╔╝██║  ██║   ██║
╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   Partition Unified ReadTer
PURT Urs Rot a Tarpition Peedurt
*/

fn main() {
	let file_arg = env::args().nth(1).unwrap();

	// start building our disk
	let mut reader = reader::new(&file_arg);
	let mut d: disk::Disk = disk::Disk {
		mbr: reader.read_header_from_offset::<mbr::Mbr>(0),
		reader: reader,
		pt_type: disk::PartitionTableType::Mbr,
		partitions: vec![],
	};
	d.mbr.pretty_print();

	// get that first partition to check for GPT
	d.set_partition_table_type(); // will panic on unimplemented partition type
	d.validate_headers();
	d.register_partitions();
	d.print_partitions_pretty();
	for part in d.partitions.clone().into_iter() {
		match part.p_type {
			disk::PartitionType::Ext4 => {
				let ext4part = part.clone();
				let mut ext4_reader = d.make_ext4_block_reader(ext4part);
				//if !ext4_reader.s.uses_64bit() {
				//continue;
				//}
				ext4_reader.populate_blocks();
			}
			disk::PartitionType::Unused => { /* */ }
			_ => {
				println!(
					"Note: Filesystem partition type {:?} is not implemented.",
					part.p_type
				);
			}
		}
	}
}
