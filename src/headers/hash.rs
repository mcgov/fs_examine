/* hash functions

*/

pub mod Tea {}

pub mod Mdfour {
    use md4::{Digest, Md4};
    pub fn hash(data: &[u8]) -> [u8; 16] {
        let mut hasher = Md4::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
