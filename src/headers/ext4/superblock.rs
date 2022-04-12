use crate::headers::reader::*;
use crate::prettify_output;
use colored::*;
use serde::Deserialize;
use serde_big_array::BigArray;
use std::fmt::Debug;
use uuid::Uuid;

// source of truth: https://ext4.wiki.kernel.org/index.php/Ext4_Disk_Layout
// also used https://wiki.osdev.org/Ext4

// wow this block sure is big buddy
#[derive(Deserialize, Debug)]
pub struct Superblock {
    pub inodes_count: u32,         //Total number of inodes in file system
    pub blocks_count_lo: u32,      //Total number of blocks in file system
    pub r_blocks_count_lo: u32,    //Number of reserved blocks
    pub free_blocks_count_lo: u32, //Total number of unallocated blocks
    pub free_inodes_count: u32,    //Total number of unallocated inodes
    pub superblock: u32, //Block number of the block containing the superblock. This is 1 on 1024 byte block size filesystems, and 0 for all others.
    pub log_block_size: u32, //log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    pub log_cluster_size: u32, //log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    pub blocks_per_group: u32, //Number of blocks in each block group
    pub clusters_per_group: u32, //Number of fragments in each block group
    pub inodes_per_group: u32, //Number of inodes in each block group
    pub mount_time: u32,       //Last mount time (in POSIX time)
    pub last_write_time: u32,  //Last written time (in POSIX time)
    pub mnt_count: u16, //Number of times the volume has been mounted since its last consistency check (fsck)
    pub max_mnt_count: u16, //Number of mounts allowed before a consistency check (fsck) must be done
    pub magic: u16, //Magic signature (0xef53), used to help confirm the presence of Ext4 on a volume
    pub fs_state: u16, //File system state.
    // Behaviour when detecting errors. One of:
    // 1	Continue
    // 2	Remount read-only
    // 3	Panic
    pub error_action: u16,
    pub version_minor: u16, //Minor portion of version (combine with Major portion below to construct full version field)
    pub last_check: u32,    //POSIX time of last consistency check (fsck)
    pub check_interval: u32, //Interval (in POSIX time) between forced consistency checks (fsck)
    pub creator_os: u32, //Operating system ID from which the filesystem on this volume was created (see below)
    // 0	Linux
    // 1	Hurd
    // 2	Masix
    // 3	FreeBSD
    // 4	Lites
    pub version_major: u32, //Major portion of version (combine with Minor portion above to construct full version field)
    // is either 0 or 1 for dynamic inode revision
    pub default_uid_owner: u16, //User ID that can use reserved blocks
    pub default_gid_owner: u16, //Group ID that can use reserved blocks
    /*
    These following fields are EXT4_DYNAMIC_REV superblocks only.
    */
    pub first_ino: u32,      // First non-reserved inode.
    pub inode_size: u16,     // Size of inode structure, in bytes.
    pub block_group_nr: u16, // 	Block group # of this superblock.
    pub feature_compat: u32, //Compatible feature set flags. Kernel can still read/write this fs even if it doesn't understand a flag; e2fsck will not attempt to fix a filesystem with any unknown COMPAT flags. Any of:
    pub feature_incompat: u32,
    pub feature_ro_compat: u32, //	Readonly-compatible feature set. If the kernel doesn't understand one of these bits, it can still mount read-only, but e2fsck will refuse to modify the filesystem. Any of:
    #[serde(deserialize_with = "uuid_deserialize")]
    pub uuid: Uuid, //	128-bit UUID for volume
    pub volume_name: [u8; 16],  // Volume label
    #[serde(with = "BigArray")]
    pub last_mounted: [u8; 64], //Directory where filesystem was last mounted.
    pub algorithm_usage_bitmap: u32, //	For compression (Not used in e2fsprogs/Linux)
    pub prealloc_blocks: u8, //	# of blocks to try to preallocate for ... files? (Not used in e2fsprogs/Linux)
    pub prealloc_dir_blocks: u8, //	# of blocks to preallocate for directories. (Not used in e2fsprogs/Linux)
    pub reserved_gdt_blocks: u16, //	Number of reserved GDT entries for future filesystem expansion.
    #[serde(deserialize_with = "uuid_deserialize")]
    pub journal_uuid: Uuid, //	UUID of journal superblock
    pub journal_inum: u32,       //	inode number of journal file.
    pub journal_dev: u32, //	Device number of journal file, if the external journal feature flag is set.
    pub last_orphan: u32, //	Start of list of orphaned inodes to delete.
    pub hash_seed: [u32; 4], //	HTREE hash seed.
    pub def_hash_version: u8, //	Default hash algorithm to use for directory hashes. One of:
    pub jnl_backup_type: u8, //	If this value is 0 or EXT3_JNL_BACKUP_BLOCKS (1), then the s_jnl_blocks field contains a duplicate copy of the inode's i_block[] array and i_size.
    pub desc_size: u16, //	Size of group descriptors, in bytes, if the 64bit incompat feature flag is set.
    pub default_mount_opts: u32, // Default mount options. Any of:
    pub first_meta_bg: u32, // First metablock block group, if the meta_bg feature is enabled.
    pub mkfs_time: u32, // When the filesystem was created, in seconds since the epoch.
    pub jnl_blocks: [u32; 17], // Backup copy of the journal inode's i_block[] array in the first 15 elements and i_size_high and i_size in the 16th and 17th elements, respectively.
    /*
    AND THESE following fields are valid if used on a 64 bit system.
    */
    pub blocks_count_hi: u32,         // High 32-bits of the block count.
    pub r_blocks_count_hi: u32,       // High 32-bits of the reserved block count.
    pub free_blocks_count_hi: u32,    // High 32-bits of the free block count.
    pub min_extra_isize: u16,         // All inodes have at least # bytes.
    pub want_extra_isize: u16,        // New inodes should reserve # bytes.
    pub flags: u32,                   // Miscellaneous flags. Any of:
    pub raid_stride: u16, // RAID stride. This is the number of logical blocks read from or written to the disk before moving to the next disk. This affects the placement of filesystem metadata, which will hopefully make RAID storage faster.
    pub mmp_interval: u16, // # seconds to wait in multi-mount prevention (MMP) checking. In theory, MMP is a mechanism to record in the superblock which host and device have mounted the filesystem, in order to prevent multiple mounts. This feature does not seem to be implemented...
    pub mmp_block: u64,    // Block # for multi-mount protection data.
    pub raid_stripe_width: u32, // RAID stripe width. This is the number of logical blocks read from or written to the disk before coming back to the current disk. This is used by the block allocator to try to reduce the number of read-modify-write operations in a RAID5/6.
    pub log_groups_per_flex: u8, // Size of a flexible block group is 2 ^ s_log_groups_per_flex.
    pub checksum_type: u8, // Metadata checksum algorithm type. The only valid value is 1 (crc32c).
    pub reserved_pad: u16, //
    pub kbytes_written: u64, //	Number of KiB written to this filesystem over its lifetime.
    pub snapshot_inum: u32, // inode number of active snapshot. (Not used in e2fsprogs/Linux.)
    pub snapshot_id: u32,  // Sequential ID of active snapshot. (Not used in e2fsprogs/Linux.)
    pub snapshot_r_blocks_count: u64, // Number of blocks reserved for active snapshot's future use. (Not used in e2fsprogs/Linux.)
    pub snapshot_list: u32, // inode number of the head of the on-disk snapshot list. (Not used in e2fsprogs/Linux.)
    pub error_count: u32,   // Number of errors seen.
    pub first_error_time: u32, // First time an error happened, in seconds since the epoch.
    pub first_error_ino: u32, // inode involved in first error.
    pub first_error_block: u64, // Number of block involved of first error.
    pub first_error_func: [u8; 32], // Name of function where the error happened.
    pub first_error_line: u32, // Line number where error happened.
    pub last_error_time: u32, // Time of most recent error, in seconds since the epoch.
    pub last_error_ino: u32, // inode involved in most recent error.
    pub last_error_line: u32, // Line number where most recent error happened.
    pub last_error_block: u64, // Number of block involved in most recent error.
    pub last_error_func: [u8; 32], // Name of function where the most recent error happened.
    #[serde(with = "BigArray")]
    pub mount_opts: [u8; 64], // ASCIIZ string of mount options.
    pub usr_quota_inum: u32, // Inode number of user quota file.
    pub grp_quota_inum: u32, // Inode number of group quota file.
    pub overhead_blocks: u32, // Overhead blocks/clusters in fs. (Huh? This field is always zero, which means that the kernel calculates it dynamically.)
    pub backup_bgs: [u32; 2], // Block groups containing superblock backups (if sparse_super2)
    pub encrypt_algos: [u8; 4], // Encryption algorithms in use. There can be up to four algorithms in use at any time; valid algorithm codes are given below:
    pub encrypt_pw_salt: [u8; 16], // Salt for the string2key algorithm for encryption.
    pub lpf_ino: u32,           // Inode number of lost+found
    pub prj_quota_inum: u32,    // Inode that tracks project quotas.
    pub checksum_seed: u32, // Checksum seed used for metadata_csum calculations. This value is crc32c(~0, $orig_fs_uuid).
    #[serde(with = "BigArray")]
    pub reserved: [u32; 98], // Padding to the end of the block.
    pub checksum: u32,      //Superblock checksum.
}

