use crate::headers::reader::*;
use crate::prettify_output;
use colored::*;
use serde::Deserialize;
// __(le|u)([0-9]+)\s+([a-z_]+)(.*)
//pub $3 : u$2, //$4

// This shit is variable length depending on the bitness

#[derive(Debug, Deserialize)]
pub struct BlockGroupDescriptor32 {
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
}

impl BlockGroupDescriptor32 {
  pub fn pretty_print(&self, index: u64) {
    prettify_output!(BlockGroupDescriptor32, purple, bright_purple, {
      println!("BGD {}: {:x?}", index, self);
      self.print_flags();
    });
  }
  pub fn print_flags(&self) {
    println!(
      "Inodes initialized?: {}",
      print_bool(!bitfield_fetch::<u16>(
        self.flags,
        bg_flags::EXT4_BG_INODE_UNINIT
      ))
    );
    println!(
      "Blocks initialized?: {}",
      print_bool(!bitfield_fetch::<u16>(
        self.flags,
        bg_flags::EXT4_BG_BLOCK_UNINIT
      ))
    );
    println!(
      "Inode table is zeroed?: {}",
      print_bool(bitfield_fetch::<u16>(
        self.flags,
        bg_flags::EXT4_BG_INODE_ZEROED
      ))
    );
  }
  pub fn is_uninitialized(&self) -> bool {
    bitfield_fetch::<u16>(self.flags, bg_flags::EXT4_BG_INODE_UNINIT)
      && bitfield_fetch::<u16>(self.flags, bg_flags::EXT4_BG_BLOCK_UNINIT)
      && bitfield_fetch::<u16>(self.flags, bg_flags::EXT4_BG_INODE_ZEROED)
  }
}

pub struct BlockGroupDescriptor64 {
  pub low_fields: BlockGroupDescriptor32,
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

pub mod bg_flags {
  pub const EXT4_BG_INODE_UNINIT: u16 = 0x1; //	inode table and bitmap are not initialized (EXT4_BG_INODE_UNINIT).
  pub const EXT4_BG_BLOCK_UNINIT: u16 = 0x2; // 	block bitmap is not initialized (EXT4_BG_BLOCK_UNINIT).
  pub const EXT4_BG_INODE_ZEROED: u16 = 0x4; //	inode table is zeroed (EXT4_BG_INODE_ZEROED).
}

/*
Filesystem volume name:   doopes
Last mounted on:          /run/media/fox/doopes
Filesystem UUID:          281ed3c4-fc24-413e-8d40-b3c967190178
Filesystem magic number:  0xEF53
Filesystem revision #:    1 (dynamic)
Filesystem features:      has_journal ext_attr resize_inode dir_index filetype needs_recovery extent flex_bg sparse_super large_file huge_file dir_nlink extra_isize metadata_csum
Filesystem flags:         signed_directory_hash
Default mount options:    user_xattr acl
Filesystem state:         clean
Errors behavior:          Continue
Filesystem OS type:       Linux
Inode count:              32768
Block count:              131072
Reserved block count:     6553
Overhead clusters:        9773
Free blocks:              121283
Free inodes:              32756
First block:              1
Block size:               1024
Fragment size:            1024
Reserved GDT blocks:      256
Blocks per group:         8192
Fragments per group:      8192
Inodes per group:         2048
Inode blocks per group:   256
Flex block group size:    16
Filesystem created:       Wed Apr  6 14:45:04 2022
Last mount time:          Thu Apr  7 21:32:12 2022
Last write time:          Thu Apr  7 21:32:12 2022
Mount count:              4
Maximum mount count:      -1
Last checked:             Wed Apr  6 14:45:04 2022
Check interval:           0 (<none>)
Lifetime writes:          4416 kB
Reserved blocks uid:      0 (user root)
Reserved blocks gid:      0 (group root)
First inode:              11
Inode size:               128
Journal inode:            8
Default directory hash:   half_md4
Directory Hash Seed:      99cd3d82-d7fd-481f-875a-a77d481d11eb
Journal backup:           inode blocks
Checksum type:            crc32c
Checksum:                 0xa36225e4
Journal features:         journal_checksum_v3
Total journal size:       4096k
Total journal blocks:     4096
Max transaction length:   4096
Fast commit length:       0
Journal sequence:         0x00000009
Journal start:            0
Journal checksum type:    crc32c
Journal checksum:         0x378804fe


Group 0: (Blocks 0x0001-0x2000) csum 0x7635 [ITABLE_ZEROED]
  Primary superblock at 0x0001, Group descriptors at 0x0002-0x0002
  Reserved GDT blocks at 0x0003-0x0102
  Block bitmap at 0x0103 (+258), csum 0x00004497
  Inode bitmap at 0x0113 (+274), csum 0x00005df5
  Inode table at 0x0123-0x0222 (+290)
  3791 free blocks, 2036 free inodes, 2 directories, 2036 unused inodes
  Free blocks: 0x1132-0x2000
  Free inodes: 0x000d-0x0800
Group 1: (Blocks 0x2001-0x4000) csum 0x7895 [INODE_UNINIT, ITABLE_ZEROED]
  Backup superblock at 0x2001, Group descriptors at 0x2002-0x2002
  Reserved GDT blocks at 0x2003-0x2102
  Block bitmap at 0x0104 (bg #0 + 259), csum 0x00004738
  Inode bitmap at 0x0114 (bg #0 + 275), csum 0x00000000
  Inode table at 0x0223-0x0322 (bg #0 + 546)
  7933 free blocks, 2048 free inodes, 0 directories, 2048 unused inodes
  Free blocks: 0x2104-0x4000
  Free inodes: 0x0801-0x1000
Group 2: (Blocks 0x4001-0x6000) csum 0x6b7d [INODE_UNINIT, BLOCK_UNINIT, ITABLE_ZEROED]
  Block bitmap at 0x0105 (bg #0 + 260), csum 0x00000000
  Inode bitmap at 0x0115 (bg #0 + 276), csum 0x00000000
  Inode table at 0x0323-0x0422 (bg #0 + 802)
  8192 free blocks, 2048 free inodes, 0 directories, 2048 unused inodes
  Free blocks: 0x4001-0x6000
  Free inodes: 0x1001-0x1800


*/
