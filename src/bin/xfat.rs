use ::xfat::headers::ext4;
use colored::*;
use std::env;
use std::mem::size_of;
use xfat::headers::ext4::dirent::*;
use xfat::headers::ext4::superblock::compat_bitflags;
use xfat::headers::ext4::superblock::Superblock;
use xfat::headers::ext4::*;
use xfat::headers::gpt::partitions::PartitionEntry;
use xfat::headers::gpt::Gpt;
use xfat::headers::mbr::Mbr;
use xfat::headers::reader::*;

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
just for me to mess around and is super messy*/

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
const BLOCK_SIZE: u64 = 512; //this needs a rename

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

	// block offsets are from block_0 on the ext* partition.
	let block_0 = ext4.first_lba * BLOCK_SIZE;
	let super_block_offset = 1024 + block_0;
	let superblock = read_header_from_offset::<Superblock>(&file_arg, super_block_offset); //ext4 pads 1024 bytes ahead of block0
	superblock.debug_print_some_stuf();
	let block_size = 1024 << superblock.log_block_size;

	fn get_offset_from_block_number(block_0: u64, index: u64, block_size: u64) -> u64 {
		block_0 + index * block_size
	}

	println!("BLOCKSIZE: {}", block_size);
	// NOTE there is a subtlety to this if the block size is not 1024, just adding 1024 isn't enough
	// the superblock is either in block 0 +1024bytes if it's large enough or or block 1 if it's 1024
	// the BGD table is at the start of the next block, so either 2 or 1 if 0 is large enough.
	let block_group_desc_table_offset =
		get_offset_from_block_number(block_0, 1 + superblock.superblock as u64, block_size);
	if !superblock.uses_64bit() {
		for i in 0..10 {
			let group_descriptor =
				read_header_from_offset::<ext4::block_group::BlockGroupDescriptor32>(
					&file_arg,
					block_group_desc_table_offset
						+ size_of::<ext4::block_group::BlockGroupDescriptor32>() as u64 * i,
				);
			println!("BGD {}: {:x?}", i, group_descriptor);
			group_descriptor.print_flags();
			if group_descriptor.is_uninitialized() {
				continue;
			}

			let inode_table = get_offset_from_block_number(
				block_0,
				group_descriptor.inode_table_lo as u64,
				block_size,
			) as u64;
			let inode_size = superblock.inode_size;
			for j in 0..superblock.inodes_per_group - group_descriptor.free_inodes_count_lo as u32 {
				let current_offset = inode_table + inode_size as u64 * j as u64;
				let inode =
					read_header_from_offset::<ext4::inode::Inode>(&file_arg, current_offset);

				println!("Inode: 0x{:X}:", j + 1);
				inode.print_fields();
				let file_size = inode.get_file_size();
				let inode_isize = inode.extra_isize;
				let true_size = format!("0x{:X}", file_size).cyan();
				let extra_isize = format!("0x{:X}", inode_isize).cyan();
				println!("FileSize: {} ", true_size);
				println!("extra size: {}", extra_isize);
				if inode.get_ext_attrs_addr() != 0 {
					let extoffset = get_offset_from_block_number(
						block_0,
						inode.get_ext_attrs_addr() as u64,
						block_size,
					) as u64;
					type HdrType = ext4::extattrs::ExtendedAttrBlock;
					let extadd = read_header_from_offset::<HdrType>(&file_arg, extoffset);
					println!("EXTATTR: {:#X?}", extadd);
					println!("size of header: 0x{:x?}", size_of::<HdrType>());
					let size_of_hdr = size_of::<HdrType>() as u64;
					let mut entry_offset = 0;
					loop {
						let extblockbytes = read_bytes_from_file(
							&file_arg,
							extoffset + entry_offset + size_of_hdr,
							0xff + ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME as usize,
						);
						//println!("{:X?}", extblockbytes);
						let extblock = ext4::extattrs::get_extended_attr_entry(&extblockbytes);
						if extblock.is_empty() {
							println!(
								"{}",
								"Next extended attr entry was empty.".cyan().to_string()
							);
							break;
						}
						println!("{:#X?}", extblock);
						entry_offset +=
							ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME + extblock.name_len as u64;
					}
				}
				// print the timestamp is not zero while we're debugging
				if inode.inode_uses_extents() {
					let extent = inode.get_extent();
					println!("Extent: {:#X?}", extent);
					let read_block = extent.leaf.get_block();
					let offset = get_offset_from_block_number(block_0, read_block, block_size);
					let mut table_offset = 0;
					loop {
						let bytes = read_bytes_from_file(&file_arg, offset + table_offset, 263);
						let dirent = get_dir_ent(&bytes[..]);
						if dirent.inode == 0 || dirent.namelen == 0 {
							break;
						} else {
							println!("dirent: {:x?}", dirent);
							println!("file_type: {}", dirent.filetype_to_str());
						}
						table_offset += dirent.record_size() as u64;
						// this logic isn't right yet
						if table_offset == block_size
							|| dirent.filetype == dirent::file_type::FAKE_TAIL_ENTRY_CHECKSUM
						{
							break;
						}
						//honestly most of this logic *waves* isn't right
					}
				}
				if inode.inode_has_extended_attrs() {}
			}
		}
	} else {
		println!("64bit not implemented yet");
	}
}

/* */
