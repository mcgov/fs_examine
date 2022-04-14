use super::reader::{print_bool, read_bytes_from_file};
use crc::{Algorithm, Crc};
use std::ops::Range;

pub trait Summable {
    fn ranges_to_zero(&self) -> Vec<Range<usize>>;
    fn range_to_include(&self) -> Range<usize>;
    fn data_to_include(&self) -> Vec<u8>;
}

pub trait Summable16<T: Summable = Self> {
    fn validate_checksum(&self, sumcheck: u16) -> bool;
    fn crc_parameters(&self) -> &'static Algorithm<u16>;
}
pub trait Summable32<T: Summable = Self> {
    fn validate_checksum(&self, sumcheck: u32) -> bool;
    fn crc_parameters(&self) -> &'static Algorithm<u32>;
}
pub trait Summable64<T: Summable = Self> {
    fn validate_checksum(&self, sumcheck: u64) -> bool;
    fn crc_parameters(&self) -> &'static Algorithm<u64>;
}

pub fn struct_validate_checksum32<Structure: Summable + Summable32>(
    file_arg: &str,
    instance: &Structure,
    offset: u64,
) {
    let chksum = crc32_structure_from_disk::<Structure>(&file_arg, &instance, offset);
    print_valid_checksum(stringify!(Gpt), instance.validate_checksum(chksum));
}

pub fn struct_validate_checksum16<Structure: Summable + Summable16>(
    file_arg: &str,
    instance: &Structure,
    offset: u64,
) {
    let chksum = crc16_structure_from_disk::<Structure>(&file_arg, &instance, offset);
    print_valid_checksum(stringify!(Gpt), instance.validate_checksum(chksum));
}

pub fn print_valid_checksum(name: &str, result: bool) {
    println!("Valid checksum {}?: {}", name, print_bool(result));
}
fn gather_and_include<T: Summable>(file_arg: &str, summable: &T, start_offset: u64) -> Vec<u8> {
    let struct_start = start_offset + summable.range_to_include().start as u64;
    let struct_size = (summable.range_to_include().end - summable.range_to_include().start) as u64;
    let mut struct_bytes = read_bytes_from_file(&file_arg, struct_start, struct_size as u64);
    for range in summable.ranges_to_zero() {
        for i in range.start..range.end {
            struct_bytes[i] = 0;
        }
    }
    struct_bytes
}

pub fn crc16_structure_from_disk<T: Summable + Summable16>(
    file_arg: &str,
    summable: &T,
    start_offset: u64,
) -> u16 {
    let struct_bytes = gather_and_include(file_arg, summable, start_offset);
    let summer = Crc::<u16>::new(&summable.crc_parameters());
    let mut digest = summer.digest();
    digest.update(&struct_bytes);
    digest.finalize()
}

pub fn crc32_structure_from_disk<T: Summable + Summable32>(
    file_arg: &str,
    summable: &T,
    start_offset: u64,
) -> u32 {
    let struct_bytes = gather_and_include(file_arg, summable, start_offset);
    let summer = Crc::<u32>::new(&summable.crc_parameters());
    let mut digest = summer.digest();
    digest.update(&struct_bytes);
    digest.finalize()
}

pub fn crc32_bytes_from_disk(
    file_arg: &str,
    algorithm: &'static Algorithm<u32>,
    start_offset: u64,
    size: u64,
) -> u32 {
    let struct_bytes = read_bytes_from_file(&file_arg, start_offset, size);
    let summer = Crc::<u32>::new(algorithm);
    let mut digest = summer.digest();
    digest.update(&struct_bytes);
    digest.finalize()
}

pub fn crc32_bytes(file_arg: &str, algorithm: &'static Algorithm<u32>, bytes: Vec<u8>) -> u32 {
    let summer = Crc::<u32>::new(algorithm);
    let mut digest = summer.digest();
    digest.update(&bytes);
    digest.finalize()
}

pub fn crc16_bytes(file_arg: &str, algorithm: &'static Algorithm<u16>, bytes: Vec<u8>) -> u16 {
    let summer = Crc::<u16>::new(algorithm);
    let mut digest = summer.digest();
    digest.update(&bytes);
    digest.finalize()
}
