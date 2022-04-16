use crate::headers::ext4::block_group::*;
use crate::headers::ext4::reader::Bg;
use crate::headers::ext4::reader::Ino;
use crate::headers::ext4::reader::Part;
use crate::headers::ext4::superblock::Superblock;
use crate::headers::reader::*;
use crate::headers::*;
impl Bg {
    pub fn init(
        start: u64,
        smol: Option<BlockGroupDescriptor32>,
        big: Option<BlockGroupDescriptor64>,
    ) -> Bg {
        Bg {
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
        let mut block: u64 = self.b32.as_ref().unwrap().inode_table_lo as u64;
        match &self.b64 {
            Some(b) => {
                block |= (b.inode_table_hi as u64) << 32;
            }
            None => {}
        }
        block
    }
    pub fn get_free_inodes_count(&self) -> u32 {
        let mut free_count = self.b32.as_ref().unwrap().free_inodes_count_lo as u32;
        match &self.b64 {
            Some(b) => {
                free_count |= (b.free_inodes_count_hi as u32) << 16;
            }
            None => {}
        }
        free_count
    }

    pub fn populate_inodes(&mut self, file: &str, s: &Superblock, start: u64) {
        let mut is64 = false;
        match self.b64 {
            Some(_) => {
                is64 = true;
            }
            None => {}
        }
        let bg32 = self.b32.as_ref().unwrap();
        if bg32.is_uninitialized() {
            return;
        }
        let inode_table =
            get_offset_from_block_number(start, self.get_inode_table_block(), s.block_size_bytes())
                as u64;
        let inode_size = s.inode_size;
        for j in 0..s.inodes_per_group - self.get_free_inodes_count() {
            let current_offset = inode_table + inode_size as u64 * j as u64;
            let inode = read_header_from_offset::<ext4::inode::Inode>(file, current_offset);
            //inode.print_fields();
            let mut ino = Ino {
                id: j + 1,
                inode: inode,
                attr: None,
                extents: vec![],
            };
            ino.populate_ext_attrs(&file, s, start);
            ino.populate_extents(&file, s, start);
            self.ino.push(ino);
        }
    }
}
