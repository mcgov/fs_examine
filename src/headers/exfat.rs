use serde::Deserialize;
use crate::headers::reader::{le_u32_deserialize, le_u16_deserialize, le_u64_deserialize};
use serde_big_array::BigArray;

macro_rules! in_range_inclusive {
    ($low:expr,$val:expr,$high:expr,$type:ty) => {
        return (($low) as $type) <= (($val) as $type) && (($val) as $type) <= (($high) as $type)
    };
}

#[derive(Deserialize, Debug)]
pub struct BootSector {
    pub jumpboot: [u8; 3],
    pub file_system_name: [u8; 8],
    #[serde(with = "BigArray")]
    pub must_be_zero: [u8; 53],
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub partition_offset: u64,
    #[serde(deserialize_with = "le_u64_deserialize")]
    pub volume_length: u64,
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub fat_offset: u32,
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub fat_length: u32,
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub cluster_heap_offset: u32,
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub cluster_count: u32,
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub first_cluster_of_root_directory: u32,
    #[serde(deserialize_with = "le_u32_deserialize")]
    pub volume_serial_number: u32,
    pub file_system_revision: [u8; 2],
    #[serde(deserialize_with = "le_u16_deserialize")]
    pub volume_flags: u16,
    pub bytes_per_sector_shift: u8,
    pub sectors_per_cluster_shift: u8,
    pub number_of_fats: u8,
    pub drive_select: u8,
    pub percent_in_use: u8,
    pub reserved: [u8; 7],
    #[serde(with = "BigArray")]
    pub boot_code: [u8; 390],
    #[serde(deserialize_with = "le_u16_deserialize")]
    pub boot_signature: u16,
    // NOTE: the Main and Backup Boot Sectors both contain the BytesPerSectorShift field.
    // NOTE: ExcessSpace following the header is (2**BytesPerSectorShift)-512
}

impl BootSector {
    pub fn print_header(&self) {
        println!("jumpboot: {:x?}", self.jumpboot);
        println!(
            "fs_name {:x?}",
            String::from_utf8(self.file_system_name.to_vec())
        );
        println!("must_be_zero {:x?}", self.must_be_zero);
        println!("partition_offset {:x}", self.partition_offset);
        println!("volume_length {:x}", self.volume_length);
        println!("fat_offset {:x}", self.fat_offset);
        println!("fat_length {:x}", self.fat_length);
        println!("cluster_heap_offset {:x}", self.cluster_heap_offset);
        println!("cluster_count {:x}", self.cluster_count);
        println!(
            "first_cluster_of_root_directory {:?}",
            self.first_cluster_of_root_directory
        );
        println!("volume_serial_number {:x}", self.volume_serial_number);
        println!("file_system_revision {:x?}", self.file_system_revision);
        println!("volume_flags {:x}", self.volume_flags);
        println!("bytes_per_sector_shift {:x}", self.bytes_per_sector_shift);
        println!(
            "sectors_per_cluster_shift {:x}",
            self.sectors_per_cluster_shift
        );
        println!("number_of_fats {:x}", self.number_of_fats);
        println!("drive_select {:x}", self.drive_select);
        println!("percent_in_use {:x}", self.percent_in_use);
        println!("reserved {:x?}", self.reserved);
        println!("boot_code {:x?}", self.boot_code);
        println!("boot_signature {:x}", self.boot_signature);
    }
}

