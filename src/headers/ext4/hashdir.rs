use colored::*;
use serde::Deserialize;
#[derive(Deserialize, Copy, Clone, Debug)]
#[repr(packed)]
pub struct Root {
    dot_inode: u32, // 	inode number of this directory.
    dot_rec_len: u16, // 	Length of this record, 12.
    dot_name_len: u8, // 	Length of the name, 1.
    dot_file_type: u8, /* 	File type of this entry, 0x2
                     * (directory) (if the feature flag
                     * is set). */
    dot_name: [u8; 4], //".\0\0\0"
    dotdot_inode: u32, /* 	inode number of parent
                        * directory. */
    dotdot_rec_len: u16, /* 	block_size - 12. The record
                          * length is long enough to
                          * cover all htree data. */
    dotdot_name_len: u8, // 	Length of the name, 2.
    dotdot_file_type: u8, /* 	File type of this entry, 0x2
                          * (directory) (if the feature
                          * flag is set). */
    dotdot_name: [u8; 4], // 	"..\0\0"
    root_info: RootInfo,
    limit: u16, /* 	Maximum number of dx_entries that can
                 * follow this header, plus 1 for the
                 * header itself. */
    count: u16, /* 	Actual number of dx_entries that
                 * follow this header, plus 1 for the
                 * header itself. */
    block: u32, /* 	The block number (within the
                 * directory file) that goes with
                 * hash=0.
                 * entries follow
                 *struct dx_entry 	entries[0] 	As many
                 * 8-byte struct dx_entry as fits in the
                 * rest of the data block. */
}

impl Root {
    pub fn hash_version(&self) -> hash_versions::HashVer {
        self.root_info.hash_version()
    }
}

macro_rules! validate_field {
    ($field:expr,$value:expr) => {
        if $field != $value {
            println!(
                "Error: {:#?} != {:#?}",
                $field, $value
            );
            return false;
        }
    };
}
impl Root {
    pub fn validate(&self, _bs: u64) -> bool {
        let drec = self.dot_rec_len;
        validate_field!(drec, 12);
        let name_len = self.dot_name_len;
        validate_field!(name_len, 1);
        validate_field!(self.dot_file_type, 2);
        validate_field!(self.dot_name, [b'.', 0, 0, 0]);
        let ddrec = self.dotdot_rec_len;
        validate_field!(ddrec, 12);
        validate_field!(self.dotdot_name_len, 2);
        validate_field!(self.dotdot_file_type, 2);
        validate_field!(
            self.dotdot_name,
            [b'.', b'.', 0, 0]
        );
        if !self.root_info.validate() {
            println!("root info didn't validate!");
            return false;
        }

        println!(
            "{}",
            "he'll yea brother root valleydatored".green()
        );
        true
    }
}

#[derive(Deserialize, Copy, Clone, Debug)]
#[repr(packed)]
pub struct RootInfo {
    reserved_zero: u32, // 	Zero.
    hash_version: u8,   // hash versions
    info_length: u8,    /* 	Length of the tree
                         * information, 0x8. */
    indirect_levels: u8, /* 	Depth of the htree. Cannot
                          * be larger than 3 if the
                          * INCOMPAT_LARGEDIR feature is
                          * set; cannot be larger than 2
                          * otherwise. */
    unused_flags: u8, //
}

impl RootInfo {
    pub fn hash_version(&self) -> hash_versions::HashVer {
        hash_versions::val_to_enum
            [self.hash_version as usize]
            .clone()
    }
}
pub mod hash_versions {
    pub const SLEGACY: u8 = 0;
    pub const SHALF_MD4: u8 = 1;
    pub const STEA: u8 = 2;
    pub const ULEGACY: u8 = 3;
    pub const UHALF_MD4: u8 = 4;
    pub const UTEA: u8 = 5;
    pub const SIPHASH: u8 = 6;

    #[derive(Clone, Debug)]
    pub enum HashVer {
        Legacy,
        HalfMd4,
        Tea,
        ULegacy,
        Utea,
        UhalfMd4,
        SipHash,
    }

    pub const val_to_enum: [HashVer; 7] = [
        HashVer::Legacy,
        HashVer::HalfMd4,
        HashVer::Tea,
        HashVer::ULegacy,
        HashVer::Utea,
        HashVer::UhalfMd4,
        HashVer::SipHash,
    ];
}
impl RootInfo {
    pub fn validate(&self) -> bool {
        true
    }
}
#[derive(Deserialize, Copy, Clone, Debug)]
#[repr(packed)]
pub struct Node {
    fake_inode: u32, /* 	Zero, to make it look like this
                      * entry is not in use. */
    fake_rec_len: u16, /* 	The size of the block, in
                        * order to hide all of the
                        * dx_node data. */
    name_len: u8, /* 	Zero. There is no name for this
                   * "unused" directory entry. */
    file_type: u8, /* 	Zero. There is no file type for
                    * this "unused" directory entry. */
    limit: u16, /* 	Maximum number of dx_entries that can
                 * follow this header, plus 1 for the
                 * header itself. */
    count: u16, /* 	Actual number of dx_entries that
                 * follow this header, plus 1 for the
                 * header itself. */
    block: u32, /* The block number (within the directory file) that goes with the lowest hash value of this block. This value is stored in the parent block. */
}
#[derive(Deserialize, Copy, Clone, Debug)]
#[repr(packed)]
pub struct Entry {
    hash: u32, // 	Hash code.
    block: u32, /* 	Block number (within the directory
                * file, not filesystem blocks) of the
                * next node in the htree. */
}
#[derive(Deserialize, Copy, Clone, Debug)]
#[repr(packed)]
pub struct Tail {
    reserved: u32,
    csum: u32, /* uuid,htree index header, all indices
                * that are in use, and tail block */
}

/*
mod offsets{
0x0
0x4
0x6
0x7
0x8
0xC
0x10
0x12
0x13
0x14
0x18
0x1C
0x0
0x1
0x2
0x3
0x4
0x5
0x1D
0x1E
0x1F
0x20
0x22
0x24
0x28
}*/
