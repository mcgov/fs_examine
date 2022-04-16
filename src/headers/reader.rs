use bincode::deserialize;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use num_traits::PrimInt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use uuid::*;
extern crate chrono;
extern crate colored;
use chrono::prelude::*;
use colored::*;

pub enum Endianness {
    Big,
    Little,
}

pub fn get_offset_from_block_number(block_0: u64, index: u64, block_size: u64) -> u64 {
    block_0 + index * block_size
}

pub fn read_bytes_from_file(file_arg: &str, offset: u64, size: u64) -> Vec<u8> {
    //let output = format!("Reading from 0x{:X}", offset).yellow();
    //println!("{}", output);
    let mut file = File::open(file_arg).unwrap();
    let res = file.seek(SeekFrom::Start(offset as u64)).unwrap();
    if res != offset {
        panic!("Failed to seek to offset\n");
    }
    let mut file_data: Vec<u8> = vec![0; size.try_into().unwrap()];
    file.read_exact(&mut file_data[..]).unwrap();
    file_data
}

pub fn read_header_from_offset<Header: Sized + DeserializeOwned>(
    file_arg: &str,
    offset: u64,
) -> Header {
    let header: Header = {
        let size = size_of::<Header>() as u64;
        let file_data = read_bytes_from_file(file_arg, offset, size);
        // read the bytes into the struct
        read_header_from_bytevec::<Header>(file_data)
    };
    header
}

pub fn read_header_from_bytevec<Header: Sized + DeserializeOwned>(bytes: Vec<u8>) -> Header {
    // read the bytes into the struct
    deserialize::<Header>(&bytes[..]).unwrap()
}

pub fn read_header_from_bytes<Header: Sized + DeserializeOwned>(bytes: &[u8]) -> Header {
    deserialize::<Header>(&bytes[..]).unwrap()
}

pub fn le_u128_deserialize<'de, D>(d: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u128>::deserialize(d)?;
    data = u128::from_le(data);
    Ok(data)
}

pub fn le_u64_deserialize<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u64>::deserialize(d)?;
    data = u64::from_le(data);
    Ok(data)
}

pub fn be_u32_deserialize<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u32>::deserialize(d)?;
    data = u32::from_be(data);
    Ok(data)
}

pub fn be_u64_deserialize<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u64>::deserialize(d)?;
    data = u64::from_be(data);
    Ok(data)
}

pub fn le_u32_deserialize<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u32>::deserialize(d)?;
    data = u32::from_le(data);
    Ok(data)
}

pub fn le_u16_deserialize<'de, D>(d: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let mut data = <u16>::deserialize(d)?;
    data = u16::from_le(data);
    Ok(data)
}

const GUID_INDEX: [u8; 16] = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];
const UUID_INDEX: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

fn _parse_uuid(uuid: &str, indexes: [u8; 16]) -> [u8; 16] {
    const START_INDEX: [u8; 16] = [0, 2, 4, 6, 9, 11, 14, 16, 19, 21, 24, 26, 28, 30, 32, 34];
    let mut out = [0u8; 16];
    for i in 0..16 {
        let hii = START_INDEX[i] + 0;
        let loi = START_INDEX[i] + 1;
        let bytes = uuid.as_bytes();
        let hib = bytes[hii as usize];
        let lob = bytes[loi as usize];
        let his = String::from_utf8(vec![hib]).unwrap();
        let lob = String::from_utf8(vec![lob]).unwrap();
        let hi = <u8>::from_str_radix(&his, 16).unwrap();
        let lo = <u8>::from_str_radix(&lob, 16).unwrap();

        out[indexes[i] as usize] = (hi << 4) | lo;
    }
    out
}

pub fn parse_guid(uuid: &str) -> [u8; 16] {
    _parse_uuid(uuid, GUID_INDEX)
}

pub fn parse_uuid(uuid: &str) -> [u8; 16] {
    _parse_uuid(uuid, UUID_INDEX)
}

pub fn guid_byteswap(bytes: [u8; 16]) -> [u8; 16] {
    let mut swapped = [0u8; 16];
    for i in 0..bytes.len() {
        swapped[GUID_INDEX[i] as usize] = bytes[i];
    }
    swapped
}
// pretty sure this Uuid class is more trouble than it's worth.
// but not stoked to redo the entire gpt::uuids file.
pub fn guid_deserialize<'de, D>(d: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let data = <[u8; 16]>::deserialize(d)?;
    let guid = guid_byteswap(data);
    Ok(Uuid::from_slice(&guid).unwrap())
}

pub fn bitfield_fetch<T: Sized + PrimInt>(target: T, bitmask: T) -> bool {
    return (target & bitmask) == bitmask;
}

pub fn timestamp_to_string(timestamp: u64) -> String {
    let _timestamp = timestamp as i64;
    let naive = NaiveDateTime::from_timestamp(_timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    format!("{}", newdate)
}

pub fn print_bool(boolean: bool) -> String {
    let result = format!("{:?}", boolean);
    if boolean {
        return result.green().to_string();
    } else {
        return result.red().to_string();
    }
}

pub trait HasHeaderMagic {
    fn magic_field_offset(&self) -> u64;
    fn magic_field_size(&self) -> u64;
    fn magic_field_endianness(&self) -> Endianness;
    fn magic_field_upcast(&self) -> u128;

    // this should check the magic value based on the partition start
    // for FS main headers or from the header start for headers
    fn check_magic_field(&self, file_arg: &str, offset: u64) -> bool {
        let magic_bytes = read_bytes_from_file(
            &file_arg,
            offset + self.magic_field_offset(),
            self.magic_field_size(),
        );

        let found_magic: u128;
        macro_rules! upcast {
            ($endian:ty, $fn:ident) => {
                <$endian>::$fn(&magic_bytes[..]) as u128
            };
        }
        macro_rules! match_types {
            ($endian:ty) => {
                match self.magic_field_size() {
                    8 => {
                        found_magic = upcast!($endian, read_u64);
                    }
                    4 => {
                        found_magic = upcast!($endian, read_u32);
                    }
                    2 => {
                        found_magic = upcast!($endian, read_u16);
                    }
                    _default => {
                        panic!("Invalid size for magic field value");
                    }
                }
            };
        }
        match self.magic_field_endianness() {
            Endianness::Big => {
                match_types!(BigEndian)
            }
            Endianness::Little => {
                match_types!(LittleEndian)
            }
        }
        println!("{} == {} ?", found_magic, self.magic_field_upcast());
        found_magic == self.magic_field_upcast()
    }
}
