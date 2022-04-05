use crate::headers::reader::{bitfield_fetch, le_u32_deserialize, le_u64_deserialize};
use byteorder::LittleEndian;
use serde::de;
use serde::{Deserialize, Deserializer};
use serde_big_array::BigArray;
use std::fmt;

/*
GPT is also little endian. (according to apple, anyway)
https://developer.apple.com/library/archive/technotes/tn2166/_index.html#//apple_ref/doc/uid/DTS10003927-CH1-SECTION2

*/
#[derive(Deserialize, Debug)]
pub struct Gpt {
    pub signature: [u8; 8], //	Signature, can be identified by 8 bytes magic "EFI PART" (45h 46h 49h 20h 50h 41h 52h 54h)
    pub revision: [u8; 4],  //	GPT Revision
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub size: u32, //	Header size
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub crc32: u32, //	CRC32 checksum of the GPT header
    pub reserved: [u8; 4],  //	Reserved
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub self_lba: u64, //	The LBA containing this header
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub alt_lba: u64, //	The LBA of the alternate GPT header
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub first_usable_block: u64, //	The first usable block that can be contained in a GPT entry
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub last_usable_block: u64, //	The last usable block that can be contained in a GPT entry
    pub guid: [u8; 16],     //	GUID of the disk
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub gpe_table_start: u64, //	Starting LBA of the GUID Partition Entry array
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub gpe_table_entries: u32, //	Number of Partition Entries
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub gpe_table_entry_size: u32, //	Size (in bytes) of each entry in the Partition Entry array - must be a value of 128×2ⁿ where n ≥ 0 (in the past, multiples of 8 were acceptable)
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub gpe_table_crc32: u32, //	CRC32 of the Partition Entry array.
    #[serde(with = "BigArray")]
    pub also_reserved: [u8; 512 - 0x5c], // Reserved (should be zeroed) 512-0x5c is 420 btw lmaoooo
}

#[derive(Deserialize, Debug)]
pub struct PartitionEntry {
    pub type_guid: [u8; 16],
    pub unique_guid: [u8; 16],
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub first_lba: u64,
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub last_lba: u64,
    attributes: Attributes,
    #[serde(with = "BigArray")]
    _name: [u16; 72 / 2],
}
#[derive(Debug)]
pub struct Attributes {
    pub container: u64,
    pub platform_essential: bool,
    pub efi_ignore: bool,
    pub legacy_bios_bootable: bool,
    pub reserved: [bool; 47],
    pub partition_reserved: [bool; 15],
}

struct AttributesVisitor;
impl<'de> Deserialize<'de> for Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Attributes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = AttributesVisitor {};
        let bitfield = deserializer.deserialize_u64(visitor).unwrap();
        let mut reserved_flags = [false; 47];
        for i in 0..47 {
            reserved_flags[i] = bitfield_fetch(bitfield, 0b1000 << i);
        }
        let mut partition_reserved = [false; 15];
        for i in 0..15 {
            partition_reserved[i] = bitfield_fetch(bitfield, 0x1000000000000 << i);
        }
        let a = Attributes {
            container: bitfield,
            platform_essential: bitfield_fetch::<u64>(bitfield, 0b1),
            efi_ignore: bitfield_fetch::<u64>(bitfield, 0b10),
            legacy_bios_bootable: bitfield_fetch::<u64>(bitfield, 0b100),
            reserved: reserved_flags,
            partition_reserved: partition_reserved,
        };

        Ok(a)
    }
}

impl<'de> de::Visitor<'de> for AttributesVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Attribute serializer...")
    }

    fn visit_u64<E>(self, s: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v = u64::from_le(s);
        Ok(v)
    }
}

impl PartitionEntry {
    pub fn name(&self) -> String {
        std::string::String::from_utf16(&self._name).unwrap()
    }
}
