/* hash functions

*/

pub const dir_seed: [u32; 4] =
    [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

pub mod dirhash {
    pub fn create_dirhash(filename: &str) -> u32 {
        0
    }
    fn str_to_hash(fname: &str) -> &[u32] {
        &[0u32]
    }
    /*
        static void str2hashbuf_signed(const char *msg, int len, __u32 *buf, int num)
    {
        __u32	pad, val;
        int	i;
        const signed char *scp = (const signed char *) msg;

        pad = (__u32)len | ((__u32)len << 8);
        pad |= pad << 16;

        val = pad;
        if (len > num*4)
            len = num * 4;
        for (i = 0; i < len; i++) {
            val = ((int) scp[i]) + (val << 8);
            if ((i % 4) == 3) {
                *buf++ = val;
                val = pad;
                num--;
            }
        }
        if (--num >= 0)
            *buf++ = val;
        while (--num >= 0)
            *buf++ = pad;
    }*/
}

pub mod tea {
    // this is probably broken, completely untested
    fn transform(v: [u32; 4], k: [u32; 4]) -> u64 {
        let mut sum: u32 = 0; /* set up */
        let delta: u32 = 0x9E3779B9; /* a key schedule constant */
        let mut v0 = v[0];
        let mut v1 = v[1];
        for _ in 0..16 {
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
}

pub mod mdfour {
    use md4::{Digest, Md4};
    pub fn hash(data: &[u8]) -> [u8; 16] {
        let mut hasher = Md4::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