impl Superblock {
    pub fn number_of_groups(&self) -> u64 {
        let mut total_blocks: u64 = self.blocks_count_lo as u64;
        if self.uses_64bit() {
            total_blocks |= (self.blocks_count_hi as u64) << 32;
        }
        total_blocks / self.blocks_per_group as u64
    }
    pub fn block_size_bytes(&self) -> u64 {
        1024 << self.log_block_size
    }

    pub fn get_group_descriptor_table_offset(&self, first_lba: u64) -> u64 {
        get_offset_from_block_number(
            first_lba * crate::headers::constants::SMOL_BLOCKS,
            1 + self.superblock as u64,
            self.block_size_bytes(),
        )
    }

    pub fn volume_name(&self) -> String {
        std::string::String::from_utf8(self.volume_name.to_vec()).unwrap()
    }
    pub fn mount_opts(&self) -> String {
        std::string::String::from_utf8(self.mount_opts.to_vec()).unwrap()
    }
    pub fn last_mounted(&self) -> String {
        std::string::String::from_utf8(self.last_mounted.to_vec()).unwrap()
    }
    pub fn first_error_func(&self) -> String {
        std::string::String::from_utf8(self.first_error_func.to_vec()).unwrap()
    }
    pub fn last_error_func(&self) -> String {
        std::string::String::from_utf8(self.last_error_func.to_vec()).unwrap()
    }

