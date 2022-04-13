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

const SMOL_BLOCKS: u64 = 512; //this needs a rename

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
	let gpt = d.get_gpt();
	d.register_partitions();
	d.print_partitions();
	return;
	let gpe_ext4 = gpt.get_parition(&file_arg, 0);
	// block offsets are from block_0 on the ext* partition.
	let ext4_block_0 = gpe_ext4.first_lba * SMOL_BLOCKS;
	// ext4 first block has 1024 bytes of padding before superblock.
	let super_block_offset = superblock::BLOCK_0_PADDING + ext4_block_0;
	let superblock = read_header_from_offset::<Superblock>(&file_arg, super_block_offset);
	if !superblock.check_magic_field(&file_arg, super_block_offset) {
		println!("Magic field was invalid for this superblock");
		return;
	}
	superblock.debug_print_some_stuf();
	let block_size = superblock.block_size_bytes();

	// all of this below is ext4 specific

	let block_group_desc_table_offset =
		superblock.get_group_descriptor_table_offset(gpe_ext4.first_lba);
	if !superblock.uses_64bit() {
		for i in 0..superblock.number_of_groups() {
			let item_group_table_offset = block_group_desc_table_offset + 64 as u64 * i;
			let mut group_descriptor = read_header_from_offset::<
				ext4::block_group::BlockGroupDescriptor32,
			>(&file_arg, item_group_table_offset);
			group_descriptor.uuid = superblock.uuid.as_u128();
			group_descriptor.bg_id = i as u16;
			summer::struct_validate_checksum16::<ext4::block_group::BlockGroupDescriptor32>(
				&file_arg,
				&group_descriptor,
				item_group_table_offset,
			);
			return;
			group_descriptor.pretty_print(i);
			if group_descriptor.is_uninitialized() {
				continue;
			}

			let inode_table = get_offset_from_block_number(
				ext4_block_0,
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
						ext4_block_0,
						inode.get_ext_attrs_addr() as u64,
						block_size,
					);
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
							0xff + ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME,
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
					let offset = get_offset_from_block_number(ext4_block_0, read_block, block_size);
					let mut table_offset = 0;
					// files and directories are different SO I GUESS ITS NOT ALL FILES
					if j + 1 != superblock.journal_inum
						&& bitfield_fetch::<u16>(
							inode.mode,
							inode::filemode_bitflags::mutex::S_IFREG,
						) {
						let bytes = read_bytes_from_file(&file_arg, offset, inode.get_file_size());
						println!("Found file content... ");
						println!(
							"{}",
							String::from_utf8(bytes).unwrap().bright_green().to_string()
						);
					} else if bitfield_fetch::<u16>(
						inode.mode,
						inode::filemode_bitflags::mutex::S_IFDIR,
					) {
						if inode.uses_hash_tree_directories() {
							println!(
								"{}",
								"Hash tree directories not implemented. Probably going to miss reading some directories here 😢".red().to_string()
							);
						}
						loop {
							let bytes = read_bytes_from_file(&file_arg, offset + table_offset, 263);
							let dirent = get_dir_ent(&bytes[..]);
							println!("dirent: {:x?}", dirent);
							println!("file_type: {}", dirent.filetype_to_str());
							// this logic isn't right yet
							if dirent.inode == 0
								|| dirent.rec_len as u64 + table_offset == block_size
								|| table_offset == block_size || dirent.filetype
								== dirent::file_type::FAKE_TAIL_ENTRY_CHECKSUM
							{
								break;
							}
							table_offset += dirent.record_size() as u64;
							//honestly most of this logic *waves* isn't right
						}
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
