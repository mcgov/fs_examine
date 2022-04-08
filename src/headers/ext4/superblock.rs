use crate::headers::reader::{bitfield_fetch, uuid_deserialize};
use serde::Deserialize;
use serde_big_array::BigArray;
use uuid::Uuid;

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
    pub error_action: u16, //What to do when an error is detectedBehaviour when detecting errors. One of:
    // 1	Continue
    // 2	Remount read-only
    // 3	Panic
    pub version_minor: u16, //Minor portion of version (combine with Major portion below to construct full version field)
    pub last_check: u32,    //POSIX time of last consistency check (fsck)
    pub check_interval: u32, //Interval (in POSIX time) between forced consistency checks (fsck)
    pub creator_os: u32, //Operating system ID from which the filesystem on this volume was created (see below)
    //0	Linux
    // 1	Hurd
    // 2	Masix
    // 3	FreeBSD
    // 4	Lites
    pub version_major: u32, //Major portion of version (combine with Minor portion above to construct full version field)
    pub default_uid_owner: u16, //User ID that can use reserved blocks
    pub default_gid_owner: u16, //Group ID that can use reserved blocks
    pub first_ino: u32,     // First non-reserved inode.
    pub inode_size: u16,    // Size of inode structure, in bytes.
    pub block_group_nr: u16, // 	Block group # of this superblock.
    pub feature_compat: u32, //Compatible feature set flags. Kernel can still read/write this fs even if it doesn't understand a flag; e2fsck will not attempt to fix a filesystem with any unknown COMPAT flags. Any of:
    pub feature_incompat: u32,
    pub feature_ro_compat: u32, //	Readonly-compatible feature set. If the kernel doesn't understand one of these bits, it can still mount read-only, but e2fsck will refuse to modify the filesystem. Any of:
    #[serde(deserialize_with = "uuid_deserialize")]
    pub uuid: Uuid, //	128-bit UUID for volume.char
    pub volume_name: [u8; 16],  // Volume label.char
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
    pub blocks_count_hi: u32,  // High 32-bits of the block count.
    pub r_blocks_count_hi: u32, // High 32-bits of the reserved block count.
    pub free_blocks_count_hi: u32, // High 32-bits of the free block count.
    pub min_extra_isize: u16,  // All inodes have at least # bytes.
    pub want_extra_isize: u16, // New inodes should reserve # bytes.
    pub flags: u32,            // Miscellaneous flags. Any of:
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
    pub fn volume_name(&self) -> String {
        std::string::String::from_utf8(self.volume_name.to_vec()).unwrap()
    }
    pub fn mount_opts(&self) -> String {
        std::string::String::from_utf8(self.mount_opts.to_vec()).unwrap()
    }
    pub fn last_mounted(&self) -> String {
        std::string::String::from_utf8(self.last_mounted.to_vec()).unwrap()
    }
}
