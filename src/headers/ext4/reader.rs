use super::block_group::*;
use super::dirent::*;
use super::extattrs::*;
use super::extent::*;
use super::inode::*;
use super::superblock::*;
use super::*;
use crate::headers::reader::read_header_from_offset;
use colored::*;

pub struct Part {
    pub file: String,
    pub start: u64,
    pub s: Superblock,
    pub bg: Vec<Bg>,
}

pub struct Bg {
    pub b32: Option<BlockGroupDescriptor32>,
    pub b64: Option<BlockGroupDescriptor64>,
    pub ino: Vec<Ino>,
}

impl Bg {
    pub fn init(smol: Option<BlockGroupDescriptor32>, big: Option<BlockGroupDescriptor64>) -> Bg {
        Bg {
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
}

pub struct Ino {
    pub node: Inode,
    pub attr: Option<Exatt>,
    pub extents: Vec<Extent>,
    // can also have a hash tree
}

pub struct Exatt {
    blk: ExtendedAttrBlock,
    attrs: Vec<ExtendedAttrEntry>,
}

pub struct Extent {
    pub hdr: ExtentHeader,
    pub branches: Vec<ExtentNode>,
    pub leafs: Vec<ExtentLeaf>,
    pub tail: Option<ExtentTail>,
}

impl Part {
    pub fn init(file: String, sb: Superblock, start: u64) -> Part {
        Part {
            file: file.clone(),
            start: start,
            s: sb,
            bg: vec![],
        }
    }
    pub fn read_entire_fs(&mut self) {
        self.s.debug_print_some_stuf();
        let bgdt_offset = self.s.get_group_descriptor_table_offset(self.start);
        for i in 0..self.s.number_of_groups() {
            if self.s.uses_64bit() && self.s.desc_size > 32 {
                let combined_size = std::mem::size_of::<BlockGroupDescriptor32>()
                    + std::mem::size_of::<BlockGroupDescriptor64>();
                if self.s.desc_size < combined_size as u16 {
                    panic!(
                        "size for 64bit group descriptor didn't validate, should be at least {}",
                        combined_size
                    );
                }
                let bg_offset = bgdt_offset + combined_size as u64 * i;
                let bg32 = read_header_from_offset::<BlockGroupDescriptor32>(&self.file, bg_offset);
                let bg64 = read_header_from_offset::<BlockGroupDescriptor64>(
                    &self.file,
                    bg_offset + std::mem::size_of::<BlockGroupDescriptor32>() as u64,
                );
                let bgboi = Bg::init(Some(bg32), Some(bg64));
                //bgboi.print();
                self.bg.push(bgboi);
            } else {
                let bg_offset =
                    bgdt_offset + std::mem::size_of::<BlockGroupDescriptor32>() as u64 * i;
                let bg = read_header_from_offset::<BlockGroupDescriptor32>(&self.file, bg_offset);
                let bgboi = Bg::init(Some(bg), None);
                //bgboi.print();
                self.bg.push(bgboi);
            }
        }
        // TODO: validate each one, these have checksums
        println!(
            "{} sanity check: {:X}",
            format!("found {:X} block group descriptors.", self.bg.len()).blue(),
            self.s.number_of_groups()
        );
    }
}
