#[derive(Deserialize, Debug)]
struct Superblock {
    #[serde(deserialize_with = "le_u32_deserialize")]
    inodes: u32, //Total number of inodes in file system
    #[serde(deserialize_with = "le_u32_deserialize")]
    blocks: u32, //Total number of blocks in file system
    #[serde(deserialize_with = "le_u32_deserialize")]
    reserved_blocks: u32, //Number of reserved blocks
    #[serde(deserialize_with = "le_u32_deserialize")]
    blocks_left: u32, //Total number of unallocated blocks
    #[serde(deserialize_with = "le_u32_deserialize")]
    inodes_left: u32, //Total number of unallocated inodes
    #[serde(deserialize_with = "le_u32_deserialize")]
    superblock: u32, //Block number of the block containing the superblock. This is 1 on 1024 byte block size filesystems, and 0 for all others.
    #[serde(deserialize_with = "le_u32_deserialize")]
    block_shift: u32, //log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    #[serde(deserialize_with = "le_u32_deserialize")]
    fragment_shift: u32, //log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    #[serde(deserialize_with = "le_u32_deserialize")]
    group_blocks: u32, //Number of blocks in each block group
    #[serde(deserialize_with = "le_u32_deserialize")]
    group_fragments: u32, //Number of fragments in each block group
    #[serde(deserialize_with = "le_u32_deserialize")]
    group_inodes: u32, //Number of inodes in each block group
    #[serde(deserialize_with = "le_u32_deserialize")]
    last_mount_at: u32, //Last mount time (in POSIX time)
    #[serde(deserialize_with = "le_u32_deserialize")]
    last_written_at: u32, //Last written time (in POSIX time)
    #[serde(deserialize_with = "le_u16_deserialize")]
    mounts_since_last_check: u16, //Number of times the volume has been mounted since its last consistency check (fsck)
    #[serde(deserialize_with = "le_u16_deserialize")]
    max_mounts_before_check: u16, //Number of mounts allowed before a consistency check (fsck) must be done
    #[serde(deserialize_with = "le_u16_deserialize")]
    magic: u16, //Magic signature (0xef53), used to help confirm the presence of Ext4 on a volume
    #[serde(deserialize_with = "le_u16_deserialize")]
    fs_state: u16, //File system state.
    #[serde(deserialize_with = "le_u16_deserialize")]
    error_action: u16, //What to do when an error is detected
    #[serde(deserialize_with = "le_u16_deserialize")]
    version_minor: u16, //Minor portion of version (combine with Major portion below to construct full version field)
    #[serde(deserialize_with = "le_u32_deserialize")]
    last_check_at: u32, //POSIX time of last consistency check (fsck)
    #[serde(deserialize_with = "le_u32_deserialize")]
    forced_check_interval: u32, //Interval (in POSIX time) between forced consistency checks (fsck)
    #[serde(deserialize_with = "le_u32_deserialize")]
    os_id: u32, //Operating system ID from which the filesystem on this volume was created (see below)
    #[serde(deserialize_with = "le_u32_deserialize")]
    version_major: u32, //Major portion of version (combine with Minor portion above to construct full version field)
    #[serde(deserialize_with = "le_u16_deserialize")]
    uid_owner: u16, //User ID that can use reserved blocks
    #[serde(deserialize_with = "le_u16_deserialize")]
    gid_owner: u16, //Group ID that can use reserved blocks
}