    pub fn uses_64bit(&self) -> bool {
        bitfield_fetch(self.feature_incompat, breaks_compat::USES_64BIT)
    }
    pub fn uses_flex_bg(&self) -> bool {
        bitfield_fetch(self.feature_incompat, breaks_compat::USES_FLEX_BG)
    }
    pub fn uses_ext_attr(&self) -> bool {
        bitfield_fetch(self.feature_compat, compat_bitflags::COMPAT_EXT_ATTR)
    }
    pub fn uses_mmp(&self) -> bool {
        bitfield_fetch(self.feature_incompat, breaks_compat::USES_MMP)
    }
    pub fn uses_journal(&self) -> bool {
        bitfield_fetch(self.feature_compat, compat_bitflags::COMPAT_HAS_JOURNAL)
    }
    pub fn flex_bg_size(&self) -> u64 {
        1 << self.log_groups_per_flex
    }

    pub fn debug_print_some_stuf(&self) {
        prettify_output!(Superblock, purple, bright_purple, {
            println!("{:x?}", self);
            println!(
                "Inodes in use: {}",
                self.inodes_count - self.free_inodes_count
            );
            println!(
                "Blocks in use: {}",
                self.blocks_count_lo - self.free_blocks_count_lo
            );
            println!(
                "Blocks in use: {}",
                self.blocks_count_lo - self.free_blocks_count_lo
            );

            println!("volume name: {}", self.volume_name());
            println!("mount opts: {}", self.mount_opts());
            println!("last mounted: {}", self.last_mounted());
            println!("first_error: {}", self.first_error_func());
            println!(
                "last check : {}",
                timestamp_to_string(self.last_check as u64)
            );
            println!(
                "Ext4 Dynamic rev?: {}",
                print_bool(self.version_major == constants::EXT4_DYNAMIC_REV)
            );
            println!("64bit_support : {}", print_bool(self.uses_64bit()));
            println!("Ext Attrs : {}", print_bool(self.uses_ext_attr()));
            println!("Flex BG : {}", print_bool(self.uses_flex_bg()));
            println!("MMP : {}", self.uses_mmp());
            println!("Journal (internal) : {}", self.uses_journal());
            println!(
                "FlexBG Size: val: {} size 0x{:X?}",
                self.log_groups_per_flex,
                self.flex_bg_size()
            );
            println!(
                "Uses EA Inode present?: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_incompat,
                    breaks_compat::USES_EA_INODE
                ))
            );
            println!(
                "Inline Data present?: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_incompat,
                    breaks_compat::USES_INLINE_DATA
                ))
            );
            println!(
                "ROCompat Extra Isize info present?: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_ro_compat,
                    compat_readonly::RO_COMPAT_EXTRA_ISIZE
                ))
            );
            println!(
                "Uses dirdata: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_incompat,
                    breaks_compat::USES_DIRDATA
                ))
            );
            println!(
                "FILETYPE flag set: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_incompat,
                    breaks_compat::USES_FILETYPE
                ))
            );
            println!(
                "META_BG flag set: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_incompat,
                    breaks_compat::USES_META_BG
                ))
            );
            println!(
                "Huge Inodes?: {}",
                print_bool(bitfield_fetch::<u32>(
                    self.feature_compat,
                    compat_readonly::RO_COMPAT_HUGE_FILE
                ))
            );
            println!("EXT4 BlockSize (decimal): {}", self.block_size_bytes());
        });
    }
}