impl BootSector {
    pub fn validate_header(&self) -> bool {
        let mut valid = true;
        if !self.validate_boot_code() {
            println!("field boot_code ({:?}) was invalid!", self.boot_code);
            valid = false;
        }
        if !self.validate_boot_signature() {
            println!(
                "field boot_signature ({:?}) was invalid!",
                self.boot_signature
            );
            valid = false;
        }
        if !self.validate_bytes_per_sector_shift() {
            println!(
                "field bytes_per_sector_shift ({:?}) was invalid!",
                self.bytes_per_sector_shift
            );
            valid = false;
        }
        if !self.validate_cluster_count() {
            println!(
                "field cluster_count ({:?}) was invalid!",
                self.cluster_count
            );
            valid = false;
        }
        if !self.validate_cluster_heap_offset() {
            println!(
                "field cluster_heap_offset ({:?}) was invalid!",
                self.cluster_heap_offset
            );
            valid = false;
        }
        if !self.validate_drive_select() {
            println!("field drive_select ({:?}) was invalid!", self.drive_select);
            valid = false;
        }
        if !self.validate_fat_length() {
            println!("field fat_length ({:?}) was invalid!", self.fat_length);
            valid = false;
        }
        if !self.validate_fat_offset() {
            println!("field fat_offset ({:?}) was invalid!", self.fat_offset);
            valid = false;
        }
        if !self.validate_file_system_revision() {
            println!(
                "field file_system_revision ({:?}) was invalid!",
                self.file_system_revision
            );
            valid = false;
        }
        if !self.validate_filesystem_name() {
            println!(
                "field filesystem_name ({:?}) was invalid!",
                self.file_system_name
            );
            valid = false;
        }
        if !self.validate_first_cluster_of_root_directory() {
            println!(
                "field first_cluster_of_root_directory ({:?}) was invalid!",
                self.first_cluster_of_root_directory
            );
            valid = false;
        }
        if !self.validate_jumpboot() {
            println!("field jumpboot ({:x?}) was invalid!", self.jumpboot);
            valid = false;
        }
        if !self.validate_must_be_zero() {
            println!("field must_be_zero ({:?}) was invalid!", self.must_be_zero);
            valid = false;
        }
        if !self.validate_number_of_fats() {
            println!(
                "field number_of_fats ({:?}) was invalid!",
                self.number_of_fats
            );
            valid = false;
        }
        if !self.validate_partition_offset() {
            println!(
                "field partition_offset ({:?}) was invalid!",
                self.partition_offset
            );
            valid = false;
        }
        if !self.validate_percent_in_use() {
            println!(
                "field percent_in_use ({:?}) was invalid!",
                self.percent_in_use
            );
            valid = false;
        }
        if !self.validate_reserved() {
            println!("field reserved ({:?}) was invalid!", self.reserved);
            valid = false;
        }
        if !self.validate_sectors_per_cluster_shift() {
            println!(
                "field sectors_per_cluster_shift ({:?}) was invalid!",
                self.sectors_per_cluster_shift
            );
            valid = false;
        }
        if !self.validate_volume_flags() {
            println!("field volume_flags ({:?}) was invalid!", self.volume_flags);
            valid = false;
        }
        if !self.validate_volume_length() {
            println!(
                "field volume_length ({:?}) was invalid!",
                self.volume_length
            );
            valid = false;
        }
        if !self.validate_volume_serial_number() {
            println!(
                "field volume_serial_number ({:?}) was invalid!",
                self.volume_serial_number
            );
            valid = false;
        }
        if valid {
            println!("Header was valid! ... ?")
        }
        valid
    }

    pub fn get_excess_space_size(&self) -> u32 {
        (2 << self.bytes_per_sector_shift) - 512
    }

    fn validate_jumpboot(&self) -> bool {
        self.jumpboot == [0xEB, 0x76, 0x90]
        // invalid jmp instruction in header
        // https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification#311-jumpboot-field
    }
    fn validate_filesystem_name(&self) -> bool {
        // NOTE: exFAT requires this to be 'EXFAT   '
        // older fat had a similar field but let it be anything.
        // we'll check that it's printable for the moment.
        for a in self.file_system_name {
            if a < 0x20 {
                // ' ' is first printable character in  ASCII
                return false;
            }
        }
        return true;
    }

    fn validate_must_be_zero(&self) -> bool {
        for z in self.must_be_zero {
            if z != 0 {
                println!("fields in header.must_be_zero are not zero!");
                return false;
            }
        }
        true
    }
    fn validate_partition_offset(&self) -> bool {
        // The PartitionOffset field shall describe the media-relative sector offset of the partition which hosts the given exFAT volume.
        // SPEC: All possible values for this field are valid; however, the value 0 indicates implementations shall ignore this field.
        true
    }

