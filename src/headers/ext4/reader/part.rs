use super::superblock::*;
use super::*;
use crate::headers::reader::*;
use crate::headers::summer;
impl Part {
    pub fn init(reader: OnDisk, sb: Superblock, start: u64) -> Part {
        Part {
            reader: reader,
            start: start,
            s: sb,
            bg: vec![],
        }
    }
    pub fn populate_block_groups(&mut self) {
        let bgdt_offset =
            self.s.get_group_descriptor_table_offset(self.start);
        for i in 0..self.s.number_of_groups() {
            if self.s.uses_64bit() && self.s.desc_size > 32 {
                let combined_size =
                    std::mem::size_of::<BlockGroupDescriptor32>()
                        + std::mem::size_of::<BlockGroupDescriptor64>(
                        );
                if self.s.desc_size < combined_size as u16 {
                    panic!(
                        "size for 64bit group descriptor didn't \
                         validate, should be at least {}",
                        combined_size
                    );
                }
                let bg_offset =
                    bgdt_offset + combined_size as u64 * i;
                let bg32 = self
                    .reader
                    .read_header_from_offset::<BlockGroupDescriptor32>(bg_offset);
                let bg64 = self
                    .reader
                    .read_header_from_offset::<BlockGroupDescriptor64>(
                        bg_offset + std::mem::size_of::<BlockGroupDescriptor32>() as u64,
                    );
                //println!("{:#x?} {:#x?}", bg32, bg64);
                let bgboi = Bg::init(
                    i as u32,
                    bg_offset,
                    Some(bg32),
                    Some(bg64),
                );
                //bgboi.print();
                self.bg.push(bgboi);
            } else {
                let bg_offset = bgdt_offset
                    + std::mem::size_of::<BlockGroupDescriptor32>()
                        as u64
                        * i;
                let bg = self
                    .reader
                    .read_header_from_offset::<BlockGroupDescriptor32>(bg_offset);
                let bgboi =
                    Bg::init(i as u32, bg_offset, Some(bg), None);
                //bgboi.print();
                self.bg.push(bgboi);
            }
        }
        println!(
            "{}",
            format!(
                "found {:X} block group descriptors.",
                self.bg.len()
            )
            .blue(),
        );
    }

    pub fn populate_inodes(&mut self) {
        if self.s.backup_bgs != [0, 0] {
            println!("Note: Backup BGS at {:x?}", self.s.backup_bgs);
        }
        for i in 0..self.bg.len() {
            self.bg[i].populate_inodes(
                &mut self.reader,
                &self.s,
                self.start,
            );
            let count = self.bg[i].ino.len();
            if count != 0 {
                println!("Found {} inodes in bg:{} ", count, i + 1);
            }
        }
    }

    pub fn validate_block_groups(&mut self) {
        self.s.debug_print_some_stuf();
        for bgid in 0..self.bg.len() {
            let mut bytes: Vec<u8> = vec![];
            if self.s.metadata_csum() {
                //crc32c with optional seed from sb
                // seed allows csum to not depend on uuid
                // to allow volume re-uuid-ing
                //if seed exists, it's the precomputed uuid
                // crc
                let mut csum_seed: u32 = self.s.checksum_seed;
                if !self.s.has_feature_checksum_seed() {
                    // else seed is 0xffffffff (or should
                    // have been)
                    csum_seed = !0;
                    for byte in self.s.uuid {
                        bytes.push(byte);
                    }
                }
                let bg_bytes =
                    <u32>::to_le_bytes(bgid.try_into().unwrap());
                bytes.append(&mut bg_bytes.to_vec());
                let bg_purt = self.bg.get(bgid).unwrap();
                let bg_start = bg_purt.start;
                let bg_ondisk =
                    self.reader.read_bytes_from_file(bg_start, 0x1e);
                bytes.append(&mut bg_ondisk.clone());
                bytes.push(0);
                bytes.push(0); //fake checksum field
                if self.s.uses_64bit() && self.s.desc_size > 32 {
                    bytes.append(
                        &mut self.reader.read_bytes_from_file(
                            bg_start + 0x20,
                            (self.s.desc_size - 0x20) as u64,
                        ),
                    );
                }
                let bgd_actual = bg_purt.b32.unwrap();

                let crcsum = summer::crc32c(csum_seed, bytes);
                if bgd_actual.checksum as u32 != (crcsum & 0xffff) {
                    println!(
                        "{}",
                        "WARNING: checksum did not match!".yellow()
                    );
                } else {
                    println!(
                        "BG#{} {}",
                        bgid,
                        "Checksum matches!".green()
                    );
                }
            } else if self.s.has_feature_gdt_csum() {
                // old crc16 version
                let bytesdisk = self.reader.read_bytes_from_file(
                    self.start + 1024 + 0x68,
                    16,
                );
                assert_eq!(bytesdisk, self.s.uuid);

                bytes.append(&mut self.s.uuid.to_vec());
                for byte in <u32>::to_le_bytes(bgid as u32) {
                    bytes.push(byte);
                }

                let bg_purt = self.bg.get(bgid).unwrap();

                let bg_start = bg_purt.start;
                let bitecopy =
                    self.reader.read_bytes_from_file(bg_start, 0x1e);
                /* not sure whether BE requires using the
                 * in-memory fields yet. */
                unsafe {
                    let bites = std::mem::transmute::<
                        BlockGroupDescriptor32,
                        [u8; 0x20],
                    >(
                        bg_purt.b32.as_ref().unwrap().clone(),
                    );
                    // assert if they're not equal for now
                    assert_eq!(
                        bitecopy,
                        bites[..bites.len() - 2].to_vec()
                    );
                    bytes.append(
                        &mut bites[..bites.len() - 2].to_vec(),
                    )
                }
                if self.s.uses_64bit() && self.s.desc_size > 32 {
                    bytes.append(
                        &mut self.reader.read_bytes_from_file(
                            bg_start + 0x20,
                            (self.s.desc_size - 0x20) as u64,
                        ),
                    );
                }

                let bg32 = bg_purt.b32.as_ref().unwrap();
                let crcsum = summer::crc16(!0, bytes.clone());
                let bgcrc = bg32.checksum;
                if bgcrc != crcsum {
                    println!(
                        "{} checksum did not match (but it's this \
                         tool that's broken): {:04x} {:04x} {:04x} \
                         {:04x}",
                        "bolo".yellow(),
                        crcsum,
                        !crcsum,
                        !bgcrc,
                        bgcrc
                    )
                } else {
                    println!("checksum matches for bg {:x}", bgid);
                }
            }
        }
    }
}
