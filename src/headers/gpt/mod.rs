use partitions::PartitionEntry;
use serde::Deserialize;
use serde_big_array::BigArray;
pub mod partitions;
pub mod uuids;
use crate::headers::constants::SMOL_BLOCKS;
use crate::headers::reader::*;
use colored::*;

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

impl Gpt {
    pub fn print_partition_table(&self, file_arg: &str) {
        let mut unused_counter = 0;
        for i in 0..self.gpe_table_entries as u64 {
            let entry = read_header_from_offset::<PartitionEntry>(
                &file_arg,
                self.gpe_table_start * SMOL_BLOCKS + i * self.gpe_table_entry_size as u64,
            );
            if entry.is_in_use() {
                println!("{}", entry.name());
                println!("{:x?}", entry);
                println!("{:?}", entry.type_to_str());
            } else {
                unused_counter += 1;
            }
        }

        println!(
            "{}",
            format!("skipped {} unused partition entries", unused_counter).blue()
        );
    }

    pub fn get_parition(&self, file_arg: &str, index: u32) -> PartitionEntry {
        read_header_from_offset::<PartitionEntry>(
            &file_arg,
            self.gpe_table_start * SMOL_BLOCKS + (index * self.gpe_table_entry_size) as u64,
        )
    }
}
