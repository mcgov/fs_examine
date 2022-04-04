use crate::headers::disx86::disassemble;
use crate::headers::reader::HasRawHeader;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize,Deserializer};
use serde_big_array::BigArray;
use std::fmt;
use std::marker::Copy;
use std::str;

#[derive(Deserialize)]
#[repr(packed)]
pub struct MbrRaw {
    #[serde(with = "BigArray")]
    bootstrap: [u8; 440],
    opt_disk_sig: [u8; 4],
    opt_reserved: [u8; 2],
    partitions: [MbrPartitionEntryRaw; 4],
    bootsector_sig: [u8; 2],
}

#[derive(Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct MbrPartitionEntryRaw {
    attributes: u8,                 // Drive attributes (bit 7 set = active or bootable)
    partition_start: [u8; 3],       // CHS Address of partition start
    partition_type: u8,             //Partition type
    last_partition_sector: [u8; 3], // CHS address of last partition sector
    lba_partition_start: [u8; 4],   // LBA of partition start
    sectors_in_partition: [u8; 4],  // Number of sectors in partition
}

impl fmt::Debug for MbrPartitionEntryRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = self.attributes & 0x80;
        let partition_label: String;
        match PartitionId::from_u8(self.partition_type) {
            None => {
                partition_label = "Unknown".to_string();
            }
            Some(x) => {
                partition_label = format!("{:?}", x);
            }
        }
        write!(f, "MbrPartitionEntryRaw: (hex) {{ is_active: {:x?} start: {:x?} type: {:x?} last sector: {:x?}last start: {:x?} sectors in partition: {:#x?}", active, self.partition_start,partition_label, self.last_partition_sector,self.lba_partition_start, self.sectors_in_partition)
    }
}
#[derive(Deserialize)]
pub struct Mbr {
    #[serde(with = "BigArray")]
    pub bootstrap: [u8; 440],
    pub opt_disk_sig: [u8; 4],
    #[serde(deserialize_with = "le_u16_deserialize")]
    pub opt_reserved: u16,
    pub partitions: [MbrPartitionEntry; 4],
    #[serde(deserialize_with = "le_u16_deserialize")]
    pub boot_sector_sig: u16,
}

impl Mbr {
    pub fn disassemble_bootstrap_sector(&self) {
        disassemble(&self.bootstrap, 16, 0, self.bootstrap.len());
    }
}

impl fmt::Debug for Mbr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MBR {{ (bootstrap above) disk_sig: {:x?} reserved: {:#x?} paritions: {:#x?} boot sig: {:#x?} }}",
            self.opt_disk_sig,
            self.opt_reserved,
            self.partitions,
            self.boot_sector_sig,
        )
    }
}

impl HasRawHeader<Mbr, MbrRaw> for Mbr {
    fn from_raw(input: &MbrRaw) -> Mbr {
        let header = Mbr {
            bootstrap: input.bootstrap,
            opt_disk_sig: input.opt_disk_sig,
            opt_reserved: LittleEndian::read_u16(&input.opt_reserved[..]),
            // these hold ascii data in the windows formatted exfat
            // I'm testing with
            partitions: [
                MbrPartitionEntry::from_raw(&input.partitions[0]),
                MbrPartitionEntry::from_raw(&input.partitions[1]),
                MbrPartitionEntry::from_raw(&input.partitions[2]),
                MbrPartitionEntry::from_raw(&input.partitions[3]),
            ],
            boot_sector_sig: LittleEndian::read_u16(&input.bootsector_sig),
        };
        return header;
    }
}

#[derive(Deserialize)]
pub struct MbrPartitionEntry {
    pub attributes: u8,           // Drive attributes (bit 7 set = active or bootable)
    pub partition_start: [u8; 3], // CHS Address of partition start
    pub partition_type: u8,       //Partition type
    pub last_partition_sector: [u8; 3], // CHS address of last partition sector
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub lba_of_partition_start: u32, // LBA of partition start
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub sectors_in_partition: u32, // Number of sectors in partition
}

fn le_u32_deserialize<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u32>::deserialize(d)?;
    data = u32::from_le(data);
    Ok(data)
}
fn le_u16_deserialize<'de, D>(d: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u16>::deserialize(d)?;
    data = u16::from_le(data);
    Ok(data)
}



impl fmt::Debug for MbrPartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = (self.attributes & 0x80) != 0;
        let partition_label: String;
        match PartitionId::from_u8(self.partition_type) {
            None => {
                partition_label = "Unknown".to_string();
            }
            Some(x) => {
                partition_label = format!("{:?}", x);
            }
        }
        write!(f, "MbrPartitionEntry: (hex) {{ is_active: {:x?} chs start: {:x?} type: {:x?} chs last part sector: {:x?} lba of part start: {:x?} sectors in partition: {:x?}", active, self.partition_start,partition_label, self.last_partition_sector,self.lba_of_partition_start, self.sectors_in_partition)
    }
}

impl HasRawHeader<MbrPartitionEntry, MbrPartitionEntryRaw> for MbrPartitionEntry {
    fn from_raw(input: &MbrPartitionEntryRaw) -> MbrPartitionEntry {
        MbrPartitionEntry {
            attributes: input.attributes,
            partition_start: input.partition_start,
            partition_type: input.partition_type,
            last_partition_sector: input.last_partition_sector,
            lba_of_partition_start: LittleEndian::read_u32(&input.lba_partition_start),
            sectors_in_partition: LittleEndian::read_u32(&input.sectors_in_partition),
        }
    }
}

#[derive(Debug, FromPrimitive)]
enum PartitionId {
    Empty = 0,
    Fat12 = 1,
    Xenix = 2,
    Fat16 = 4,
    Ebr = 5,
    Fat16b = 6,
    NtfsAlsoExfat = 7,
    OldBullshit = 8,
    AixQnx = 9,
    Os2Bm = 0xa,
    Fat32Chs = 0xB,
    Fat32Lba = 0xC,
    Forbidden = 0xd,
    Fat16Lba = 0xe,
    EbrLba = 0xf,
    Fat12Fat16Logical = 0x11,
    ShitOemPartition = 0x12,
    WindowsServiceFs = 0x27,
    LynxRtos = 0x50,
    YocFs = 0x59,
    LinuxSwap = 0x82,
    LinuxFs = 0x83,
    Hibernation = 0x84,
    LinuxRaidSuperblock = 0x86,
    LinuxLvm = 0x8e,
    AppleUfs = 0xa8,
    AppleBoot = 0xAB,
    AppleRaid = 0xAc,
    WindowsGptSafeMbr = 0xEF,
    LinuxExt3Pache = 0xFD,
}
