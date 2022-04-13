use crate::headers::*;
use colored::*;
/* I don't care that nobody uses disks anymore I'm calling it this to justify the name of the exe */
#[derive(Debug)]
pub struct Disk {
    pub pt_type: PartitionTableType,
    pub partitions: Vec<Partition>,
    pub mbr: mbr::Mbr,
    pub file_arg: String,
}

#[derive(Debug)]
pub enum PartitionTableType {
    Mbr,
    Gpt,
}
#[derive(Debug)]
pub struct PartitionTable {
    pt_type: PartitionTableType,
    pt_offset: u64,
}
#[derive(Debug, Copy, Clone)]
pub enum PartitionType {
    Xfs,
    Ext4,
    Exfat,
    Unused,
    EfiSystem,
    BiosBoot,
    LinuxFsTBD,
    Unknown,
}
#[derive(Debug, Clone)]
pub struct Partition {
    p_type: PartitionType,
    p_offset: u64,
    p_size: u64,
    p_name: String,
}

impl Disk {
    pub fn set_partition_table_type(&mut self) {
        let _gpt_part = self.mbr.get_partition(0);
        match _gpt_part.get_partition_type() {
            mbr::PartitionId::Gpt => {
                self.pt_type = PartitionTableType::Gpt;
            }
            _ => {
                panic!("No other partition type is implemented yet.");
            }
        }
    }

    pub fn get_gpt(&self) -> gpt::Gpt {
        assert_eq!(matches!(self.pt_type, PartitionTableType::Gpt), true);
        let gpt =
            reader::read_header_from_offset::<gpt::Gpt>(&self.file_arg, constants::SMOL_BLOCKS);
        summer::struct_validate_checksum32::<gpt::Gpt>(
            &self.file_arg,
            &gpt,
            constants::SMOL_BLOCKS,
        );
        gpt.validate_table_checksums(&self.file_arg);
        gpt.print_partition_table(&self.file_arg);
        gpt
    }

    pub fn register_partitions(&mut self) {
        match self.pt_type {
            PartitionTableType::Gpt => {
                let gpt = self.get_gpt();
                for partition in gpt.create_partition_table(&self.file_arg) {
                    let part = Partition {
                        p_type: partition.get_partition_type(),
                        p_offset: partition.first_lba * constants::SMOL_BLOCKS,
                        p_size: (partition.last_lba - partition.first_lba) * constants::SMOL_BLOCKS,
                        p_name: partition.name(),
                    };
                    self.partitions.push(part);
                }
            }
            _ => {
                panic!("Partition type not implemented: {:?}", self.pt_type);
            }
        }
    }

    pub fn print_partitions(&self) {
        for part in self.partitions.clone().into_iter() {
            if !matches!(part.p_type, PartitionType::Unused) {
                println!(
                    "PartitionName:{}\n\
                    PartitionType:{:?}\n\
                    PartitionStart:0x{:X}\n\
                    PartitionSize:0x{:X}\n\
                    ---------------------------",
                    part.p_name.yellow(),
                    part.p_type,
                    part.p_offset,
                    part.p_size
                );
            }
        }
    }
}
