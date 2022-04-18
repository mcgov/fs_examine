use super::block_group::*;
use super::extattrs::*;
use super::extent::*;
use super::inode::*;
use super::superblock::Superblock;
use super::*;
use crate::headers::reader::OnDisk;
use colored::*;

pub struct Part {
    pub reader: OnDisk,
    pub start: u64,
    pub s: Superblock,
    pub bg: Vec<Bg>,
}
pub mod part;

pub struct Bg {
    pub start: u64,
    pub b32: Option<BlockGroupDescriptor32>,
    pub b64: Option<BlockGroupDescriptor64>,
    pub ino: Vec<Ino>,
}
pub mod bg;

pub struct Ino {
    pub start: u64,
    pub id: u32,
    pub inode: Inode,
    pub attr: Option<Exatt>,
    pub extent: Option<ExtentTree>,
    pub seed: u32,
    // can also have a hash tree
}
pub mod ino;

pub struct Exatt {
    blk: ExtendedAttrBlock,
    attrs: Vec<ExtendedAttrEntry>,
}

pub struct PartReader {
    pub reader: OnDisk,
    pub first: Part,
    pub blocks: std::collections::HashMap<u64, Part>,
    pub fs_start: u64,
    pub fs_size: u64,
}

impl Part {
    pub fn populate_blocks(&mut self) {
        self.populate_block_groups();
        self.validate_block_groups();
        self.populate_inodes();
    }
}
