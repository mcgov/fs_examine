use serde::Deserialize;

// __(le|u)([0-9]+)\s+([a-z_]+)(.*)
//pub $3 : u$2, //$4

#[derive(Debug, Deserialize)]
pub struct BlockGroupDescriptor {
    pub block_bitmap_lo: u32,      // 	Lower 32-bits of location of block bitmap.
    pub inode_bitmap_lo: u32,      // 	Lower 32-bits of location of inode bitmap.
    pub inode_table_lo: u32,       // 	Lower 32-bits of location of inode table.
    pub free_blocks_count_lo: u16, // 	Lower 16-bits of free block count.
    pub free_inodes_count_lo: u16, // 	Lower 16-bits of free inode count.
    pub used_dirs_count_lo: u16,   // 	Lower 16-bits of directory count.
    pub flags: u16,                // 	Block group flags.
    pub exclude_bitmap_lo: u32,    // 	Lower 32-bits of location of snapshot exclusion bitmap.
    pub block_bitmap_csum_lo: u16, // 	Lower 16-bits of the block bitmap checksum.
    pub inode_bitmap_csum_lo: u16, // 	Lower 16-bits of the inode bitmap checksum.
    pub itable_unused_lo: u16, // 	Lower 16-bits of unused inode count. If set, we needn't scan past the (sb.s_inodes_per_group - gdt.bg_itable_unused)th entry in the inode table for this group.
    pub checksum: u16, // 	Group descriptor checksum; crc16(sb_uuid+group+desc) if the RO_COMPAT_GDT_CSUM feature is set, or crc32c(sb_uuid+group_desc) & 0xFFFF if the RO_COMPAT_METADATA_CSUM feature is set.
    //These fields only exist if the 64bit feature is enabled and s_desc_size > 32.
    pub block_bitmap_hi: u32, // 	Upper 32-bits of location of block bitmap.
    pub inode_bitmap_hi: u32, // 	Upper 32-bits of location of inodes bitmap.
    pub inode_table_hi: u32,  // 	Upper 32-bits of location of inodes table.
    pub free_blocks_count_hi: u16, // 	Upper 16-bits of free block count.
    pub free_inodes_count_hi: u16, // 	Upper 16-bits of free inode count.
    pub used_dirs_count_hi: u16, // 	Upper 16-bits of directory count.
    pub itable_unused_hi: u16, // 	Upper 16-bits of unused inode count.
    pub exclude_bitmap_hi: u32, // 	Upper 32-bits of location of snapshot exclusion bitmap.
    pub block_bitmap_csum_hi: u16, // 	Upper 16-bits of the block bitmap checksum.
    pub inode_bitmap_csum_hi: u16, // 	Upper 16-bits of the inode bitmap checksum.
    pub reserved: u32,        // 	Padding to 64 bytes.
}
