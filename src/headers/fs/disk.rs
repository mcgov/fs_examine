use xfat::headers:*;

pub struct Disk {
    partition_table_type: PartitonTableType,
    partitions: Vec<Partition>,
    disk_mbr: Mbr,
}

enum PartitonTableType{
    Mbr,
    Gpt,
}

pub struct PartitionTable{
    pt_type: PartitionTableType,
    pt_offset: u64,
}

enum PartitionType{
    Xfs,
    Ext4,
    Exfat,
}

pub struct Partition{
    p_type: PartitionTableType,
    p_offset:u64,
    p_size: u64,
    p_name: String,
}