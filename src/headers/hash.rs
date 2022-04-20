/* hash functions

*/

pub mod Tea {}

pub mod md4 {
    fn lshift(x: u32, s: u32) -> u32 {
        ((x << s) & 0xFFFFFFFF) | (x >> (32 - s))
    }

    fn F(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | ((!x) & z)
    }

    fn G(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (x & z) | (y & z)
    }

    fn H(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    pub fn round_1(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: u32,
        s: u32,
    ) -> u32 {
        lshift(a + F(b, c, d) + k, s)
    }
    pub fn round_2(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: u32,
        s: u32,
    ) -> u32 {
        lshift(a + G(b, c, d) + k + 0x5A827999, s)
    }
    pub fn round_3(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: u32,
        s: u32,
    ) -> u32 {
        lshift(a + H(b, c, d) + k + 0x6ED9EBA1, s)
    }

    pub fn init() -> md4_ctx {
        md4_ctx {
            hash: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476],
            byte_count: 0,
            block: [0u32; 16],
        }
    }

    pub fn hash_transform(ctx: md4_ctx) -> md4_ctx {
        let hash: [u32; 4] = ctx.hash;
        let block: [u32; 16] = ctx.block;
        let mut a = hash[0];
        let mut b = hash[1];
        let mut c = hash[2];
        let mut d = hash[3];

        a = round_1(a, b, c, d, block[0], 3);
        a = round_1(d, a, b, c, block[1], 7);
        a = round_1(c, d, a, b, block[2], 11);
        a = round_1(b, c, d, a, block[3], 19);
        a = round_1(a, b, c, d, block[4], 3);
        a = round_1(d, a, b, c, block[5], 7);
        a = round_1(c, d, a, b, block[6], 11);
        a = round_1(b, c, d, a, block[7], 19);
        a = round_1(a, b, c, d, block[8], 3);
        a = round_1(d, a, b, c, block[9], 7);
        a = round_1(c, d, a, b, block[10], 11);
        a = round_1(b, c, d, a, block[11], 19);
        a = round_1(a, b, c, d, block[12], 3);
        a = round_1(d, a, b, c, block[13], 7);
        a = round_1(c, d, a, b, block[14], 11);
        a = round_1(b, c, d, a, block[15], 19);

        a = round_2(a, b, c, d, block[0], 3);
        a = round_2(d, a, b, c, block[4], 5);
        a = round_2(c, d, a, b, block[8], 9);
        a = round_2(b, c, d, a, block[12], 13);
        a = round_2(a, b, c, d, block[1], 3);
        a = round_2(d, a, b, c, block[5], 5);
        a = round_2(c, d, a, b, block[9], 9);
        a = round_2(b, c, d, a, block[13], 13);
        a = round_2(a, b, c, d, block[2], 3);
        a = round_2(d, a, b, c, block[6], 5);
        a = round_2(c, d, a, b, block[10], 9);
        a = round_2(b, c, d, a, block[14], 13);
        a = round_2(a, b, c, d, block[3], 3);
        a = round_2(d, a, b, c, block[7], 5);
        a = round_2(c, d, a, b, block[11], 9);
        a = round_2(b, c, d, a, block[15], 13);
        a = round_3(a, b, c, d, block[0], 3);
        a = round_3(d, a, b, c, block[8], 9);
        a = round_3(c, d, a, b, block[4], 11);
        a = round_3(b, c, d, a, block[12], 15);
        a = round_3(a, b, c, d, block[2], 3);
        a = round_3(d, a, b, c, block[10], 9);
        a = round_3(c, d, a, b, block[6], 11);
        a = round_3(b, c, d, a, block[14], 15);
        a = round_3(a, b, c, d, block[1], 3);
        a = round_3(d, a, b, c, block[9], 9);
        a = round_3(c, d, a, b, block[5], 11);
        a = round_3(b, c, d, a, block[13], 15);
        a = round_3(a, b, c, d, block[3], 3);
        a = round_3(d, a, b, c, block[11], 9);
        a = round_3(c, d, a, b, block[7], 11);
        a = round_3(b, c, d, a, block[15], 15);

        hash[0] += a;
        hash[1] += b;
        hash[2] += c;
        hash[3] += d;

        md4_ctx {
            hash: hash,
            block: block,
            byte_count: ctx.byte_count,
        }
    }
    struct md4_ctx {
        hash: [u32; 4],
        block: [u32; 16],
        byte_count: u64,
    }
}

/*void encrypt (uint32_t v[2], const uint32_t k[4]) {
    uint32_t v0=v[0], v1=v[1], sum=0, i;           /* set up */
    uint32_t delta=0x9E3779B9;                     /* a key schedule constant */
    uint32_t k0=k[0], k1=k[1], k2=k[2], k3=k[3];   /* cache key */
    for (i=0; i<32; i++) {                         /* basic cycle start */
        sum += delta;
        v0 += ((v1<<4) + k0) ^ (v1 + sum) ^ ((v1>>5) + k1);
        v1 += ((v0<<4) + k2) ^ (v0 + sum) ^ ((v0>>5) + k3);
    }                                              /* end cycle */
    v[0]=v0; v[1]=v1;
}

void decrypt (uint32_t v[2], const uint32_t k[4]) {
    uint32_t v0=v[0], v1=v[1], sum=0xC6EF3720, i;  /* set up; sum is (delta << 5) & 0xFFFFFFFF */
    uint32_t delta=0x9E3779B9;                     /* a key schedule constant */
    uint32_t k0=k[0], k1=k[1], k2=k[2], k3=k[3];   /* cache key */
    for (i=0; i<32; i++) {                         /* basic cycle start */
        v1 -= ((v0<<4) + k2) ^ (v0 + sum) ^ ((v0>>5) + k3);
        v0 -= ((v1<<4) + k0) ^ (v1 + sum) ^ ((v1>>5) + k1);
        sum -= delta;
    }                                              /* end cycle */
    v[0]=v0; v[1]=v1;
}
*/
