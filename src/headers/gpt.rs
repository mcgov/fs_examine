use crate::headers::disx86::disassemble;
use crate::headers::reader::{le_u16_deserialize,le_u32_deserialize};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize};
use serde_big_array::BigArray;
use std::fmt;
use std::str;

#[derive(Deserialize)]
pub struct Gpt {

    signature: [u8;	8],//	Signature, can be identified by 8 bytes magic "EFI PART" (45h 46h 49h 20h 50h 41h 52h 54h)
    revision: [u8;4], //	GPT Revision
    size : u32, //	Header size
    crc32: [u8;4], //	CRC32 checksum of the GPT header
    reserved: [u8;4], //	Reserved
    self_lba: u64, //	The LBA containing this header
    alt_lba: u64, //	The LBA of the alternate GPT header
    first_usable_block: u64, //	The first usable block that can be contained in a GPT entry
    last_usable_block: u64, //	The last usable block that can be contained in a GPT entry
    guid: [u8;16], //	GUID of the disk
    gpe_table_start : u64, //	Starting LBA of the GUID Partition Entry array
    gpe_table_entries: u32, //	Number of Partition Entries
    gpe_table_entry_size: u32, //	Size (in bytes) of each entry in the Partition Entry array - must be a value of 128×2ⁿ where n ≥ 0 (in the past, multiples of 8 were acceptable)
    gpe_table_crc32: [u8;4], //	CRC32 of the Partition Entry array.
    reserved: [u8:512-0x5c],	// Reserved (should be zeroed) 512-0x5c is 420 btw lmaoooo
}
