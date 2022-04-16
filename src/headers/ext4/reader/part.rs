use super::superblock::*;
use super::*;
use crate::headers::reader::read_header_from_offset;
use crate::headers::summer;
impl Part {
    pub fn init(file: String, sb: Superblock, start: u64) -> Part {
        Part {
            file: file.clone(),
            start: start,
            s: sb,
            bg: vec![],
        }
    }
    pub fn populate_block_groups(&mut self) {
        let bgdt_offset = self.s.get_group_descriptor_table_offset(self.start);
        for i in 0..self.s.number_of_groups() {
            if self.s.uses_64bit() && self.s.desc_size > 32 {
                let combined_size = std::mem::size_of::<BlockGroupDescriptor32>()
                    + std::mem::size_of::<BlockGroupDescriptor64>();
                if self.s.desc_size < combined_size as u16 {
                    panic!(
                        "size for 64bit group descriptor didn't validate, should be at least {}",
                        combined_size
                    );
                }
                let bg_offset = bgdt_offset + combined_size as u64 * i;
                let bg32 = read_header_from_offset::<BlockGroupDescriptor32>(&self.file, bg_offset);
                let bg64 = read_header_from_offset::<BlockGroupDescriptor64>(
                    &self.file,
                    bg_offset + std::mem::size_of::<BlockGroupDescriptor32>() as u64,
                );
                let bgboi = Bg::init(bg_offset, Some(bg32), Some(bg64));
                //bgboi.print();
                self.bg.push(bgboi);
            } else {
                let bg_offset =
                    bgdt_offset + std::mem::size_of::<BlockGroupDescriptor32>() as u64 * i;
                let bg = read_header_from_offset::<BlockGroupDescriptor32>(&self.file, bg_offset);
                let bgboi = Bg::init(bg_offset, Some(bg), None);
                //bgboi.print();
                self.bg.push(bgboi);
            }
        }
        // TODO:

        //validate each one, these have checksums
        println!(
            "{} sanity check: {:X}",
            format!("found {:X} block group descriptors.", self.bg.len()).blue(),
            self.s.number_of_groups()
        );
    }

    pub fn populate_inodes(&mut self) {
        for i in 0..self.bg.len() {
            self.bg[i].populate_inodes(&self.file, &self.s, self.start);
        }
    }

    pub fn validate_block_groups(&mut self) {
        self.s.debug_print_some_stuf();
        if self.s.metadata_csum() {
            let mut csum_seed: u32 = self.s.checksum_seed;
            if !self.s.has_feature_checksum_seed() {
                csum_seed = !0;
            }
            for bgid in 0..self.bg.len() {
                let mut bytes: Vec<u8> = vec![];

                for byte in self.s.uuid {
                    bytes.push(byte);
                }
                for byte in <u32>::to_le_bytes(bgid.try_into().unwrap()) {
                    bytes.push(byte);
                }
                let bg_item = self.bg.get(bgid).unwrap();
                let bg_start = bg_item.start;
                bytes.append(&mut reader::read_bytes_from_file(
                    &self.file, bg_start, 0x1e,
                ));
                bytes.push(0);
                bytes.push(0); //fake checksum field
                if self.s.uses_64bit() && self.s.desc_size > 32 {
                    bytes.append(&mut reader::read_bytes_from_file(
                        &self.file,
                        bg_start + 0x20,
                        (self.s.desc_size - 0x20) as u64,
                    ));
                }
                let bgd = bg_item.b32.unwrap();

                let crcsum = summer::crc32c(csum_seed, bytes);
                if bgd.checksum as u32 != (crcsum & 0xffff) {
                    println!("{}", "WARNING: checksum did not match!".yellow());
                } else {
                    println!("BG#{} {}", bgid, "Checksum matches!".green());
                }
            }
        } else if self.s.has_feature_gdt_csum() {
            // old version
            for bgid in 0..self.bg.len() {
                let mut bytes: Vec<u8> = vec![];

                let bytesdisk =
                    reader::read_bytes_from_file(&self.file, self.start + 1024 + 0x68, 16);
                assert_eq!(bytesdisk, self.s.uuid);

                bytes.append(&mut self.s.uuid.to_vec());
                for byte in <u32>::to_le_bytes(bgid as u32) {
                    bytes.push(byte);
                }

                let bg_item = self.bg.get(bgid).unwrap();

                let bg_start = bg_item.start;
                let bitecopy = reader::read_bytes_from_file(&self.file, bg_start, 0x1e);

                unsafe {
                    let bites = std::mem::transmute::<BlockGroupDescriptor32, [u8; 0x20]>(
                        bg_item.b32.as_ref().unwrap().clone(),
                    );
                    assert_eq!(bitecopy, bites[..bites.len() - 2].to_vec());
                    bytes.append(&mut bites[..bites.len() - 2].to_vec())
                }

                let bg32 = bg_item.b32.as_ref().unwrap();
                let crcsum = summer::crc16(!0, bytes.clone());
                let bgcrc = bg32.checksum;
                if bgcrc != crcsum {
                    println!(
                                 "{} checksum did not match (but it's this tool that's broken): {:04x} {:04x} {:04x} {:04x}",
                                 "bolo".yellow(), crcsum, !crcsum, !bgcrc, bgcrc
                             )
                } else {
                    println!("checksum matches for bg {:x}", bgid);
                }
            }
        }
    }
}
