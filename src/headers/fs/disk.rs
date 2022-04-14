use super::DiskPart;
use crate::headers::reader::HasHeaderMagic;
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
    pub p_type: PartitionType,
    p_offset: u64,
    p_size: u64,
    p_name: String,
}

impl Partition {
    pub fn check_linux_fs_type(&self, file_arg: &str) -> PartitionType {
        // really wish I could loop on types, this might be macro-able though once I need to

        let sb = reader::read_header_from_offset::<ext4::superblock::Superblock>(
            &file_arg,
            self.p_offset + constants::EXT4_SUPERBLOCK_0_OFFSET,
        );
        if sb.check_magic_field(
            &file_arg,
            self.p_offset + constants::EXT4_SUPERBLOCK_0_OFFSET,
        ) {
            return PartitionType::Ext4;
        }
        return PartitionType::LinuxFsTBD;
        //let xfs = read::read_header_from_offset::<xfs::ondiskhdr::XfsOndiskHeader> when implemented
    }
    pub fn get_partition_bitness(&self, file_arg: &str) -> u16 {
        match self.p_type {
            PartitionType::Ext4 => {
                let sb = reader::read_header_from_offset::<ext4::superblock::Superblock>(
                    &file_arg,
                    self.p_offset + constants::EXT4_SUPERBLOCK_0_OFFSET,
                );
                return sb.bitness();
            }
            _ => {
                panic!("not implemented")
            }
        }
    }
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
        //gpt.print_partition_table(&self.file_arg);
        gpt
    }

    pub fn register_partitions(&mut self) {
        match self.pt_type {
            PartitionTableType::Gpt => {
                let gpt = self.get_gpt();
                for partition in gpt.create_partition_table(&self.file_arg) {
                    let mut part = Partition {
                        p_type: partition.get_partition_type(),
                        p_offset: partition.first_lba * constants::SMOL_BLOCKS,
                        p_size: (partition.last_lba - partition.first_lba) * constants::SMOL_BLOCKS,
                        p_name: partition.name(),
                    };
                    if matches!(part.p_type, PartitionType::LinuxFsTBD) {
                        part.p_type = part.check_linux_fs_type(&self.file_arg)
                    }
                    self.partitions.push(part);
                }
            }
            _ => {
                panic!("Partition type not implemented: {:?}", self.pt_type);
            }
        }
    }

    pub fn print_partitions_shitty(&self) {
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

    pub fn print_partitions_pretty(&self) {
        match &self.pt_type {
            PartitionTableType::Gpt => {
                self.get_gpt().print_partition_table(&self.file_arg);
            }
            PartitionTableType::Mbr => {
                self.mbr.pretty_print();
            }
        }
    }

    pub fn get_partition(&self, ptid: usize) -> Partition {
        return self.partitions.get(ptid).unwrap().clone();
    }

    pub fn make_ext4_reader(&mut self, p: Partition) -> ext4::reader::Part {
        assert!(matches!(p.p_type, PartitionType::Ext4));
        let sb = reader::read_header_from_offset::<ext4::superblock::Superblock>(
            &self.file_arg,
            p.p_offset + constants::EXT4_SUPERBLOCK_0_OFFSET,
        );
        ext4::reader::Part::init(self.file_arg.clone(), sb, p.p_offset)
    }
    pub fn make_ext4_reader_index(&mut self, ptid: usize) -> ext4::reader::Part {
        let part = self.get_partition(ptid);
        self.make_ext4_reader(part)
    }
}
