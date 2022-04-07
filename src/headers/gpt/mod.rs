use serde::Deserialize;
use serde_big_array::BigArray;

pub mod partitions;
pub mod uuids;

#[derive(Deserialize, Debug)]
pub struct Gpt {
    pub signature: [u8; 8], //	Signature, can be identified by 8 bytes magic "EFI PART" (45h 46h 49h 20h 50h 41h 52h 54h)
    pub revision: [u8; 4],  //	GPT Revision
    pub size: u32,          //	Header size
    pub crc32: u32,         //	CRC32 checksum of the GPT header
    pub reserved: [u8; 4],  //	Reserved
    pub self_lba: u64,      //	The LBA containing this header
    pub alt_lba: u64,       //	The LBA of the alternate GPT header
    pub first_usable_block: u64, //	The first usable block that can be contained in a GPT entry
    pub last_usable_block: u64, //	The last usable block that can be contained in a GPT entry
    pub guid: [u8; 16],     //	GUID of the disk
    pub gpe_table_start: u64, //	Starting LBA of the GUID Partition Entry array
    pub gpe_table_entries: u32, //	Number of Partition Entries
    pub gpe_table_entry_size: u32, //	Size (in bytes) of each entry in the Partition Entry array - must be a value of 128×2ⁿ where n ≥ 0 (in the past, multiples of 8 were acceptable)
    pub gpe_table_crc32: u32,      //	CRC32 of the Partition Entry array.
    #[serde(with = "BigArray")]
    pub also_reserved: [u8; 512 - 0x5c], // Reserved (should be zeroed) 512-0x5c is 420 btw lmaoooo
}
