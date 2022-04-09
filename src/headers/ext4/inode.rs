use crate::headers::reader::*;
use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
pub struct Inode {
    pub mode: u16,              //
    pub uid: u16,               // 	Lower 16-bits of Owner UID.
    pub size_lo: u32,           // 	Lower 32-bits of size in bytes.
    pub atime: u32, // 	Last access time, in seconds since the epoch. However, if the EA_INODE inode flag is set, this inode stores an extended attribute value and this field contains the checksum of the value.
    pub ctime: u32, //Last inode change time, in seconds since the epoch. However, if the EA_INODE inode flag is set, this inode stores an extended attribute value and this field contains the lower 32 bits of the attribute values reference count.
    pub mtime: u32, // 	Last data modification time, in seconds since the epoch. However, if the EA_INODE inode flag is set, this inode stores an extended attribute value and this field contains the number of the inode that owns the extended attribute.
    pub dtime: u32, // 	Deletion Time, in seconds since the epoch.
    pub gid: u16,   // 	Lower 16-bits of GID.
    pub links_count: u16, //Hard link count. Normally, ext4 does not permit an inode to have more than 65,000 hard links. This applies to files as well as directories, which means that there cannot be more than 64,998 subdirectories in a directory (each subdirectory's '..' entry counts as a hard link, as does the '.' entry in the directory itself). With the DIR_NLINK feature enabled, ext4 supports more than 64,998 subdirectories by setting this field to 1 to indicate that the number of hard links is not known.
    pub blocks_lo: u32, //Lower 32-bits of "block" count. If the huge_file feature flag is not set on the filesystem, the file consumes blocks_lo 512-byte blocks on disk. If huge_file is set and EXT4_HUGE_FILE_FL is NOT set in inode.flags, then the file consumes blocks_lo + (blocks_hi << 32) 512-byte blocks on disk. If huge_file is set and EXT4_HUGE_FILE_FL IS set in inode.flags, then this file consumes (blocks_lo + blocks_hi << 32) filesystem blocks on disk.
    pub flags: u32,     //
    pub ext_attr_refcount: u32, //sorry hurd
    #[serde(with = "BigArray")]
    pub block: [u8; 60], //block[EXT4_N_BLOCKS=15] 	Block map or extent tree. See the section "The Contents of inode.block".
    pub generation: u32,    // 	File version (for NFS).
    pub file_acl_lo: u32, // 	Lower 32-bits of extended attribute block. ACLs are of course one of many possible extended attributes; I think the name of this field is a result of the first use of extended attributes being for ACLs.
    pub size_high: u32, // aka dir_acl 	Upper 32-bits of file/directory size. In ext2/3 this field was named dir_acl, though it was usually set to zero and never used.
    pub obso_faddr: u32, // 	(Obsolete) fragment address.
    pub osd2: [u8; 12], // union
    pub blocks_high: u16, // 	Upper 16-bits of the block count. Please see the note attached to i_blocks_lo.
    pub file_acl_high: u16, // 	Upper 16-bits of the extended attribute block (historically, the file ACL location). See the Extended Attributes section below.
    pub uid_high: u16,      // 	Upper 16-bits of the Owner UID.
    pub gid_high: u16,      // 	Upper 16-bits of the GID.
    pub checksum_lo: u16,   // 	Lower 16-bits of the inode checksum.
    pub reserved: u16,      // 	Unused.
    pub extra_isize: u16, // 	Size of this inode - 128. Alternately, the size of the extended inode fields beyond the original ext2 inode, including this field.
    pub checksum_hi: u16, // 	Upper 16-bits of the inode checksum.
    pub ctime_extra: u32, // 	Extra change time bits. This provides sub-second precision. See Inode Timestamps section.
    pub mtime_extra: u32, // 	Extra modification time bits. This provides sub-second precision.
    pub atime_extra: u32, //Extra access time bits. This provides sub-second precision.
    pub crtime: u32,      // 	File creation time, in seconds since the epoch.
    pub crtime_extra: u32, // 	Extra file creation time bits. This provides sub-second precision.
    pub version_hi: u32,  // 	Upper 32-bits for version number.
    pub projid: u32,      // 	Project ID.
}

