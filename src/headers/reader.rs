use bincode::deserialize;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;

pub trait HasRawHeader<Header: Sized, RawHeader: Sized> {
    fn from_raw(raw: &RawHeader) -> Header;
}

pub fn read_header_from_file_unsafe<
    Header: Sized + HasRawHeader<Header, HeaderRaw>,
    HeaderRaw: Sized + DeserializeOwned,
>(
    file_arg: &str,
) -> Header {
    let header: HeaderRaw = {
        let mut file = File::open(file_arg).unwrap();
        let size = size_of::<HeaderRaw>();
        let mut file_data: Vec<u8> = vec![0; size];
        file.read_exact(&mut file_data[..]).unwrap();
        // read the bytes into the struct
        deserialize::<HeaderRaw>(&file_data[..]).unwrap()
    };
    Header::from_raw(&header)
}
