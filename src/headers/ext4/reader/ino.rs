/*
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
pub struct Ino {
    pub inode: Inode,
    pub attr: Option<Exatt>,
    pub extents: Vec<Extent>,
    // can also have a hash tree
}

// print the timestamp is not zero while we're debugging
if inode.inode_uses_extents() {
    let extent = inode.get_extent();
    println!("Extent: {:#X?}", extent);
    let read_block = extent.leaf.get_block();
    let offset = get_offset_from_block_number(ext4_block_0, read_block, block_size);
    let mut table_offset = 0;
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

*/
use super::hashdir;
use crate::headers::constants;
use crate::headers::ext4;
use crate::headers::ext4::dirent;
use crate::headers::ext4::extent;
use crate::headers::ext4::inode::Inode;
use crate::headers::ext4::reader::Exatt;
use crate::headers::ext4::reader::Ino;
use crate::headers::ext4::superblock::Superblock;
use crate::headers::hash;
use crate::headers::reader::*;
use crate::headers::summer;
use std::mem::size_of;

impl Ino {
    pub fn populate_ext_attrs(
        &mut self,
        reader: &mut OnDisk,
        s: &Superblock,
        block0: u64,
    ) {
        if self.inode.get_ext_attrs_addr() != 0 {
            let extoffset = get_offset_from_block_number(
                block0,
                self.inode.get_ext_attrs_addr() as u64,
                s.block_size_bytes(),
            );
            type HdrType = ext4::extattrs::ExtendedAttrBlock;
            let extadd =
                reader.read_header_from_offset::<HdrType>(extoffset);
            //println!("EXTATTR: {:#X?}", extadd);
            //println!("size of header: 0x{:x?}",
            // size_of::<HdrType>());
            let size_of_hdr = size_of::<HdrType>() as u64;
            let mut entry_offset = 0;
            let mut exat = Exatt {
                blk: extadd,
                attrs: vec![],
            };
            loop {
                let extblockbytes = reader.read_bytes_from_file(
                    extoffset + entry_offset + size_of_hdr,
                    0xff + ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME,
                );
                //println!("{:X?}", extblockbytes);
                let extblock =
                    ext4::extattrs::get_extended_attr_entry(
                        &extblockbytes,
                    );
                if !extblock.is_empty() {
                    //println!("{:#X?}", extblock);
                    entry_offset +=
                        ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME
                            + extblock.name_len as u64;
                    exat.attrs.push(extblock);
                } else {
                    break;
                }
            }
            self.attr = Some(exat);
        }
    }

    pub fn populate_extents(
        &mut self,
        reader: &mut OnDisk,
        s: &Superblock,
        block0: u64,
    ) {
        let inode = self.inode;
        inode.print_fields();

        if !inode.inode_uses_extents() {
            return;
        }
        let mut extent = inode.get_extent();
        extent.ascend(reader, block0, s.block_size_bytes());
        self.extent = Some(extent);
    }

    pub fn set_inode_checksum_seed(&mut self, s: &Superblock) {
        if self.seed == 0 || !s.has_feature_checksum_seed() {
            let uuid = s.uuid.clone();
            let inonum = u32::to_le_bytes(self.id);
            let inogen = u32::to_le_bytes(self.inode.generation);
            self.seed = summer::crc32c(!0, uuid.to_vec());
            self.seed = summer::crc32c(self.seed, inonum.to_vec());
            self.seed = summer::crc32c(self.seed, inogen.to_vec());
        }
        if s.has_feature_checksum_seed() {
            self.seed = s.checksum_seed;
        }
    }

    pub fn validate_checksum(
        &mut self,
        reader: &mut OnDisk,
        s: &Superblock,
    ) -> bool {
        if !s.metadata_csum() {
            println!(
                "METADATA_CSUM not set, skipping inode csum \
                 validation"
            );
            return true;
        }

        let mut csum = self.seed;
        let mut inode = self.inode.clone();
        inode.checksum_hi = 0;
        inode.checksum_lo = 0;
        let inode_size = s.inode_size;
        let inode_des = bincode::serialize::<Inode>(&inode).unwrap();

        let mut inode_bytes = reader
            .read_bytes_from_file(self.start, s.inode_size as u64);
        for i in 0..2 {
            inode_bytes[constants::EXT4_INODE_CHECKSUM_LO_OFFSET
                as usize
                + i] = 0;
            if inode_size
                > constants::EXT4_INODE_CHECKSUM_HI_OFFSET + 2
            {
                inode_bytes[constants::EXT4_INODE_CHECKSUM_HI_OFFSET
                    as usize
                    + i] = 0;
            }
        }
        assert_eq!(inode_des[..s.inode_size as usize], inode_bytes);
        let byte_content = &inode_bytes[..s.inode_size as usize];
        csum = summer::crc32c(csum, byte_content.to_vec());
        let mut in_inode = self.inode.checksum();
        if s.inode_size == constants::EXT4_GOOD_OLD_INODE_SIZE
            || s.inode_size
                <= constants::EXT4_INODE_CHECKSUM_HI_OFFSET
        {
            csum &= 0xFFFF;
            in_inode &= 0xFFFF;
        }
        println!(
            "Validation checksum for inode {:X}: expect: {:X} \
             found:{:X}: match?: {}",
            self.id,
            in_inode,
            csum,
            print_bool(in_inode == csum)
        );
        in_inode == csum
    }

    pub fn get_file_content(
        &self,
        reader: &mut OnDisk,
        s: &Superblock,
        block0: u64,
    ) -> Vec<u8> {
        if !self.inode.inode_uses_extents() {
            return vec![];
        }
        let mut tree = self.extent.clone().unwrap();
        tree.walk(
            reader,
            block0,
            s.block_size_bytes(),
            self.inode.get_file_size() as usize,
        )
    }

