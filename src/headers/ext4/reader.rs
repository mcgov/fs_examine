use super::block_group::*;
use super::dirent::*;
use super::extattrs::*;
use super::extent::*;
use super::inode::*;
use super::superblock::*;
use super::*;

pub struct reader {
    pub s: Superblock,
    pub bg: Vec<bg>,
}

pub struct bg {
    pub is64: bool,
    pub b32: BlockGroupDescriptor32,
    pub b64: BlockGroupDescriptor64,
    pub ino: Vec<ino>,
}

pub struct ino {
    pub node: Inode,
    pub attr: ExtendedAttrBlock,
    pub extents: Vec<extent>,
    // can also have a hash tree
}

pub struct extent {
    pub hdr: ExtentHeader,
    pub branches: Vec<ExtentNode>,
    pub leafs: Vec<ExtentLeaf>,
    pub tail: ExtentTail,
}