pub mod constants {
    pub const EXT4_GOOD_OLD_REV: u32 = 0; /* The good old (original) format */
    pub const EXT4_DYNAMIC_REV: u32 = 1;
}

//(0x[0-9]+)(.*)\(([A-Z0-9_]+)\)
//const $3 : u32 = $1; //$2
pub mod hashalgo_bitflags {
    // Default hash algorithm to use for directory hashes. One of:
    pub const LEGACY: u8 = 0x0; // 	Legacy.
    pub const HALF_MD4: u8 = 0x1; // 	Half MD4.
    pub const TEA: u8 = 0x2;
    pub const UNSIGNED_LEGACY: u8 = 0x3; // 	Legacy, unsigned.
    pub const HALF_MD4_UNSIGNED: u8 = 0x4; // 	Half MD4, unsigned.
    pub const TEA_UNSIGNED: u8 = 0x5;
}

pub mod compat_bitflags {
    //optional feature set flags. Kernel can still read/write this fs even if it doesn't understand a flag; e2fsck will not attempt to fix a filesystem with any unknown COMPAT flags. Any of:
    pub const COMPAT_DIR_PREALLOC: u32 = 0x1; // Directory preallocation (COMPAT_DIR_PREALLOC).
    pub const COMPAT_IMAGIC_INODES: u32 = 0x2; //"imagic inodes". Used by AFS to indicate inodes that are not linked into the directory namespace. Inodes marked with this flag will not be added to lost+found by e2fsck. (COMPAT_IMAGIC_INODES).
    pub const COMPAT_HAS_JOURNAL: u32 = 0x4; // 	Has a journal (COMPAT_HAS_JOURNAL).
    pub const COMPAT_EXT_ATTR: u32 = 0x8; // 	Supports extended attributes (COMPAT_EXT_ATTR).
    pub const COMPAT_RESIZE_INODE: u32 = 0x10; // 	Has reserved GDT blocks for filesystem expansion. Requires RO_COMPAT_SPARSE_SUPER. (COMPAT_RESIZE_INODE).
    pub const COMPAT_DIR_INDEX: u32 = 0x20; //Has indexed directories. (COMPAT_DIR_INDEX).
    pub const COMPAT_LAZY_BG: u32 = 0x40; // 	"Lazy BG". Not in Linux kernel, seems to have been for uninitialized block groups? (COMPAT_LAZY_BG).
    pub const COMPAT_EXCLUDE_INODE: u32 = 0x80; // 	"Exclude inode". Intended for filesystem snapshot feature, but not used. (COMPAT_EXCLUDE_INODE).
    pub const COMPAT_EXCLUDE_BITMAP: u32 = 0x100; // 	"Exclude bitmap". Seems to be used to indicate the presence of snapshot-related exclude bitmaps? Not defined in kernel or used in e2fsprogs. (COMPAT_EXCLUDE_BITMAP).
    pub const COMPAT_SPARSE_SUPER2: u32 = 0x200; // 	Sparse Super Block, v2. If this flag is set, the SB field s_backup_bgs points to the two block groups that contain backup superblocks. (COMPAT_SPARSE_SUPER2).
}
pub mod compat_readonly {
    // Readonly-compatible feature set. If the kernel doesn't understand one of these bits, it can still mount read-only, but e2fsck will refuse to modify the filesystem. Any of:
    pub const RO_COMPAT_SPARSE_SUPER: u32 = 0x1; // 	Sparse superblocks. See the earlier discussion of this feature. .
    pub const RO_COMPAT_LARGE_FILE: u32 = 0x2; // 	Allow storing files larger than 2GiB .
    pub const RO_COMPAT_BTREE_DIR: u32 = 0x4; // 	Was intended for use with htree directories, but was not needed. Not used in kernel or e2fsprogs .
    pub const RO_COMPAT_HUGE_FILE: u32 = 0x8; // 	This filesystem has files whose space usage is stored in i_blocks in units of filesystem blocks, not 512-byte sectors. Inodes using this feature will be marked with EXT4_INODE_HUGE_FILE.
    pub const RO_COMPAT_GDT_CSUM: u32 = 0x10; // 	Group descriptors have checksums. In addition to detecting corruption, this is useful for lazy formatting with uninitialized groups .
    pub const RO_COMPAT_DIR_NLINK: u32 = 0x20; // 	Indicates that the old ext3 32,000 subdirectory limit no longer applies. A directory's i_links_count will be set to 1 if it is incremented past 64,999. .
    pub const RO_COMPAT_EXTRA_ISIZE: u32 = 0x40; // 	Indicates that large inodes exist on this filesystem, storing extra fields after EXT2_GOOD_OLD_INODE_SIZE. .
    pub const RO_COMPAT_HAS_SNAPSHOT: u32 = 0x80; // 	This filesystem has a snapshot. Not implemented in ext4. .
    pub const RO_COMPAT_QUOTA: u32 = 0x100; // 	Quota is handled transactionally with the journal .
    pub const RO_COMPAT_BIGALLOC: u32 = 0x200; // 	This filesystem supports "bigalloc", which means that filesystem block allocation bitmaps are tracked in units of clusters (of blocks) instead of blocks .
    pub const RO_COMPAT_METADATA_CSUM: u32 = 0x400; // 	This filesystem supports metadata checksumming. (RO_COMPAT_METADATA_CSUM; implies RO_COMPAT_GDT_CSUM, though GDT_CSUM must not be set)
    pub const RO_COMPAT_REPLICA: u32 = 0x800; // 	Filesystem supports replicas. This feature is neither in the kernel nor e2fsprogs. .
    pub const RO_COMPAT_READONLY: u32 = 0x1000; // 	Read-only filesystem image; the kernel will not mount this image read-write and most tools will refuse to write to the image. .
    pub const RO_COMPAT_PROJECT: u32 = 0x2000; // 	Filesystem tracks project quotas.
}

