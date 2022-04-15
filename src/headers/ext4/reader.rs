use super::block_group::*;
use super::dirent::*;
use super::extattrs::*;
use super::extent::*;
use super::inode::*;
use super::superblock::*;
use super::*;
use crate::headers::reader::{read_bytes_from_file, read_header_from_offset};
use crate::headers::summer;
use colored::*;
use crc::*;
use lazy_static::lazy_static;
use uuid::Uuid;

pub struct Part {
    pub file: String,
    pub start: u64,
    pub s: Superblock,
    pub bg: Vec<Bg>,
}

pub struct Bg {
    pub start: u64,
    pub b32: Option<BlockGroupDescriptor32>,
    pub b64: Option<BlockGroupDescriptor64>,
    pub ino: Vec<Ino>,
}

impl Bg {
    pub fn init(
        start: u64,
        smol: Option<BlockGroupDescriptor32>,
        big: Option<BlockGroupDescriptor64>,
    ) -> Bg {
        Bg {
            start,
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
    pub fn populate_block_groups(&mut self) {
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
                let bgboi = Bg::init(bg_offset, Some(bg32), Some(bg64));
                //bgboi.print();
                self.bg.push(bgboi);
            } else {
                let bg_offset =
                    bgdt_offset + std::mem::size_of::<BlockGroupDescriptor32>() as u64 * i;
                let bg = read_header_from_offset::<BlockGroupDescriptor32>(&self.file, bg_offset);
                let bgboi = Bg::init(bg_offset, Some(bg), None);
                //bgboi.print();
                self.bg.push(bgboi);
            }
        }
        // TODO:

        //validate each one, these have checksums
        println!(
            "{} sanity check: {:X}",
            format!("found {:X} block group descriptors.", self.bg.len()).blue(),
            self.s.number_of_groups()
        );
    }

    pub fn validate_block_groups(&mut self) {
        self.s.debug_print_some_stuf();
        if self.s.metadata_csum() {
            let csum_seed = self.s.checksum_seed;
            unsafe {
                Algo32.init = csum_seed;
            }
            for bgid in 0..self.bg.len() {
                let mut bytes: Vec<u8> = vec![];

                for byte in self.s.uuid {
                    bytes.push(byte);
                }
                for byte in <u32>::to_le_bytes(bgid.try_into().unwrap()) {
                    bytes.push(byte);
                }
                let bg_item = self.bg.get(bgid).unwrap();
                let bg_start = bg_item.start;
                bytes.append(&mut reader::read_bytes_from_file(
                    &self.file, bg_start, 0x1e,
                ));
                bytes.push(0);
                bytes.push(0); //fake checksum field
                if self.s.uses_64bit() && self.s.desc_size > 32 {
                    bytes.append(&mut reader::read_bytes_from_file(
                        &self.file,
                        bg_start + 0x20,
                        (self.s.desc_size - 0x20) as u64,
                    ));
                }

                unsafe {
                    let crcsum = summer::crc32_bytes(&self.file, &Algo32, bytes);
                    if bg_item.b32.as_ref().unwrap().checksum as u32 != (crcsum & 0xffff) {
                        panic!("checksum did not match!!");
                    }
                }
            }
        } else if self.s.has_feature_gdt_csum() {
            // old version
            for bgid in 0..self.bg.len() {
                let mut bytes: Vec<u8> = vec![];

                let mut bytesdisk =
                    reader::read_bytes_from_file(&self.file, self.start + 1024 + 0x68, 16);

                bytes.append(&mut bytesdisk);
                println!(
                    "{:x?}",
                    reader::read_bytes_from_file(&self.file, self.start + 1024 + 0x68, 16)
                );
                println!("{:x?}", bytes);
                println!("bgid:{}", bgid);
                println!("desc_size {}", self.s.superblock);
                for byte in <u32>::to_le_bytes(bgid as u32 + 1) {
                    bytes.push(byte);
                }
                println!("{:x?}", bytes);

                let bg_item = self.bg.get(bgid).unwrap();
                let bg_start = bg_item.start;
                unsafe {
                    let bites = std::mem::transmute::<BlockGroupDescriptor32, [u8; 0x20]>(
                        bg_item.b32.as_ref().unwrap().clone(),
                    );
                    println!("as:{:02x?}", bites[..bites.len() - 2].to_vec());
                    //bytes.append(&mut bites[..bites.len() - 2].to_vec())
                }
                println!(
                    "dk:{:02x?}",
                    reader::read_bytes_from_file(&self.file, bg_start, 0x1e),
                );
                bytes.append(&mut reader::read_bytes_from_file(
                    &self.file, bg_start, 0x1e,
                ));

                println!("{:02x?}", bytes);
                println!("{:x?}", bg_item.b32.as_ref().unwrap());
                let a = summer::crc16_bytes(&self.file, &Algo161, bytes.clone());
                println!("{:x?} {:x?}", a, !a,);
                let b = summer::crc16_bytes(&self.file, &Algo162, bytes.clone());
                println!("{:x?} {:x?}", b, !b);
                let c = summer::crc16_bytes(&self.file, &Algo163, bytes.clone());
                println!("{:x?} {:x?}", c, !c,);
                let crcsum = summer::crc16_bytes(&self.file, &Algo16, bytes.clone());
                let bgcrc = bg_item.b32.as_ref().unwrap().checksum;
                if bgcrc != (crcsum & 0xffff) {
                    panic!(
                        "checksum did not match: {:x} {:x} {:x} {:x}",
                        crcsum, !crcsum, !bgcrc, bgcrc
                    );
                } else {
                    println!("checksum matches for bg {}", bgid);
                }
            }
        }
    }
}

static Algo16: Algorithm<u16> = Algorithm::<u16> {
    poly: 0x8005,
    init: 0xFFFF,
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0,
    residue: 0,
};
static Algo161: Algorithm<u16> = Algorithm::<u16> {
    poly: 0x8005,
    init: 0xFFFF,
    refin: false,
    refout: true,
    xorout: 0xffff,
    check: 0,
    residue: 0,
};
static Algo162: Algorithm<u16> = Algorithm::<u16> {
    poly: 0x8005,
    init: 0xFFFF,
    refin: true,
    refout: false,
    xorout: 0xffff,
    check: 0,
    residue: 0,
};
static Algo163: Algorithm<u16> = Algorithm::<u16> {
    poly: 0x8005,
    init: 0xFFFF,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0,
    residue: 0,
};

static mut Algo32: Algorithm<u32> = Algorithm::<u32> {
    poly: 0x04c11db7,
    init: 0,
    refin: true,
    refout: true,
    xorout: 0xFFFFFFFF,
    check: 0,
    residue: 0,
};