    pub fn validate_dirent_checksum(
        &self,
        checksum: u32,
        data_block: &[u8],
    ) -> bool {
        let crc = summer::crc32c(self.seed, data_block.to_vec());
        println!(
            "Dirent checksum check (seed:{:X}): {:X} == {:X} ? : {}",
            self.seed,
            crc,
            checksum,
            print_bool(crc == checksum)
        );
        crc == checksum
    }
    pub fn get_directory_entries(
        &mut self,
        reader: &mut OnDisk,
        s: &Superblock,
        block0: u64,
    ) {
        if !self.inode.directory() {
            return;
        }
        let inode = self.inode.clone();
        let mut extents: extent::ExtentTree;
        let mut dirs = vec![];
        match &self.extent {
            Some(tree) => {
                extents = tree.clone();
            }
            None => {
                return;
            }
        }
        let bs = s.block_size_bytes();
        let data = extents.walk(
            reader,
            block0,
            bs,
            self.inode.get_file_size() as usize,
        );
        let mut slice = &data[..];
        println!("Data from extent was length: {}", data.len());

        if s.uses_indexed_dirs()
            && self.inode.uses_hash_tree_directories()
        {
            let root = read_header_from_bytes::<hashdir::Root>(&data);
            root.validate(bs as u16);

            println!("{:X?}", root,);
            println!("{:?}", root.hash_version());

            let entry_offset = std::mem::size_of::<hashdir::Root>();
            let entry_size = std::mem::size_of::<hashdir::Entry>();
            for i in 0..root.count as usize {
                let entry = read_header_from_bytes::<hashdir::Entry>(
                    &data[entry_offset + entry_size * i..],
                );
                println!("{:x?}", entry);
                if !root.last_level() {
                    //the next level is a node to another level
                    println!("not the last level");
                }
                let hash = entry.hash;
                let hashblk = entry.get_block();
                let leaf_found = extents.dx_get_file_block(hashblk);
                let mut leaf: extent::ExtentLeaf;
                match leaf_found {
                    Some(lea) => {
                        leaf = lea;
                    }
                    None => {
                        println!(
                            "Couldn't find file block for {:x?}!!!",
                            entry
                        );
                        continue;
                    }
                }
                let dir_data =
                    leaf.get_file_content(reader, block0, bs, 1024);
                let (_ino, len) =
                    dirent::peek_record_len(&dir_data[..]);
                let dirent =
                    dirent::get_dir_ent(&dir_data[..len as usize]);
                println!("{:x?}", dirent);
                let (major, _minor) = hash::dirhash::create_dirhash(
                    s.hash_seed,
                    &dirent.filename,
                );
                println!("{:X} {:X} {:X}", hash, major, _minor);
                //entry.validate();
                // ah yes, reading the btree as an array to validate
                // it.
            }

            std::process::exit(0);
        } else {
            loop {
                // based on docs I'm pretty sure there isn't more than
                // 1 block of them at a time but not sure if there
                // can be an entire extent tree of blocks of them?
                // either way need to peek header to read len first
                let len_left = data.len() - slice.len();
                let (ino, rec_len) = dirent::peek_record_len(slice);
                if ino == 0 {
                    println!(
                        "found last entry at offset: {}",
                        len_left
                    );
                }
                let cur_slice = &slice[..rec_len as usize];
                let dirent = dirent::get_dir_ent(&cur_slice);
                println!("dirent: {:x?}", dirent);
                println!("file_type: {}", dirent.filetype_to_str());
                let last = dirent.is_last_dirent();
                if dirent.is_checksum_entry() {
                    let csum = dirent.csum.unwrap();

                    println!(
                        "DATA_LEN: {} LEN LEFT: {}",
                        data.len(),
                        len_left
                    );
                    self.validate_dirent_checksum(
                        csum,
                        &data[..len_left],
                    );
                }
                if !last || dirent.is_checksum_entry() {
                    dirs.push(dirent);
                }

                slice = &slice[rec_len as usize..];
                if last || slice.len() == 0 {
                    break;
                }
            }
            self.dirs = Some(dirs);
        }
    }

    /*
    let read_block = extent.leaf.get_block();
        let block_size = s.block_size_bytes();
        let mut table_offset = 0;

        // skip dealing with the journal for now
        if self.id != s.journal_inum && inode.regular_file() {
            let bytes = reader.read_bytes_from_file(offset, inode.get_file_size());
            println!("Found file content... ");
            println!(
                "{}",
                String::from_utf8(bytes).unwrap().to_string().bright_green()
            );
        } else if inode.directory() {
            if inode.uses_hash_tree_directories() {
                println!(
                    "{}",
                    "Hash tree directories not implemented. Probably going to miss reading some directories here 😢"
                );
            }
            loop {
                let bytes = reader.read_bytes_from_file(offset + table_offset, 263);
                let dirent = dirent::get_dir_ent(&bytes[..]);
                println!("dirent: {:x?}", dirent);
                println!("file_type: {}", dirent.filetype_to_str());
                // this logic isn't right yet
                if dirent.inode == 0
                    || dirent.rec_len as u64 + table_offset == block_size
                    || table_offset == block_size
                    || dirent.filetype == dirent::file_type::FAKE_TAIL_ENTRY_CHECKSUM
                {
                    break;
                }
                table_offset += dirent.record_size() as u64;
                //honestly most of this logic *waves* isn't right
            }
        }
    */
}