pub mod breaks_compat {
    //NOTE: linux names these INCOMPAT_ which confuses my puny brain.
    // The flag is set when the feature is present,if the FS doesn't support it, it will fail to mount.
    // I'm naming them USES because I have to read them when I'm tired.
    // Actual docs not follows: Incompatible feature set. If the kernel or e2fsck doesn't understand one of these bits, it will refuse to mount or attempt to repair the filesystem. Any of:
    pub const USES_COMPRESSION: u32 = 0x1; // 	Compression. Not implemented. .
    pub const USES_FILETYPE: u32 = 0x2; // 	Directory entries record the file type. See ext4_dir_entry_2 below. .
    pub const USES_RECOVER: u32 = 0x4; // 	Filesystem needs journal recovery. .
    pub const USES_JOURNAL_DEV: u32 = 0x8; // 	Filesystem has a separate journal device. .
    pub const USES_META_BG: u32 = 0x10; // 	Meta block groups. See the earlier discussion of this feature. .
    pub const USES_EXTENTS: u32 = 0x40; // 	Files in this filesystem use extents. .
    pub const USES_64BIT: u32 = 0x80; //Enable a filesystem size over 2^32 blocks. (INCOMPAT_64BIT).
    pub const USES_MMP: u32 = 0x100; // 	Multiple mount protection. Prevent multiple hosts from mounting the filesystem concurrently by updating a reserved block periodically while mounted and checking this at mount time to determine if the filesystem is in use on another host. .
    pub const USES_FLEX_BG: u32 = 0x200; // 	Flexible block groups. See the earlier discussion of this feature. .
    pub const USES_EA_INODE: u32 = 0x400; // 	Inodes can be used to store large extended attribute values .
    pub const USES_DIRDATA: u32 = 0x1000; // 	Data in directory entry. Allow additional data fields to be stored in each dirent, after struct ext4_dirent. The presence of extra data is indicated by flags in the high bits of ext4_dirent file type flags (above EXT4_FT_MAX). The flag EXT4_DIRENT_LUFID = 0x10 is used to store a 128-bit File Identifier for Lustre. The flag EXT4_DIRENT_IO64 = 0x20 is used to store the high word of 64-bit inode numbers. Feature still in development. .
    pub const USES_CSUM_SEED: u32 = 0x2000; // 	Metadata checksum seed is stored in the superblock. This feature enables the administrator to change the UUID of a metadata_csum filesystem while the filesystem is mounted; without it, the checksum definition requires all metadata blocks to be rewritten. .
    pub const USES_LARGEDIR: u32 = 0x4000; // 	Large directory >2GB or 3-level htree. Prior to this feature, directories could not be larger than 4GiB and could not have an htree more than 2 levels deep. If this feature is enabled, directories can be larger than 4GiB and have a maximum htree depth of 3. .
    pub const USES_INLINE_DATA: u32 = 0x8000; // 	Data in inode. Small files or directories are stored directly in the inode i_blocks and/or xattr space. .
    pub const USES_ENCRYPT: u32 = 0x10000; // 	Encrypted inodes are present on the filesystem .
}

