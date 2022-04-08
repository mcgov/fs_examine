use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExtantHeader {
    pub eh_magic: u16,      // 	Magic number, 0xF30A.
    pub eh_entries: u16,    // 	Number of valid entries following the header.
    pub eh_max: u16,        // 	Maximum number of entries that could follow the header.
    pub eh_depth: u16, // 	Depth of this extent node in the extent tree. 0 = this extent node points to data blocks; otherwise, this extent node points to other extent nodes. The extent tree can be at most 5 levels deep: a logical block number can be at most 2^32, and the smallest n that satisfies 4*(((blocksize - 12)/12)^n) >= 2^32 is 5.
    pub eh_generation: u32, // 	Generation of the tree. (Used by Lustre, but not standard ext4).
}
#[derive(Deserialize, Debug)]
pub struct ExtantNode {
    pub ei_block: u32,   // 	This index node covers file blocks from 'block' onward.
    pub ei_leaf_lo: u32, // 	Lower 32-bits of the block number of the extent node that is the next level lower in the tree. The tree node pointed to can be either another internal node or a leaf node, described below.
    pub ei_leaf_hi: u16, // 	Upper 16-bits of the previous field.
    pub ei_unused: u16,  //
}
#[derive(Deserialize, Debug)]
pub struct ExtantLeaf {
    pub ee_block: u32,    // 	First file block number that this extent covers.
    pub ee_len: u16, // 	Number of blocks covered by extent. If the value of this field is <= 32768, the extent is initialized. If the value of the field is > 32768, the extent is uninitialized and the actual extent length is ee_len - 32768. Therefore, the maximum length of a initialized extent is 32768 blocks, and the maximum length of an uninitialized extent is 32767.
    pub ee_start_hi: u16, // 	Upper 16-bits of the block number to which this extent points.
    pub ee_start_lo: u32, // 	Lower 32-bits of the block number to which this extent points.
}
#[derive(Deserialize, Debug)]
pub struct ExtantTail {
    pub eb_checksum: u32, // 	Checksum of the extent block, crc32c(uuid+inum+igeneration+extentblock)
}
