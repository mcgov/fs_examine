use crate::headers::reader::read_bytes_from_file;
use crc::{Algorithm, Crc};
use std::ops::Range;

pub trait Summable {
    fn ranges_to_zero(&self) -> Vec<Range<usize>>;
    fn range_to_include(&self) -> Range<usize>;
}

pub trait Summable32<Summable = Self> {
    fn crc_parameters(&self) -> &'static Algorithm<u32>;
}
pub trait Summable64<Summable = Self> {
    fn crc_parameters(&self) -> &'static Algorithm<u64>;
}
pub trait Summable16<Summable = Self> {
    fn crc_parameters(&self) -> &'static Algorithm<u16>;
}

// would be nice to have this working later. lifetimes aren't right at the moment.
macro_rules! create_crc_instance {
    ($usize:ty, $algo:expr) => {
        Crc::<$usize>::new($algo)
    };
    (_) => {
        compile_error!("type must be one of: [u8 u16 u32 u64]")
    };
}

pub fn crc32_structure_from_disk<T: Summable + Summable32>(
    file_arg: &str,
    summable: &T,
    start_offset: usize,
) -> u32 {
    let struct_start = start_offset + summable.range_to_include().start;
    let struct_size = summable.range_to_include().end - summable.range_to_include().start;
    let mut struct_bytes = read_bytes_from_file(&file_arg, struct_start as u64, struct_size);
    for range in summable.ranges_to_zero() {
        for i in range.start..range.end {
            struct_bytes[i] = 0;
        }
    }

    let summer = create_crc_instance!(u32, &summable.crc_parameters());
    let mut digest = summer.digest();
    digest.update(&struct_bytes);
    digest.finalize()
}
