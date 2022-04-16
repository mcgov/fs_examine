use ::xfat::headers::ext4;
use colored::*;
use std::env;
use std::mem::size_of;
use xfat::headers::ext4::dirent::*;
use xfat::headers::ext4::superblock::Superblock;
use xfat::headers::ext4::*;
use xfat::headers::fs::disk;
use xfat::headers::gpt::Gpt;
use xfat::headers::mbr;
use xfat::headers::reader::*;
use xfat::headers::summer;

/*
███████╗██╗   ██╗██████╗ ███████╗██████╗
██╔════╝██║   ██║██╔══██╗██╔════╝██╔══██╗
███████╗██║   ██║██████╔╝█████╗  ██████╔╝
╚════██║██║   ██║██╔═══╝ ██╔══╝  ██╔══██╗
███████║╚██████╔╝██║     ███████╗██║  ██║
╚══════╝ ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═╝
██████╗ ██╗      ██████╗  ██████╗██╗  ██╗
██╔══██╗██║     ██╔═══██╗██╔════╝██║ ██╔╝
██████╔╝██║     ██║   ██║██║     █████╔╝
██╔══██╗██║     ██║   ██║██║     ██╔═██╗
██████╔╝███████╗╚██████╔╝╚██████╗██║  ██╗
╚═════╝ ╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝
enjoy this fun header and don't bother reading this main it's
just for me to mess around and is super messy
*/

fn main() {
	let file_arg = env::args().nth(1).unwrap();

	// start building our disk
	let mut d: disk::Disk = disk::Disk {
		mbr: read_header_from_offset::<mbr::Mbr>(&file_arg, 0),
		pt_type: disk::PartitionTableType::Mbr,
		partitions: vec![],
		file_arg: file_arg.clone(),
	};
	d.mbr.pretty_print();

	// get that first partition to check for GPT
	d.set_partition_table_type(); // will panic on unimplemented partition type
	d.print_partitions_pretty();
	d.register_partitions();
	//d.print_partitions();
	for part in d.partitions.clone().into_iter() {
		if matches!(part.p_type, disk::PartitionType::Ext4) {
			let ext4part = part.clone();
			let mut ext4_reader = d.make_ext4_reader(ext4part);
			//if !ext4_reader.s.uses_64bit() {
			//continue;
			//}
			ext4_reader.populate_block_groups();
			ext4_reader.validate_block_groups();
			ext4_reader.populate_inodes(); // genuinely stumped on what's broken in the crc16 for 32bit.
		}
	}
}
