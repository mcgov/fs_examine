use crate::headers::reader::*;
use crate::prettify_output;
use colored::*;
use serde::Deserialize;

// __(le|u)([0-9]+)\s+([a-z_]+)(.*)
//pub $3 : u$2, //$4

// This shit is variable length depending on the bitness

#[derive(Debug, Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct BlockGroupDescriptor32 {
  pub block_bitmap_lo: u32, /* 	Lower 32-bits of location of block
                             * bitmap. */
  pub inode_bitmap_lo: u32, /* 	Lower 32-bits of location of inode
                             * bitmap. */
  pub inode_table_lo: u32, /* 	Lower 32-bits of location of inode
                            * table. */
  pub free_blocks_count_lo: u16, /* 	Lower 16-bits of free block
                                  * count. */
  pub free_inodes_count_lo: u16, /* 	Lower 16-bits of free inode
                                  * count. */
  pub used_dirs_count_lo: u16, // 	Lower 16-bits of directory count.
  pub flags: u16,              // 	Block group flags.
  pub exclude_bitmap_lo: u32,  /* 	Lower 32-bits of location of
                                * snapshot exclusion bitmap. */
  pub block_bitmap_csum_lo: u16, /* 	Lower 16-bits of the block
                                  * bitmap checksum. */
  pub inode_bitmap_csum_lo: u16, /* 	Lower 16-bits of the inode
                                  * bitmap checksum. */
  pub itable_unused_lo: u16, /* 	Lower 16-bits of unused inode
                              * count. If set, we needn't scan
                              * past the (sb.s_inodes_per_group -
                              * gdt.bg_itable_unused)th entry in
                              * the inode table for this group. */
  pub checksum: u16, /* 	Group descriptor checksum;
                      * crc16(sb_uuid+group+desc) if the
                      * RO_COMPAT_GDT_CSUM feature is set, or
                      * crc32c(sb_uuid+group_desc) & 0xFFFF if the
                      * RO_COMPAT_METADATA_CSUM feature is set. */
}

impl BlockGroupDescriptor32 {
  pub fn pretty_print(&self, index: u64) {
    prettify_output!(
      BlockGroupDescriptor32,
      purple,
      bright_purple,
      {
        println!("BGD {}: {:x?}", index, self);
        self.print_flags();
      }
    );
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
  pub fn inodes_uninit(&self) -> bool {
    bitfield_fetch::<u16>(self.flags, bg_flags::EXT4_BG_INODE_UNINIT)
  }
  pub fn is_uninitialized(&self) -> bool {
    bitfield_fetch::<u16>(self.flags, bg_flags::EXT4_BG_INODE_UNINIT)
      && bitfield_fetch::<u16>(
        self.flags,
        bg_flags::EXT4_BG_BLOCK_UNINIT,
      )
  }
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct BlockGroupDescriptor64 {
  pub block_bitmap_hi: u32, /* 	Upper 32-bits of location of block
                             * bitmap. */
  pub inode_bitmap_hi: u32, /* 	Upper 32-bits of location of inodes
                             * bitmap. */
  pub inode_table_hi: u32, /* 	Upper 32-bits of location of inodes
                            * table. */
  pub free_blocks_count_hi: u16, /* 	Upper 16-bits of free block
                                  * count. */
  pub free_inodes_count_hi: u16, /* 	Upper 16-bits of free inode
                                  * count. */
  pub used_dirs_count_hi: u16, // 	Upper 16-bits of directory count.
  pub itable_unused_hi: u16,   /* 	Upper 16-bits of unused inode
                                * count. */
  pub exclude_bitmap_hi: u32, /* 	Upper 32-bits of location of
                               * snapshot exclusion bitmap. */
  pub block_bitmap_csum_hi: u16, /* 	Upper 16-bits of the block
                                  * bitmap checksum. */
  pub inode_bitmap_csum_hi: u16, /* 	Upper 16-bits of the inode
                                  * bitmap checksum. */
  pub reserved: u32, // 	Padding to 64 bytes.
}

pub mod bg_flags {
  pub const EXT4_BG_INODE_UNINIT: u16 = 0x1; //	inode table and bitmap are not initialized
                                             // (EXT4_BG_INODE_UNINIT).
  pub const EXT4_BG_BLOCK_UNINIT: u16 = 0x2; // 	block bitmap is not initialized (EXT4_BG_BLOCK_UNINIT).
  pub const EXT4_BG_INODE_ZEROED: u16 = 0x4; //	inode table is zeroed
                                             // (EXT4_BG_INODE_ZEROED).
                                             //
}
