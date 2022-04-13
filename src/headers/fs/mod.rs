pub mod disk;

pub trait DiskPart {
    fn bitness(&self) -> u16;
}
