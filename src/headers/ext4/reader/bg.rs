use crate::headers::ext4::block_group::*;
use crate::headers::ext4::reader::Bg;
use crate::headers::ext4::reader::Ino;
use crate::headers::ext4::superblock::Superblock;
use crate::headers::reader::*;
use crate::headers::*;
use colored::*;
impl Bg {
    pub fn init(
        id: u32,
        start: u64,
        smol: Option<BlockGroupDescriptor32>,
        big: Option<BlockGroupDescriptor64>,
    ) -> Bg {
        Bg {
            id: id,
            start: start,
            b32: smol,
            b64: big,
            ino: vec![],
        }
    }

    pub fn print(&self) {
        match &self.b32 {
            Some(bg) => {
                println!("BG32: {:#X?}", bg);
            }
            _ => {}
        }
        match &self.b64 {
            Some(bg) => {
                println!("BG64: {:#X?}", bg);
            }
            _ => {}
        }
    }

    pub fn get_inode_table_block(&self) -> u64 {
        let mut block: u64 =
            self.b32.as_ref().unwrap().inode_table_lo
                as u64;
        match &self.b64 {
            Some(b) => {
                block |= (b.inode_table_hi as u64) << 32;
            }
            None => {}
        }
        block
    }
    pub fn get_free_inodes_count(&self) -> u32 {
        let mut free_count =
            self.b32.as_ref().unwrap().free_inodes_count_lo
                as u32;
        match &self.b64 {
            Some(b) => {
                free_count |=
                    (b.free_inodes_count_hi as u32) << 16;
            }
            None => {}
        }
        free_count
    }

    pub fn is_uninitialized(&self) -> bool {
        let bg32 = self.b32.unwrap();
        bg32.is_uninitialized()
    }

    pub fn populate_inodes(
        &mut self,
        reader: &mut OnDisk,
        s: &Superblock,
        start: u64,
    ) {
        if self.is_uninitialized() {
            //println!("Skipping uninitialized block
            // group...");
            return;
        }
        let block_table = self.get_inode_table_block();
        let bs = s.block_size_bytes();
        let inode_table = get_offset_from_block_number(
            start,
            block_table,
            bs,
        ) as u64;
        let inode_size = s.inode_size;
        let node_count = s.inodes_per_group
            - self.get_free_inodes_count();
        if node_count != 0 {
            println!("found {} inodes", node_count);
        }
        for j in 0..s.inodes_per_group
            - self.get_free_inodes_count()
        {
            let current_offset =
                inode_table + inode_size as u64 * j as u64;
            let inode = reader.read_header_from_offset::<ext4::inode::Inode>(current_offset);
            //inode.print_fields();
            let mut ino = Ino {
                start: current_offset,
                id: (j + 1) + self.id * s.inodes_per_group,
                inode: inode,
                attr: None,
                extent: None,
                dirs: None,
                seed: s.checksum_seed,
            };
            ino.populate_ext_attrs(reader, s, start);
            ino.populate_extents(reader, s, start);
            // doesn't differentiate between file content
            // and dirents yet
            if ino.id != s.journal_inum {
                let cnt =
                    ino.get_file_content(reader, s, start);
                if cnt.len() > 0 && cnt.len() < 0x1000 {
                    println!("Inode number: {}", ino.id);
                    if cnt.len() > 0x100 {
                        //println!("{:X?}", &cnt[..100]);
                    }
                    let cont =
                        String::from_utf8_lossy(&cnt);
                    println!(
                        "Content:{}, {}",
                        cont.len(),
                        format!("{}", cont.green())
                    );
                }
            }
            ino.set_inode_checksum_seed(s);
            ino.validate_checksum(reader, s);
            ino.get_directory_entries(reader, s, start);

            self.ino.push(ino);
        }
    }
}