pub mod ext4mount_bitflags {
    //Default mount options. Any of:
    pub const EXT4_DEFM_DEBUG: u32 = 0x0001; // 	Print debugging info upon mount.
    pub const EXT4_DEFM_BSDGROUPS: u32 = 0x0002; // 	New files take the gid of the containing directory (instead of the fsgid of the current process).
    pub const EXT4_DEFM_XATTR_USER: u32 = 0x0004; // 	Support userspace-provided extended attributes.
    pub const EXT4_DEFM_ACL: u32 = 0x0008; // 	Support POSIX access control lists aka ACLs.
    pub const EXT4_DEFM_UID16: u32 = 0x0010; // 	Do not support 32-bit UIDs.
    pub const EXT4_DEFM_JMODE_DATA: u32 = 0x0020; // 	All data and metadata are commited to the journal.
    pub const EXT4_DEFM_JMODE_ORDERED: u32 = 0x0040; // 	All data are flushed to the disk before metadata are committed to the journal.
    pub const EXT4_DEFM_JMODE_WBACK: u32 = 0x0060; // 	Data ordering is not preserved; data may be written after the metadata has been written.
    pub const EXT4_DEFM_NOBARRIER: u32 = 0x0100; // 	Disable write flushes.
    pub const EXT4_DEFM_BLOCK_VALIDITY: u32 = 0x0200; // 	Track which blocks in a filesystem are metadata and therefore should not be used as data blocks. This option will be enabled by default on 3.18, hopefully.
    pub const EXT4_DEFM_DISCARD: u32 = 0x0400; // 	Enable DISCARD support, where the storage device is told about blocks becoming unused.
    pub const EXT4_DEFM_NODELALLOC: u32 = 0x0800; // 	Disable delayed allocation.
}

pub mod cipher_bitflags {
    //Encryption algorithms in use. There can be up to four algorithms in use at any time; valid algorithm codes are given below:
    pub const ENCRYPTION_MODE_INVALID: u32 = 0; // 	Invalid algorithm .
    pub const ENCRYPTION_MODE_AES_256_XTS: u32 = 1; // 	256-bit AES in XTS mode .
    pub const ENCRYPTION_MODE_AES_256_GCM: u32 = 2; // 	256-bit AES in GCM mode .
    pub const ENCRYPTION_MODE_AES_256_CBC: u32 = 3; // 	256-bit AES in CBC mode .
}

impl HasHeaderMagic for Superblock {
    fn magic_field_endianness(&self) -> Endianness {
        return Endianness::Little;
    }
    fn magic_field_offset(&self) -> u64 {
        0x38
    }
    fn magic_field_size(&self) -> u64 {
        2
    }
    fn magic_field_upcast(&self) -> u128 {
        0xef53
    }
}

pub const BLOCK_0_PADDING: u64 = 1024;
