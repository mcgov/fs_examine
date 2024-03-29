use fs_examine::headers::fs::disk;
use fs_examine::headers::mbr;
use fs_examine::headers::reader;
use std::env;
// fs_examine
// look at what's on some kinds of disk like devices

fn main() {
	let file_arg = env::args().nth(1);
	match file_arg {
		Some(_) => {
			println!("Let's get fs_examine-ey!");
		}
		None => {
			println!(
				"usage: fs_examine /dev/sdb \n(will fail on a disk \
				 without an MBR or GPT, ext4 fs only at the moment)"
			)
		}
	}
	// start building our disk
	let mut reader = reader::new(&file_arg.unwrap());

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
				let mut ext4_reader =
					d.make_ext4_block_reader(ext4part);
				ext4_reader.populate_blocks();
				// NOTE: haven't 100%'d EXT4,
				// hash indexed directories are broken still.
				// 64/32bit linear directories should work.
				// extent tree walking works
				// flex_bg and meta_bg not implemented yet.
			}
			disk::PartitionType::Unused => { /* */ }
			_ => {
				println!(
					"Note: Filesystem partition type {:?} is not \
					 implemented.",
					part.p_type
				);
			}
		}
	}
}
