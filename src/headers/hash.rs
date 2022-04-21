/* hash functions

*/

pub mod Tea {
    /* this is probably broken, completely untested
    fn en(v: [u32; 2], k: [u32; 4]) -> u64 {
        let mut sum: u32 = 0; /* set up */
        let delta: u32 = 0x9E3779B9; /* a key schedule constant */
        let mut v0 = v[0];
        let mut v1 = v[1];
        for _ in 0..32 {
            /* basic cycle start */
            sum = sum.overflowing_add(delta).0;
            v0 += ((v1 << 4).overflowing_add(k[0]).0)
                ^ (v1.overflowing_add(sum).0)
                ^ ((v1 >> 5).overflowing_add(k[1]).0);
            v1 += ((v0 << 4).overflowing_add(k[2]).0)
                ^ (v0.overflowing_add(sum).0)
                ^ ((v0 >> 5).overflowing_add(k[3]).0);
        } /* end cycle */
        let mut accum = v0.to_ne_bytes().to_vec();
        accum.append(&mut v1.to_ne_bytes().to_vec());

        u64::from_ne_bytes(accum[..].try_into().unwrap())
    }
    */
}

pub mod Mdfour {
    use md4::{Digest, Md4};
    pub fn hash(data: &[u8]) -> [u8; 16] {
        let mut hasher = Md4::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
