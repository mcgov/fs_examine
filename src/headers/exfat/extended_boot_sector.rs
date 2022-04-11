use crate::headers::disx86::disassemble;
use byteorder::{ByteOrder, LittleEndian};
use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
pub struct ExtendedBootSector {
    /* this section sucks. It's always 8 sectors large at least.

    Boot code is variable based on an boot sector param
    there's a signature at the end, but it's position is also based on the boot code
    size.

    Just going to say 'nope' and munch up the bytes
    and mooooove on. I'll come back when I'm better at Rust.

    */
    #[serde(with = "BigArray")]
    raw_sector_bytes: [u8; 512 * 8],
}

impl ExtendedBootSector {
    pub fn disassemble_boot_code(&self, boot_code_size: u64, rip: u64) {
        disassemble(
            &self.raw_sector_bytes,
            32,
            rip,
            boot_code_size.try_into().unwrap(),
        );
    }
    pub fn get_boot_sector_signature(&self, boot_code_size: u64) -> u32 {
        let usize_size = boot_code_size.try_into().unwrap();
        LittleEndian::read_u32(&self.raw_sector_bytes[usize_size..usize_size + 4])
    }
    pub fn get_boot_code_size(&self, bytes_per_sector_shift: u8) -> u64 {
        (1 << bytes_per_sector_shift) - 4
    }

    pub fn section_is_valid(&self, bytes_per_sector_shift: u8) -> bool {
        let signature =
            self.get_boot_sector_signature(self.get_boot_code_size(bytes_per_sector_shift));
        signature == 0xAA550000
    }
}
