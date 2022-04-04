use crate::headers::reader::HasRawHeader;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use serde::Deserialize;
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
    partitions: [MbrPartitionEntry; 4],
    bootsector_sig: [u8; 2],
}

#[derive(Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct MbrPartitionEntry {
    attributes: [u8; 1],      // Drive attributes (bit 7 set = active or bootable)
    partition_start: [u8; 3], // CHS Address of partition start
    partition_type: [u8; 1],  //Partition type
    last_partition_sector: [u8; 3], // CHS address of last partition sector
    lba_partition_start: [u8; 4], // LBA of partition start
    sectors_in_partition: [u8; 4], // Number of sectors in partition
}

impl fmt::Debug for MbrPartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MbrPartitionEntry: (hex) {{ attributes: {:x?} start: {:x?} type: {:x?} last sector: {:x?}last start: {:x?} sectors in partition: {:#x?}", self.attributes, self.partition_start,self.partition_type, self.last_partition_sector,self.lba_partition_start, self.sectors_in_partition)
    }
}

pub struct Mbr {
    bootstrap: [u8; 440],
    opt_disk_sig: [u8; 4],
    opt_reserved: u16,
    partitions: [MbrPartitionEntry; 4],
    boot_sector_sig: u16,
}

impl fmt::Debug for Mbr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBR {{ bootstrap: {:x?} disk_sig: {:x?} type: {:#x?} last sector: {:#x?} last start: {:#x?} }}", self.bootstrap, self.opt_disk_sig,self.opt_reserved,self.partitions,self.boot_sector_sig)
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
            partitions: input.partitions,
            boot_sector_sig: LittleEndian::read_u16(&input.bootsector_sig),
        };
        return header;
    }
}
