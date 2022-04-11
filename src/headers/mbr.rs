use super::reader::*;
use crate::headers::disx86::disassemble;
use crate::prettify_output;
use colored::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::Deserialize;
use serde_big_array::BigArray;
use std::fmt;
use std::str;

#[derive(Deserialize)]
pub struct Mbr {
    #[serde(with = "BigArray")]
    pub bootstrap: [u8; 440],
    pub opt_disk_sig: [u8; 4],
    pub opt_reserved: u16,
    pub partitions: [MbrPartitionEntry; 4],
    pub boot_sector_sig: u16,
}

impl Mbr {
    pub fn disassemble_bootstrap_sector(&self) {
        disassemble(&self.bootstrap, 16, 0, self.bootstrap.len());
    }
    pub fn get_partition(&self, index: u64) -> MbrPartitionEntry {
        return self.partitions[index as usize];
    }
    pub fn pretty_print(&self) {
        prettify_output!(Mbr, purple, bright_purple, {
            println!(
                "MBR (skipping bootstrap...) disk_sig: {:x?} reserved:{}",
                self.opt_disk_sig, self.opt_reserved,
            );
            for partition in self.partitions.iter() {
                partition.pretty_print();
            }
            println!("boot sector signature: {:x}", self.boot_sector_sig);
        });
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

#[derive(Deserialize, Clone, Copy)]
pub struct MbrPartitionEntry {
    pub attributes: u8,           // Drive attributes (bit 7 set = active or bootable)
    pub partition_start: [u8; 3], // CHS Address of partition start
    pub partition_type: u8,       //Partition type
    pub last_partition_sector: [u8; 3], // CHS address of last partition sector
    pub lba_of_partition_start: u32, // LBA of partition start
    pub sectors_in_partition: u32, // Number of sectors in partition
}

impl MbrPartitionEntry {
    pub fn pretty_print(&self) {
        let active = (self.attributes & 0x80) != 0;
        let partition_label: String;
        println!("{}", "Partitions:".purple().to_string());
        match PartitionId::from_u8(self.partition_type) {
            None => {
                partition_label = "Unknown".bright_purple().to_string();
            }
            Some(x) => {
                partition_label = format!("{:?}", x).bright_blue().to_string();
            }
        }
        println!(
            "{}",
            format!("[{}] is_active: {}\nchs start: {:x?}\nchs last part sector: {:x?}\nlba of part start: {:x?}\nsectors in partition: {:x?}", partition_label, print_bool(active), self.partition_start, self.last_partition_sector,self.lba_of_partition_start, self.sectors_in_partition)
        );
    }
}

// this one sucks it doesn't have fun colors
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
    Gpt = 0xEE, // if this is the type we just ignore all the other shit in the MBR.
    WindowsGptSafeMbr = 0xEF,
    LinuxExt3Pache = 0xFD,
}
