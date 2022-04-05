use crate::headers::reader::{le_u32_deserialize, le_u64_deserialize};
use serde::Deserialize;
use serde_big_array::BigArray;
/*
GPT is also little endian. (according to apple, anyway)
https://developer.apple.com/library/archive/technotes/tn2166/_index.html#//apple_ref/doc/uid/DTS10003927-CH1-SECTION2

*/
#[derive(Deserialize)]
pub struct Gpt {
    signature: [u8; 8], //	Signature, can be identified by 8 bytes magic "EFI PART" (45h 46h 49h 20h 50h 41h 52h 54h)
    revision: [u8; 4],  //	GPT Revision
    #[serde(deserialize_with = "le_u32_deserialize")]
    size: u32, //	Header size
    #[serde(deserialize_with = "le_u32_deserialize")]
    crc32: u32, //	CRC32 checksum of the GPT header
    reserved: [u8; 4],  //	Reserved
    #[serde(deserialize_with = "le_u64_deserialize")]
    self_lba: u64, //	The LBA containing this header
    #[serde(deserialize_with = "le_u64_deserialize")]
    alt_lba: u64, //	The LBA of the alternate GPT header
    #[serde(deserialize_with = "le_u64_deserialize")]
    first_usable_block: u64, //	The first usable block that can be contained in a GPT entry
    #[serde(deserialize_with = "le_u64_deserialize")]
    last_usable_block: u64, //	The last usable block that can be contained in a GPT entry
    guid: [u8; 16],     //	GUID of the disk
    #[serde(deserialize_with = "le_u64_deserialize")]
    gpe_table_start: u64, //	Starting LBA of the GUID Partition Entry array
    #[serde(deserialize_with = "le_u32_deserialize")]
    gpe_table_entries: u32, //	Number of Partition Entries
    #[serde(deserialize_with = "le_u32_deserialize")]
    gpe_table_entry_size: u32, //	Size (in bytes) of each entry in the Partition Entry array - must be a value of 128×2ⁿ where n ≥ 0 (in the past, multiples of 8 were acceptable)
    #[serde(deserialize_with = "le_u32_deserialize")]
    gpe_table_crc32: u32, //	CRC32 of the Partition Entry array.
    #[serde(with = "BigArray")]
    also_reserved: [u8; 512 - 0x5c], // Reserved (should be zeroed) 512-0x5c is 420 btw lmaoooo
}
