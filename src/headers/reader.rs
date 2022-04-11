use bincode::deserialize;
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

pub fn get_offset_from_block_number(block_0: u64, index: u64, block_size: u64) -> u64 {
    block_0 + index * block_size
}

pub fn read_bytes_from_file(file_arg: &str, offset: u64, size: usize) -> Vec<u8> {
    //let output = format!("Reading from 0x{:X}", offset).yellow();
    //println!("{}", output);
    let mut file = File::open(file_arg).unwrap();
    let res = file.seek(SeekFrom::Start(offset)).unwrap();
    if res != offset {
        panic!("Failed to seek to offset\n");
    }
    let mut file_data: Vec<u8> = vec![0; size];
    file.read_exact(&mut file_data[..]).unwrap();
    file_data
}

pub fn read_header_from_offset<Header: Sized + DeserializeOwned>(
    file_arg: &str,
    offset: u64,
) -> Header {
    let header: Header = {
        let size = size_of::<Header>();
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

pub fn uuid_deserialize<'de, D>(d: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    // mixed endian field whyy
    // going to discover there was a library that already did this at some point
    let data = <[u8; 16]>::deserialize(d)?;
    let mut reversed = [0u8; 16];
    let first = &data[..4];
    let second = &data[4..6];
    let third = &data[6..8];
    let fourth = &data[8..10];
    let last = &data[10..];
    let mut counter = 0;
    for i in first.iter().rev() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in second.iter().rev() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in third.iter().rev() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in fourth.iter() {
        reversed[counter] = *i;
        counter += 1;
    }
    for i in last.iter() {
        reversed[counter] = *i;
        counter += 1;
    }
    Ok(uuid::Builder::from_bytes(reversed).build())
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