    fn volume_length_if_zero(&self) -> u64 {
        return (self.cluster_heap_offset as u64)
            + (0xFFFFFFF4 as u64) * ((1 as u64) << self.sectors_per_cluster_shift);
    }
    fn get_volume_length(&self) -> u64 {
        if self.get_excess_space_size() == 0 {
            return self.volume_length_if_zero();
        }
        self.volume_length
    }
    fn validate_volume_length(&self) -> bool {
        // https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification#315-volumelengthfield
        let val_if_zero =
            self.get_excess_space_size() == 0 && self.volume_length == self.volume_length_if_zero();
        let check_if_not_zero = self.volume_length
            >= ((2 << 20) / (2 << self.bytes_per_sector_shift))
            && self.volume_length <= (0xFFFFFFFFFFFFFFFE);
        if self.get_excess_space_size() == 0 && !val_if_zero {
            println!(
                "excess_space_size is zero and volume length is unexpected, should be {} found {}",
                self.volume_length_if_zero(),
                self.volume_length
            );
            return false;
        }
        if !check_if_not_zero {
            println!("Volume length was out of expected range, should be >= (2^20)/{} and <= <2^63-512, found {}", self.bytes_per_sector_shift, self.volume_length);
            return false;
        }
        true
    }
    fn validate_fat_offset(&self) -> bool {
        return self.partition_offset >= 24
            || self.partition_offset
                <= (self.cluster_heap_offset as u64
                    - self.fat_length as u64 * self.number_of_fats as u64);
    }
    fn validate_fat_length(&self) -> bool {
        // At least (ClusterCount + 2) * 2^2/ 2BytesPerSectorShift rounded up to the nearest integer,
        // which ensures each FAT has sufficient space for describing all the clusters in the Cluster Heap
        let numerator = (self.cluster_count as u64 + 2) * 4;
        let denominator = (2 as u64) << self.bytes_per_sector_shift;
        let mut must_be_at_least = numerator / denominator;
        if numerator % denominator != 0 {
            must_be_at_least += 1;
        }
        if must_be_at_least > 0xFFFFFFFF {
            return false;
        }
        // At most (ClusterHeapOffset - FatOffset) / NumberOfFats rounded down to the nearest integer,
        // which ensures the FATs exist before the Cluster Heap
        if self.cluster_count <= self.fat_offset {
            return false;
        }
        let must_be_at_most = (self.cluster_count - self.fat_offset) / self.number_of_fats as u32;
        // NOTE: must be at least is range checked above ^
        in_range_inclusive!(must_be_at_least,self.fat_length, must_be_at_most, u32)
    }
    fn validate_cluster_heap_offset(&self) -> bool {
        //At least FatOffset + FatLength * NumberOfFats, to account for the sectors all the preceding regions consume
        let total_fats_length = self.fat_length as u64 * self.number_of_fats as u64;
        let at_least = self.fat_offset as u64 + total_fats_length;
        //At most 2^32- 1 or VolumeLength - (ClusterCount * 2SectorsPerClusterShift), whichever calculation is less
        let at_most = std::cmp::min::<u64>(
            0xFFFFFFF5, // (2**32)-11
            self.cluster_count as u64 * ((1 as u64) << self.sectors_per_cluster_shift),
        );
        in_range_inclusive!(at_least, self.cluster_heap_offset, at_most, u64)
    }
    fn validate_cluster_count(&self) -> bool {
        if self.volume_length <= self.cluster_heap_offset as u64 {
            println!(
                "volume length {} < cluster offset {}",
                self.volume_length, self.cluster_heap_offset
            );
            return false;
        }
        let must_be_at_least = (self.volume_length - self.cluster_heap_offset as u64)
            / (1 << self.sectors_per_cluster_shift);
        if must_be_at_least == 0 {
            return false;
        }
        let must_be_at_most: u32 = 0xfffffff5; // 2^32 - 11
        in_range_inclusive!(must_be_at_least, self.cluster_count, must_be_at_most, u64)
    }
    fn validate_first_cluster_of_root_directory(&self) -> bool {
        in_range_inclusive!(
            2,
            self.first_cluster_of_root_directory,
            self.cluster_count + 1,
            u64
        )
    }
    fn validate_volume_serial_number(&self) -> bool {
        // all values are valid
        true
    }
    fn validate_file_system_revision(&self) -> bool {
        // implementation requires data on disk to be little endian
        // low order byte is less than 99
        // high order byte is greater than 1 and less than 99
        return self.file_system_revision[0] < 99
            && self.file_system_revision[1] < 99
            && self.file_system_revision[1] > 0;
    }
    fn validate_volume_flags(&self) -> bool {
        // these can sort of be whatever apparently
        true
    }

    fn validate_bytes_per_sector_shift(&self) -> bool {
        in_range_inclusive!(9, self.bytes_per_sector_shift, 12, u8)
    }
    fn validate_sectors_per_cluster_shift(&self) -> bool {
        if !self.validate_bytes_per_sector_shift() {
            // warn, it will fail with the other validation but it's interesting if they aren't both invalid.
            println!("WARN: bytes per sector shift is out of range and sectors per cluster depends on it's validity.");
        }
        self.sectors_per_cluster_shift <= (25 - self.bytes_per_sector_shift)
    }
    fn validate_number_of_fats(&self) -> bool {
        in_range_inclusive!(1, self.number_of_fats, 2, u8)
    }
    fn validate_drive_select(&self) -> bool {
        //The DriveSelect field shall contain the extended INT 13h drive number,
        // which aids boot-strapping from this volume using extended INT 13h on
        // personal computers.
        //All possible values for this field are valid. Similar fields in previous
        // FAT-based file systems frequently contained the value 80h.
        true
    }
    fn validate_percent_in_use(&self) -> bool {
        (self.percent_in_use <= 100) || (self.percent_in_use == 0xff)
    }
    fn validate_reserved(&self) -> bool {
        // looks good m8
        true
    }
    fn validate_boot_code(&self) -> bool {
        true
    }
    fn validate_boot_signature(&self) -> bool {
        // if this is anything else it's supposed to invalidate the boot sector
        // so maybe force backup sector use? need to keep reading.
        self.boot_signature == 0xAA55
    }

    fn get_flag_active_fat(&self) -> bool {
        0 != (self.volume_flags & 0b1)
    }
    fn get_flag_dirty(&self) -> bool {
        0 != (self.volume_flags & 0b10)
    }
    fn get_flag_media_failure(&self) -> bool {
        0 != (self.volume_flags & 0b100)
    }
    fn get_flag_clear_to_zero(&self) -> bool {
        //The ClearToZero field does not have significant meaning in this specification.
        // great
        0 != (self.volume_flags & 0b1000)
    }
    fn get_flag_reserved(&self) -> u16 {
        (self.volume_flags & (!0b1111 as u16)) >> 4
    }
}
