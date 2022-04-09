use crate::headers::reader::*;
use serde;
use serde::de;
use serde::de::SeqAccess;
use serde::{Deserialize, Deserializer};

// __(le|u)([0-9]+)\s+([a-z_]+)(.*)
//pub $3 : u$2, //$4
#[derive(Debug)]
pub struct DirEnt {
    inode: u32,   // 	Number of the inode that this directory entry points to.
    dirlen: u16,  // 	Length of this directory entry.
    namelen: u8,  // 	Length of the file name.
    filetype: u8, // 	File type code, one of:
    filename: String,
}

pub fn get_dir_ent(bytes: &[u8]) -> DirEnt {
    let namelen = bytes[6];
    let dir: [u8; 2] = [bytes[4], bytes[5]];
    let dirlen = u16::from_le_bytes(dir);
    let inode: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    let inode = u32::from_le_bytes(inode);
    let filetype = bytes[7];
    println!("namelen: {:x}", namelen);
    let filename = String::from_utf8(bytes[..namelen as usize].to_vec()).unwrap();
    DirEnt {
        inode: inode,
        dirlen: dirlen,
        namelen: namelen,
        filetype: filetype,
        filename: filename,
    }
}

pub mod file_type {
    pub const UNKNOWN: u8 = 0x0;
    pub const REGULAR_FILE: u8 = 0x1;
    pub const DIRECTORY: u8 = 0x2;
    pub const CHAR_DEV: u8 = 0x3; // 	Character device file.
    pub const BLOCK_DEV: u8 = 0x4; // 	Block device file.
    pub const FIFO: u8 = 0x5; // 	FIFO.
    pub const SOCKET: u8 = 0x6; // 	Socket.
    pub const SYMLINK: u8 = 0x7; //	Symbolic link.
}

pub fn filename_deserializer<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok("".to_string())
}
