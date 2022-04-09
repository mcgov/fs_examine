use crate::headers::reader::*;
use colored::*;
use serde;
use serde::de;
use serde::de::SeqAccess;
use serde::{Deserialize, Deserializer};

// __(le|u)([0-9]+)\s+([a-z_]+)(.*)
//pub $3 : u$2, //$4
#[derive(Debug)]
pub struct DirEnt {
    inode: u32,   // 	Number of the inode that this directory entry points to.
    rec_len: u16, // 	Length of this directory entry.
    namelen: u8,  // 	Length of the file name.
    filetype: u8, // 	File type code, one of:
    filename: String,
}

pub fn get_dir_ent(bytes: &[u8]) -> DirEnt {
    let dir: [u8; 2] = [bytes[4], bytes[5]];
    let rec_len_u = u16::from_le_bytes(dir);
    let inode_arr: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    let inode_u = u32::from_le_bytes(inode_arr);
    println!("namelen: {:x}", bytes[6]);
    let filename = String::from_utf8(bytes[..bytes[6] as usize].to_vec()).unwrap();
    DirEnt {
        inode: inode_u,
        rec_len: rec_len_u,
        namelen: bytes[6],
        filetype: bytes[7],
        filename: filename,
    }
}

impl DirEnt {
    pub fn filetype_to_str(&self) -> String {
        let ft: &str;
        match self.filetype {
            file_type::UNKNOWN => {
                return stringify!(file_type::UNKNOWN).purple().to_string();
            }
            file_type::REGULAR_FILE => {
                ft = stringify!(file_type::REGULAR_FILE);
            }
            file_type::DIRECTORY => {
                ft = stringify!(file_type::DIRECTORY);
            }
            file_type::CHAR_DEV => {
                ft = stringify!(file_type::CHAR_DEV);
            }
            file_type::BLOCK_DEV => {
                ft = stringify!(file_type::BLOCK_DEV);
            }
            file_type::FIFO => {
                ft = stringify!(file_type::FIFO);
            }
            file_type::SOCKET => {
                ft = stringify!(file_type::SOCKET);
            }
            file_type::SYMLINK => {
                ft = stringify!(file_type::SYMLINK);
            }
            x => {
                panic!("Error, unknown filetype for Dirent: {:X}", x);
            }
        }

        ft.green().to_string()
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
    pub const FAKE_TAIL_ENTRY_CHECKSUM: u8 = 0xDE;
}

pub fn filename_deserializer<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok("".to_string())
}