impl Inode {
    pub fn print_times(&self) {
        println!("accessed: {:#?}", timestamp_to_string(self.atime as u64));
        println!("created: {:#?}", timestamp_to_string(self.crtime as u64));
        println!("modified: {:#?}", timestamp_to_string(self.mtime as u64));
        println!("deleted: {:#?}", timestamp_to_string(self.dtime as u64));
    }

    pub fn inode_uses_extants(&self) -> bool {
        bitfield_fetch::<u32>(self.flags, attr_bitflags::EXT4_EXTENTS_FL)
    }
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

pub mod attr_bitflags {
    pub const EXT4_SECRM_FL: u32 = 0x1; //This file requires secure deletion . (not implemented)
    pub const EXT4_UNRM_FL: u32 = 0x2; //This file should be preserved, should undeletion be desired . (not implemented)
    pub const EXT4_COMPR_FL: u32 = 0x4; //File is compressed . (not really implemented)
    pub const EXT4_SYNC_FL: u32 = 0x8; //All writes to the file must be synchronous .
    pub const EXT4_IMMUTABLE_FL: u32 = 0x10; //File is immutable .
    pub const EXT4_APPEND_FL: u32 = 0x20; //File can only be appended .
    pub const EXT4_NODUMP_FL: u32 = 0x40; //The dump(1) utility should not dump this file .
    pub const EXT4_NOATIME_FL: u32 = 0x80; //Do not update access time .
    pub const EXT4_DIRTY_FL: u32 = 0x100; //Dirty compressed file . (not used)
    pub const EXT4_COMPRBLK_FL: u32 = 0x200; //File has one or more compressed clusters . (not used)
    pub const EXT4_NOCOMPR_FL: u32 = 0x400; //Do not compress file . (not used)
    pub const EXT4_ENCRYPT_FL: u32 = 0x800; //Encrypted inode . This bit value previously was EXT4_ECOMPR_FL (compression error), which was never used.
    pub const EXT4_INDEX_FL: u32 = 0x1000; //Directory has hashed indexes .
    pub const EXT4_IMAGIC_FL: u32 = 0x2000; //AFS magic directory .
    pub const EXT4_JOURNAL_DATA_FL: u32 = 0x4000; //File data must always be written through the journal .
    pub const EXT4_NOTAIL_FL: u32 = 0x8000; //File tail should not be merged . (not used by ext4)
    pub const EXT4_DIRSYNC_FL: u32 = 0x10000; //All directory entry data should be written synchronously (see dirsync) .
    pub const EXT4_TOPDIR_FL: u32 = 0x20000; //Top of directory hierarchy .
    pub const EXT4_HUGE_FILE_FL: u32 = 0x40000; //This is a huge file .
    pub const EXT4_EXTENTS_FL: u32 = 0x80000; //Inode uses extents .
    pub const EXT4_EA_INODE_FL: u32 = 0x200000; //Inode stores a large extended attribute value in its data blocks .
    pub const EXT4_EOFBLOCKS_FL: u32 = 0x400000; // deprecated This file has blocks allocated past EOF ().
    pub const EXT4_SNAPFILE_FL: u32 = 0x01000000; //Inode is a snapshot . (not in mainline)
    pub const EXT4_SNAPFILE_DELETED_FL: u32 = 0x04000000; //Snapshot is being deleted . (not in mainline)
    pub const EXT4_SNAPFILE_SHRUNK_FL: u32 = 0x08000000; //Snapshot shrink has completed . (not in mainline)
    pub const EXT4_INLINE_DATA_FL: u32 = 0x10000000; //Inode has inline data .
    pub const EXT4_PROJINHERIT_FL: u32 = 0x20000000; //Create children with the same project ID .
    pub const EXT4_RESERVED_FL: u32 = 0x80000000; //Reserved for ext4 library .
                                                  //Aggregate flags:
    pub const EXT4_USER_VISIBLE_MASK: u32 = 0x4BDFFF; //User-visible flags.
    pub const EXT4_USER_MODIFIEABLE_MASK: u32 = 0x4B80FF; //User-modifiable flags. Note that while EXT4_JOURNAL_DATA_FL and EXT4_EXTENTS_FL can be set with setattr, they are not in the kernel's EXT4_FL_USER_MODIFIABLE mask, since it needs to handle the setting of these flags in a special manner and they are masked out of the set of flags that are saved directly to i_flags.
}
