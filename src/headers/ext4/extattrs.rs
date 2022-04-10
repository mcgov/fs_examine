use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExtendedAttrBlock {
    pub magic: u32,    // 	Magic number for identification, 0xEA020000.
    pub refcount: u32, // 	Reference count.
    pub blocks: u32,   // 	Number of disk blocks used.
    pub hash: u32,     // 	Hash value of all attributes.
    pub checksum: u32, // 	Checksum of the extended attribute block.
    pub reserved: [u32; 3], //[2] //this is documented to be 32 bytes but that would add
                       // an extra field to reserved... hm.
}
pub const MAGIC: u32 = 0xEA020000;

#[derive(Debug)]
pub struct ExtendedAttrEntry {
    pub name_len: u8,    // 	Length of name.
    pub name_index: u8,  // 	Attribute name index. There is a discussion of this below.
    pub value_offs: u16, // 	Location of this attribute's value on the disk block where it is stored. Multiple attributes can share the same value. For an inode attribute this value is relative to the start of the first entry; for a block this value is relative to the start of the block (i.e. the header).
    pub value_inum: u32, // 	The inode where the value is stored. Zero indicates the value is in the same block as this entry. This field is only used if the INCOMPAT_EA_INODE feature is enabled.
    pub value_size: u32, // 	Length of attribute value.
    pub hash: u32, // 	Hash value of attribute name and attribute value. The kernel doesn't update the hash for in-inode attributes, so for that case this value must be zero, because e2fsck validates any non-zero hash regardless of where the xattr lives.
    pub name: String, //[e_name_len] 	Attribute name. Does not include trailing NULL.
}

pub const EXTATTR_ENTRY_SIZE_WO_NAME: u64 = 16;

pub fn get_extended_attr_entry(bytes: &[u8]) -> ExtendedAttrEntry {
    let value_offs_ = u16::from_le_bytes([bytes[2], bytes[3]]);
    let value_inum_ = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let value_size_ = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let hash_ = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
    let filename_ = String::from_utf8(bytes[16..(16 + bytes[0] as usize)].to_vec()).unwrap();
    ExtendedAttrEntry {
        name_len: bytes[0],
        name_index: bytes[1],
        value_offs: value_offs_,
        value_inum: value_inum_,
        value_size: value_size_,
        hash: hash_,
        name: filename_,
    }
}

impl ExtendedAttrEntry {
    pub fn is_empty(&self) -> bool {
        self.name_len == 0 && self.name_index == 0 && self.value_offs == 0 && self.value_inum == 0
    }
}
