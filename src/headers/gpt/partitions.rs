use crate::headers::gpt::uuids;
use crate::headers::reader::{bitfield_fetch, uuid_deserialize};
use serde::de;
use serde::{Deserialize, Deserializer};
use serde_big_array::BigArray;
use std::fmt;
use uuid::Uuid;

/*
GPT is also little endian. (according to apple, anyway)
https://developer.apple.com/library/archive/technotes/tn2166/_index.html#//apple_ref/doc/uid/DTS10003927-CH1-SECTION2

*/

#[derive(Deserialize, Debug)]
pub struct PartitionEntry {
    #[serde(deserialize_with = "uuid_deserialize")]
    pub type_guid: Uuid,
    #[serde(deserialize_with = "uuid_deserialize")]
    pub unique_guid: Uuid,
    pub first_lba: u64,
    pub last_lba: u64,
    attributes: Attributes,
    #[serde(with = "BigArray")]
    _name: [u16; 72 / 2], // spec is 72 bytes
}
pub struct Attributes {
    pub container: u64,
    pub platform_essential: bool,
    pub efi_ignore: bool,
    pub legacy_bios_bootable: bool,
    pub reserved: [bool; 47],
    pub partition_reserved: [bool; 15],
}

impl Attributes {
    // msft specific stuff
    // https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/gpt
    pub fn msft_is_read_only(&self) -> bool {
        return bitfield_fetch(self.container, 1 << 60);
    }
    pub fn msft_is_shadow_copy(&self) -> bool {
        return bitfield_fetch(self.container, 1 << 61);
    }
    pub fn msft_is_hidden_partition(&self) -> bool {
        return bitfield_fetch(self.container, 1 << 62);
    }
    pub fn msft_has_no_drive_letter(&self) -> bool {
        return bitfield_fetch(self.container, 1 << 63);
    }
    // some notes about chromeos and boot partitions. TODO: anything w this
    // https://chromium.googlesource.com/chromiumos/docs/+/HEAD/disk_format.md#Selecting-the-kernel
    pub fn chrome_successful_boot_flag(&self) -> bool {
        return bitfield_fetch(self.container, 1 << 56);
    }
    pub fn chrome_boot_tries_remaining(&self) -> u8 {
        return (self.container & (0b1111 << 52) >> 52) as u8;
    }
    pub fn chrome_priority(&self) -> u8 {
        return (self.container & (0b1111 << 48) >> 48) as u8;
    }
}

impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut reserved_all_zero = true;
        let mut partition_reserved_all_zero = true;
        for i in 0..self.reserved.len() {
            if self.reserved[i] == true {
                reserved_all_zero = false;
            }
        }
        for i in 0..self.partition_reserved.len() {
            if self.partition_reserved[i] == true {
                partition_reserved_all_zero = false;
            }
        }
        let mut output_string: String = format!(
            "GpeAttributes: is_platform_essential:{:?} efi_ignore_parition:{:?} is_legacy_bios_bootable:{:?} ",
            self.platform_essential, self.efi_ignore, self.legacy_bios_bootable
        );
        if reserved_all_zero {
            output_string += " reserved: [0;47]"
        } else {
            output_string += format!(" {:?}", self.reserved).as_str();
        }
        if partition_reserved_all_zero {
            output_string += " parition_reserved: [0;15]"
        } else {
            output_string += format!(" {:?}", self.partition_reserved).as_str();
        }

        write!(f, "{}", output_string)
    }
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
    pub fn type_to_str(&self) -> &str {
        match uuids::GUID_TYPE_MAP.get(&self.type_guid) {
            None => "Unknown partition type",
            Some(v) => v,
        }
    }
}

// ref: https://github.com/torvalds/linux/blob/19901165d90fdca1e57c9baa0d5b4c63d15c476a/block/partitions/efi.c
