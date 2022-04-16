use crate::headers::reader::{be_u32_deserialize, be_u64_deserialize, guid_deserialize};
use serde::Deserialize;
use uuid::Uuid;
/*
struct xfs_ondisk_hdr {
    __be32 magic; /* magic number */
    __be32 crc; /* CRC, not logged */
    uuid_t uuid; /* filesystem identifier */
    __be64 owner; /* parent object */
    __be64 blkno; /* location on disk */
    __be64 lsn; /* last modification in log, not logged */
    };
*/

#[derive(Deserialize)]
pub struct XfsOndiskHeader {
    #[serde(deserialize_with = "be_u32_deserialize")]
    pub magic: u32,
    #[serde(deserialize_with = "be_u32_deserialize")]
    pub crc: u32,
    #[serde(deserialize_with = "guid_deserialize")]
    pub uuid: Uuid,
    #[serde(deserialize_with = "be_u64_deserialize")]
    pub owner: u64,
    #[serde(deserialize_with = "be_u64_deserialize")]
    pub blkno: u64,
    #[serde(deserialize_with = "be_u64_deserialize")]
    pub last_modified: u64,
}
