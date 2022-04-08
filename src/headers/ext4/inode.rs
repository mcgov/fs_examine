pub struct Inode {
    pub i_mode: u16,        //
    pub i_uid: u16,         // 	Lower 16-bits of Owner UID.
    pub i_size_lo: u32,     // 	Lower 32-bits of size in bytes.
    pub i_atime: u32, // 	Last access time, in seconds since the epoch. However, if the EA_INODE inode flag is set, this inode stores an extended attribute value and this field contains the checksum of the value.
    pub i_ctime: u32, //Last inode change time, in seconds since the epoch. However, if the EA_INODE inode flag is set, this inode stores an extended attribute value and this field contains the lower 32 bits of the attribute values reference count.
    pub i_mtime: u32, // 	Last data modification time, in seconds since the epoch. However, if the EA_INODE inode flag is set, this inode stores an extended attribute value and this field contains the number of the inode that owns the extended attribute.
    pub i_dtime: u32, // 	Deletion Time, in seconds since the epoch.
    pub i_gid: u16,   // 	Lower 16-bits of GID.
    pub i_links_count: u16, //Hard link count. Normally, ext4 does not permit an inode to have more than 65,000 hard links. This applies to files as well as directories, which means that there cannot be more than 64,998 subdirectories in a directory (each subdirectory's '..' entry counts as a hard link, as does the '.' entry in the directory itself). With the DIR_NLINK feature enabled, ext4 supports more than 64,998 subdirectories by setting this field to 1 to indicate that the number of hard links is not known.
    pub i_blocks_lo: u32, //Lower 32-bits of "block" count. If the huge_file feature flag is not set on the filesystem, the file consumes i_blocks_lo 512-byte blocks on disk. If huge_file is set and EXT4_HUGE_FILE_FL is NOT set in inode.i_flags, then the file consumes i_blocks_lo + (i_blocks_hi << 32) 512-byte blocks on disk. If huge_file is set and EXT4_HUGE_FILE_FL IS set in inode.i_flags, then this file consumes (i_blocks_lo + i_blocks_hi << 32) filesystem blocks on disk.
    pub i_flags: u32,     //
    pub union_osd1: [u8; 4],
    pub i_block: [u8; 60], //i_block[EXT4_N_BLOCKS=15] 	Block map or extent tree. See the section "The Contents of inode.i_block".
    pub i_generation: u32, // 	File version (for NFS).
    pub i_file_acl_lo: u32, // 	Lower 32-bits of extended attribute block. ACLs are of course one of many possible extended attributes; I think the name of this field is a result of the first use of extended attributes being for ACLs.
    pub i_size_high: u32, // aka i_dir_acl 	Upper 32-bits of file/directory size. In ext2/3 this field was named i_dir_acl, though it was usually set to zero and never used.
    pub i_obso_faddr: u32, // 	(Obsolete) fragment address.
    pub osd2: [u8; 12],   // union
    pub i_extra_isize: u16, // 	Size of this inode - 128. Alternately, the size of the extended inode fields beyond the original ext2 inode, including this field.
    pub i_checksum_hi: u16, // 	Upper 16-bits of the inode checksum.
    pub i_ctime_extra: u32, // 	Extra change time bits. This provides sub-second precision. See Inode Timestamps section.
    pub i_mtime_extra: u32, // 	Extra modification time bits. This provides sub-second precision.
    pub i_atime_extra: u32, //Extra access time bits. This provides sub-second precision.
    pub i_crtime: u32,      // 	File creation time, in seconds since the epoch.
    pub i_crtime_extra: u32, // 	Extra file creation time bits. This provides sub-second precision.
    pub i_version_hi: u32,  // 	Upper 32-bits for version number.
    pub i_projid: u32,      // 	Project ID.
}

// (0x[0-9A-Z]+)\s+([A-Z_]+)\s(.*)
// pub $2  : u16 = $1; //$3
// File mode. Any of:
pub mod filemode_bitflags {
    pub mod any {
        pub const S_IXOTH: u16 = 0x1; //(Others may execute)
        pub const S_IWOTH: u16 = 0x2; //(Others may write)
        pub const S_IROTH: u16 = 0x4; //(Others may read)
        pub const S_IXGRP: u16 = 0x8; //(Group members may execute)
        pub const S_IWGRP: u16 = 0x10; //(Group members may write)
        pub const S_IRGRP: u16 = 0x20; //(Group members may read)
        pub const S_IXUSR: u16 = 0x40; //(Owner may execute)
        pub const S_IWUSR: u16 = 0x80; //(Owner may write)
        pub const S_IRUSR: u16 = 0x100; //(Owner may read)
        pub const S_ISVTX: u16 = 0x200; //(Sticky bit)
        pub const S_ISGID: u16 = 0x400; //(Set GID)
        pub const S_ISUID: u16 = 0x800; //(Set UID)
    }
    pub mod mutex {
        pub const S_IFIFO: u16 = 0x1000; //(FIFO)
        pub const S_IFCHR: u16 = 0x2000; //(Character device)
        pub const S_IFDIR: u16 = 0x4000; //(Directory)
        pub const S_IFBLK: u16 = 0x6000; //(Block device)
        pub const S_IFREG: u16 = 0x8000; //(Regular file)
        pub const S_IFLNK: u16 = 0xA000; //(Symbolic link)
        pub const S_IFSOCK: u16 = 0xC000; //(Socket)
    }
}
