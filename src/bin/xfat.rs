use ::xfat::headers::ext4;
use colored::*;
use std::env;
use std::mem::size_of;
use xfat::headers::ext4::dirent::*;
use xfat::headers::ext4::superblock::Superblock;
use xfat::headers::ext4::*;
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
const SMOL_BLOCKS: u64 = 512; //this needs a rename

fn main() {
	let file_arg = env::args().nth(1).unwrap();
	let mbr = read_header_from_offset::<Mbr>(&file_arg, 0);
	mbr.pretty_print();
	//can add parition sizes to get expected image size.
	let _gpt_part = mbr.get_partition(0);
	let gpt = read_header_from_offset::<Gpt>(&file_arg, 1 * SMOL_BLOCKS);
	println!("{:x?}", gpt);
	gpt.print_partition_table(&file_arg);
	let gpe_ext4 = gpt.get_parition(&file_arg, 0);
	// block offsets are from block_0 on the ext* partition.
	let ext4_block_0 = gpe_ext4.first_lba * SMOL_BLOCKS;
	// ext4 first block has 1024 bytes of padding before superblock.
	let super_block_offset = 1024 + ext4_block_0;
	let superblock = read_header_from_offset::<Superblock>(&file_arg, super_block_offset);
	superblock.debug_print_some_stuf();
	let block_size = superblock.block_size_bytes();

	println!("BLOCKSIZE: {}", block_size);

	// all of this below is ext4 specific

	let block_group_desc_table_offset =
		superblock.get_group_descriptor_table_offset(gpe_ext4.first_lba);
	if !superblock.uses_64bit() {
		for i in 0..superblock.number_of_groups() {
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
					let offset = get_offset_from_block_number(ext4_block_0, read_block, block_size);
					let mut table_offset = 0;
					// files and directories are different SO I GUESS ITS NOT ALL FILES
					if j + 1 != superblock.journal_inum
						&& bitfield_fetch::<u16>(
							inode.mode,
							inode::filemode_bitflags::mutex::S_IFREG,
						) {
						let bytes =
							read_bytes_from_file(&file_arg, offset, inode.get_file_size() as usize);
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
