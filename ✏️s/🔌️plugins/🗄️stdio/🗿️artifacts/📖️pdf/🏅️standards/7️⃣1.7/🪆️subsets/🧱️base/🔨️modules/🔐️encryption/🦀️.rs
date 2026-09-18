//! 🔐️ The standard security handler (ISO 32000-1 §7.6 and the ISO 32000-2 / Adobe Extension
//! Level 3 AES-256 revision 6): password-derived file keys (Algorithms 2 and 2.A/2.B), owner and
//! user password entries (Algorithms 3–5, 8–10), per-object keys (Algorithm 1), and the RC4 /
//! AES-CBC ciphers with the MD5 / SHA-2 digests they need — all first-party (RFC 1321, FIPS 197,
//! FIPS 180-4), zero runtime dependencies.

use super::lexer::{dict_get, dict_i64, dict_name, PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfEncryption, PdfEncryptionAlgorithm, PdfObject};
use semio_framework_hash::Sha256;

//#region 🔖️Md5
const MD5_SHIFTS: [u32; 64] = [7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21];
const MD5_K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6,
    0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244, 0x432aff97,
    0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

/// 🧮 MD5 (RFC 1321).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn md5(input: &[u8]) -> [u8; 16] {
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;
    let mut message = input.to_vec();
    let bit_len = (input.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_le_bytes());
    for chunk in message.chunks(64) {
        let mut m = [0u32; 16];
        for (index, word) in m.iter_mut().enumerate() {
            *word = u32::from_le_bytes([chunk[index * 4], chunk[index * 4 + 1], chunk[index * 4 + 2], chunk[index * 4 + 3]]);
        }
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f = f.wrapping_add(a).wrapping_add(MD5_K[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(MD5_SHIFTS[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&a0.to_le_bytes());
    out[4..8].copy_from_slice(&b0.to_le_bytes());
    out[8..12].copy_from_slice(&c0.to_le_bytes());
    out[12..16].copy_from_slice(&d0.to_le_bytes());
    out
}
//#endregion 🔖️Md5

//#region 🔖️Sha2
const SHA512_K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235,
    0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725,
    0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b, 0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218,
    0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b, 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sha512_family(input: &[u8], initial: [u64; 8], output_len: usize) -> Vec<u8> {
    let mut state = initial;
    let mut message = input.to_vec();
    let bit_len = (input.len() as u128).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 128 != 112 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in message.chunks(128) {
        let mut w = [0u64; 80];
        for (index, word) in w.iter_mut().enumerate().take(16) {
            *word = u64::from_be_bytes(chunk[index * 8..index * 8 + 8].try_into().expect("8 bytes"));
        }
        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = state;
        for i in 0..80 {
            let s1 = v[4].rotate_right(14) ^ v[4].rotate_right(18) ^ v[4].rotate_right(41);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(SHA512_K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(28) ^ v[0].rotate_right(34) ^ v[0].rotate_right(39);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(t1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = t1.wrapping_add(t2);
        }
        for (s, x) in state.iter_mut().zip(v) {
            *s = s.wrapping_add(x);
        }
    }
    let mut out = Vec::with_capacity(64);
    for word in state {
        out.extend_from_slice(&word.to_be_bytes());
    }
    out.truncate(output_len);
    out
}

/// 🧮 SHA-384 (FIPS 180-4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sha384(input: &[u8]) -> Vec<u8> {
    sha512_family(input, [0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939, 0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4], 48)
}

/// 🧮 SHA-512 (FIPS 180-4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sha512(input: &[u8]) -> Vec<u8> {
    sha512_family(input, [0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1, 0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179], 64)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sha256(input: &[u8]) -> Vec<u8> {
    Sha256::digest(input).to_vec()
}
//#endregion 🔖️Sha2

//#region 🔖️Rc4
/// 🔑 RC4 stream cipher (encryption and decryption are the same operation).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut s: [u8; 256] = [0; 256];
    for (i, slot) in s.iter_mut().enumerate() {
        *slot = i as u8;
    }
    let mut j: u8 = 0;
    let key = if key.is_empty() { &[0u8][..] } else { key };
    for i in 0..256 {
        j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
        s.swap(i, j as usize);
    }
    let mut out = Vec::with_capacity(data.len());
    let (mut i, mut j) = (0u8, 0u8);
    for byte in data {
        i = i.wrapping_add(1);
        j = j.wrapping_add(s[i as usize]);
        s.swap(i as usize, j as usize);
        let k = s[(s[i as usize].wrapping_add(s[j as usize])) as usize];
        out.push(byte ^ k);
    }
    out
}
//#endregion 🔖️Rc4

//#region 🔖️Aes
const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a,
    0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
    0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf, 0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aes_inverse_sbox() -> [u8; 256] {
    let mut inverse = [0u8; 256];
    for (index, value) in AES_SBOX.iter().enumerate() {
        inverse[*value as usize] = index as u8;
    }
    inverse
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xtime(value: u8) -> u8 {
    (value << 1) ^ if value & 0x80 != 0 { 0x1b } else { 0 }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut product = 0u8;
    while b != 0 {
        if b & 1 != 0 {
            product ^= a;
        }
        a = xtime(a);
        b >>= 1;
    }
    product
}

/// 🔑 An expanded AES key schedule (FIPS 197 §5.2) for a 128- or 256-bit key.
pub struct AesKey {
    round_keys: Vec<[u8; 16]>,
}

impl AesKey {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(key: &[u8]) -> Self {
        let nk = key.len() / 4;
        let rounds = nk + 6;
        let total_words = 4 * (rounds + 1);
        let mut words: Vec<[u8; 4]> = key.chunks(4).map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]]).collect();
        let mut rcon = 1u8;
        for i in nk..total_words {
            let mut temp = words[i - 1];
            if i % nk == 0 {
                temp = [AES_SBOX[temp[1] as usize] ^ rcon, AES_SBOX[temp[2] as usize], AES_SBOX[temp[3] as usize], AES_SBOX[temp[0] as usize]];
                rcon = xtime(rcon);
            } else if nk > 6 && i % nk == 4 {
                temp = [AES_SBOX[temp[0] as usize], AES_SBOX[temp[1] as usize], AES_SBOX[temp[2] as usize], AES_SBOX[temp[3] as usize]];
            }
            let previous = words[i - nk];
            words.push([previous[0] ^ temp[0], previous[1] ^ temp[1], previous[2] ^ temp[2], previous[3] ^ temp[3]]);
        }
        let round_keys = words
            .chunks(4)
            .map(|chunk| {
                let mut round = [0u8; 16];
                for (index, word) in chunk.iter().enumerate() {
                    round[index * 4..index * 4 + 4].copy_from_slice(word);
                }
                round
            })
            .collect();
        Self { round_keys }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn add_round_key(state: &mut [u8; 16], key: &[u8; 16]) {
        for (s, k) in state.iter_mut().zip(key) {
            *s ^= k;
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = *block;
        let rounds = self.round_keys.len() - 1;
        Self::add_round_key(&mut state, &self.round_keys[0]);
        for round in 1..=rounds {
            for byte in state.iter_mut() {
                *byte = AES_SBOX[*byte as usize];
            }
            let mut shifted = state;
            for column in 0..4 {
                for row in 0..4 {
                    shifted[column * 4 + row] = state[((column + row) % 4) * 4 + row];
                }
            }
            state = shifted;
            if round != rounds {
                for column in 0..4 {
                    let c = [state[column * 4], state[column * 4 + 1], state[column * 4 + 2], state[column * 4 + 3]];
                    state[column * 4] = gmul(c[0], 2) ^ gmul(c[1], 3) ^ c[2] ^ c[3];
                    state[column * 4 + 1] = c[0] ^ gmul(c[1], 2) ^ gmul(c[2], 3) ^ c[3];
                    state[column * 4 + 2] = c[0] ^ c[1] ^ gmul(c[2], 2) ^ gmul(c[3], 3);
                    state[column * 4 + 3] = gmul(c[0], 3) ^ c[1] ^ c[2] ^ gmul(c[3], 2);
                }
            }
            Self::add_round_key(&mut state, &self.round_keys[round]);
        }
        state
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn decrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let inverse = aes_inverse_sbox();
        let mut state = *block;
        let rounds = self.round_keys.len() - 1;
        Self::add_round_key(&mut state, &self.round_keys[rounds]);
        for round in (0..rounds).rev() {
            let mut shifted = state;
            for column in 0..4 {
                for row in 0..4 {
                    shifted[((column + row) % 4) * 4 + row] = state[column * 4 + row];
                }
            }
            state = shifted;
            for byte in state.iter_mut() {
                *byte = inverse[*byte as usize];
            }
            Self::add_round_key(&mut state, &self.round_keys[round]);
            if round != 0 {
                for column in 0..4 {
                    let c = [state[column * 4], state[column * 4 + 1], state[column * 4 + 2], state[column * 4 + 3]];
                    state[column * 4] = gmul(c[0], 14) ^ gmul(c[1], 11) ^ gmul(c[2], 13) ^ gmul(c[3], 9);
                    state[column * 4 + 1] = gmul(c[0], 9) ^ gmul(c[1], 14) ^ gmul(c[2], 11) ^ gmul(c[3], 13);
                    state[column * 4 + 2] = gmul(c[0], 13) ^ gmul(c[1], 9) ^ gmul(c[2], 14) ^ gmul(c[3], 11);
                    state[column * 4 + 3] = gmul(c[0], 11) ^ gmul(c[1], 13) ^ gmul(c[2], 9) ^ gmul(c[3], 14);
                }
            }
        }
        state
    }
}

/// 🔒 AES-CBC with PKCS#5 padding; the 16-byte IV is prepended to the output (§7.6.2 AESV2/V3).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn aes_cbc_encrypt(key: &[u8], iv: [u8; 16], data: &[u8]) -> Vec<u8> {
    let aes = AesKey::new(key);
    let mut padded = data.to_vec();
    let pad = 16 - (padded.len() % 16);
    padded.extend(std::iter::repeat_n(pad as u8, pad));
    let mut out = Vec::with_capacity(16 + padded.len());
    out.extend_from_slice(&iv);
    let mut previous = iv;
    for chunk in padded.chunks(16) {
        let mut block = [0u8; 16];
        for (index, byte) in chunk.iter().enumerate() {
            block[index] = byte ^ previous[index];
        }
        let encrypted = aes.encrypt_block(&block);
        out.extend_from_slice(&encrypted);
        previous = encrypted;
    }
    out
}

/// 🔓 AES-CBC decrypt of `iv || ciphertext`, stripping PKCS#5 padding (lenient about a missing or
/// damaged pad, like shipping readers).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn aes_cbc_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    if data.len() < 16 {
        return Vec::new();
    }
    let aes = AesKey::new(key);
    let mut previous: [u8; 16] = data[..16].try_into().expect("16 bytes");
    let mut out = Vec::with_capacity(data.len());
    for chunk in data[16..].chunks(16) {
        if chunk.len() < 16 {
            break;
        }
        let block: [u8; 16] = chunk.try_into().expect("16 bytes");
        let decrypted = aes.decrypt_block(&block);
        for (index, byte) in decrypted.iter().enumerate() {
            out.push(byte ^ previous[index]);
        }
        previous = block;
    }
    if let Some(&pad) = out.last() {
        if (1..=16).contains(&pad) && out.len() >= pad as usize && out[out.len() - pad as usize..].iter().all(|b| *b == pad) {
            out.truncate(out.len() - pad as usize);
        }
    }
    out
}

/// 🔒 AES-CBC with no padding and a zero IV — the AES-256 revision 6 key wrapping (Algorithm 8).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aes_cbc_no_pad(key: &[u8], iv: [u8; 16], data: &[u8], encrypt: bool) -> Vec<u8> {
    let aes = AesKey::new(key);
    let mut out = Vec::with_capacity(data.len());
    let mut previous = iv;
    for chunk in data.chunks(16) {
        let block: [u8; 16] = chunk.try_into().expect("16-byte multiple");
        if encrypt {
            let mut input = block;
            for (index, byte) in input.iter_mut().enumerate() {
                *byte ^= previous[index];
            }
            let encrypted = aes.encrypt_block(&input);
            out.extend_from_slice(&encrypted);
            previous = encrypted;
        } else {
            let decrypted = aes.decrypt_block(&block);
            for (index, byte) in decrypted.iter().enumerate() {
                out.push(byte ^ previous[index]);
            }
            previous = block;
        }
    }
    out
}
//#endregion 🔖️Aes

//#region 🔖️StandardSecurityHandler
const PAD: [u8; 32] = [0x28, 0xBF, 0x4E, 0x5E, 0x4E, 0x75, 0x8A, 0x41, 0x64, 0x00, 0x4E, 0x56, 0xFF, 0xFA, 0x01, 0x08, 0x2E, 0x2E, 0x00, 0xB6, 0xD0, 0x68, 0x3E, 0x80, 0x2F, 0x0C, 0xA9, 0xFE, 0x64, 0x53, 0x69, 0x7A];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn padded_password(password: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let take = password.len().min(32);
    out[..take].copy_from_slice(&password[..take]);
    out[take..].copy_from_slice(&PAD[..32 - take]);
    out
}

/// 🔑 Algorithm 2: file encryption key from the user password (revisions 2–4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn legacy_file_key(password: &[u8], owner_entry: &[u8], permissions: i32, document_id: &[u8], revision: u32, key_length: usize, encrypt_metadata: bool) -> Vec<u8> {
    let mut input = Vec::new();
    input.extend_from_slice(&padded_password(password));
    input.extend_from_slice(&owner_entry[..owner_entry.len().min(32)]);
    input.extend_from_slice(&(permissions as u32).to_le_bytes());
    input.extend_from_slice(document_id);
    if revision >= 4 && !encrypt_metadata {
        input.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
    }
    let mut hash = md5(&input).to_vec();
    let n = if revision == 2 { 5 } else { key_length.clamp(5, 16) };
    if revision >= 3 {
        for _ in 0..50 {
            hash = md5(&hash[..n]).to_vec();
        }
    }
    hash.truncate(n);
    hash
}

/// 🔑 Algorithm 3: the `/O` entry from owner and user passwords (revisions 2–4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn legacy_owner_entry(owner_password: &[u8], user_password: &[u8], revision: u32, key_length: usize) -> Vec<u8> {
    let mut hash = md5(&padded_password(owner_password)).to_vec();
    let n = if revision == 2 { 5 } else { key_length.clamp(5, 16) };
    if revision >= 3 {
        for _ in 0..50 {
            hash = md5(&hash[..n]).to_vec();
        }
    }
    let key = &hash[..n];
    let mut out = rc4(key, &padded_password(user_password));
    if revision >= 3 {
        for i in 1..=19u8 {
            let round_key: Vec<u8> = key.iter().map(|byte| byte ^ i).collect();
            out = rc4(&round_key, &out);
        }
    }
    out
}

/// 🔑 Algorithms 4/5: the `/U` entry from the file key.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn legacy_user_entry(file_key: &[u8], document_id: &[u8], revision: u32) -> Vec<u8> {
    if revision == 2 {
        return rc4(file_key, &PAD);
    }
    let mut input = PAD.to_vec();
    input.extend_from_slice(document_id);
    let mut out = rc4(file_key, &md5(&input));
    for i in 1..=19u8 {
        let round_key: Vec<u8> = file_key.iter().map(|byte| byte ^ i).collect();
        out = rc4(&round_key, &out);
    }
    out.extend_from_slice(&[0u8; 16]);
    out
}

/// 🔑 Algorithm 2.B (revision 6): the SHA-2 iteration hash of the AES-256 handler.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn revision6_hash(password: &[u8], salt: &[u8], user_data: &[u8]) -> Vec<u8> {
    let mut input = password.to_vec();
    input.extend_from_slice(salt);
    input.extend_from_slice(user_data);
    let mut k = sha256(&input);
    let mut round = 0usize;
    loop {
        let mut k1 = Vec::new();
        for _ in 0..64 {
            k1.extend_from_slice(password);
            k1.extend_from_slice(&k);
            k1.extend_from_slice(user_data);
        }
        let iv: [u8; 16] = k[16..32].try_into().expect("16 bytes");
        let e = aes_cbc_no_pad(&k[..16], iv, &k1, true);
        let modulo = e[..16].iter().map(|byte| *byte as u32).sum::<u32>() % 3;
        k = match modulo {
            0 => sha256(&e),
            1 => sha384(&e),
            _ => sha512(&e),
        };
        round += 1;
        if round >= 64 && (*e.last().expect("non-empty") as usize) <= round - 32 {
            break;
        }
    }
    k.truncate(32);
    k
}

/// 🔐 A ready-to-use per-document cipher: the file key plus how strings and streams are wrapped.
#[derive(Clone, Debug)]
pub struct Decryptor {
    file_key: Vec<u8>,
    aes: bool,
    identity_strings: bool,
    identity_streams: bool,
    revision: u32,
}

impl Decryptor {
    /// 🔑 Algorithm 1: the object key for `(num, gen)` (revisions ≤ 4); revision 6 uses the file
    /// key directly.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn object_key(&self, num: u32, gen: u16) -> Vec<u8> {
        if self.revision >= 5 {
            return self.file_key.clone();
        }
        let mut input = self.file_key.clone();
        input.extend_from_slice(&num.to_le_bytes()[..3]);
        input.extend_from_slice(&gen.to_le_bytes()[..2]);
        if self.aes {
            input.extend_from_slice(&[0x73, 0x41, 0x6C, 0x54]);
        }
        let hash = md5(&input);
        let n = (self.file_key.len() + 5).min(16);
        hash[..n].to_vec()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn decrypt_bytes(&self, data: &[u8], num: u32, gen: u16) -> Vec<u8> {
        let key = self.object_key(num, gen);
        if self.aes {
            aes_cbc_decrypt(&key, data)
        } else {
            rc4(&key, data)
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn encrypt_bytes(&self, data: &[u8], num: u32, gen: u16) -> Vec<u8> {
        let key = self.object_key(num, gen);
        if self.aes {
            let mut iv = [0u8; 16];
            let seed = md5(&[key.as_slice(), &num.to_le_bytes(), &gen.to_le_bytes(), &(data.len() as u64).to_le_bytes()].concat());
            iv.copy_from_slice(&seed);
            aes_cbc_encrypt(&key, iv, data)
        } else {
            rc4(&key, data)
        }
    }

    /// 🔓 Decrypts every string and stream inside one indirect object.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn decrypt_object(&self, value: PdfObject, num: u32, gen: u16) -> PdfObject {
        self.map_object(value, num, gen, true)
    }

    /// 🔒 Encrypts every string and stream inside one indirect object.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn encrypt_object(&self, value: PdfObject, num: u32, gen: u16) -> PdfObject {
        self.map_object(value, num, gen, false)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn map_object(&self, value: PdfObject, num: u32, gen: u16, decrypt: bool) -> PdfObject {
        match value {
            PdfObject::Str(bytes) if !self.identity_strings => PdfObject::Str(if decrypt { self.decrypt_bytes(&bytes, num, gen) } else { self.encrypt_bytes(&bytes, num, gen) }),
            PdfObject::Array(items) => PdfObject::Array(items.into_iter().map(|item| self.map_object(item, num, gen, decrypt)).collect()),
            PdfObject::Dict(entries) => PdfObject::Dict(entries.into_iter().map(|entry| PdfDictEntry { key: entry.key, value: self.map_object(entry.value, num, gen, decrypt) }).collect()),
            PdfObject::Stream { dict, data, filters } => {
                let identity = self.identity_streams || dict.iter().any(|entry| entry.key == "Type" && entry.value.as_name() == Some("XRef")) || dict.iter().any(|entry| entry.key == "Filter" && matches!(&entry.value, PdfObject::Name(name) if name == "Crypt")) || dict.iter().any(|entry| entry.key == "Filter" && matches!(&entry.value, PdfObject::Array(items) if items.first().and_then(PdfObject::as_name) == Some("Crypt")));
                let data = if identity {
                    data
                } else if decrypt {
                    self.decrypt_bytes(&data, num, gen)
                } else {
                    self.encrypt_bytes(&data, num, gen)
                };
                PdfObject::Stream { dict: dict.into_iter().map(|entry| PdfDictEntry { key: entry.key, value: self.map_object(entry.value, num, gen, decrypt) }).collect(), data, filters }
            }
            other => other,
        }
    }
}

/// 🔐 Reads a trailer `/Encrypt` dictionary, authenticates `password` (as user, then as owner)
/// and returns the cipher plus the typed encryption parameters.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn open_standard_security(encrypt: &[PdfDictEntry], document_id: &[u8], password: &str) -> PResult<(Decryptor, PdfEncryption)> {
    if dict_name(encrypt, "Filter") != Some("Standard") {
        return Err(PdfEngineError::Unsupported(format!("security handler /{} is not the standard handler", dict_name(encrypt, "Filter").unwrap_or("?"))));
    }
    let v = dict_i64(encrypt, "V").unwrap_or(0) as u32;
    let r = dict_i64(encrypt, "R").unwrap_or(2) as u32;
    let length_bits = dict_i64(encrypt, "Length").unwrap_or(40) as usize;
    let permissions = dict_i64(encrypt, "P").unwrap_or(-1) as i32;
    let owner_entry = dict_get(encrypt, "O").and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
    let user_entry = dict_get(encrypt, "U").and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
    let encrypt_metadata = dict_get(encrypt, "EncryptMetadata").and_then(PdfObject::as_bool).unwrap_or(true);
    let (stream_filter, string_filter, crypt_aes, crypt_length) = if v >= 4 {
        let stmf = dict_name(encrypt, "StmF").unwrap_or("Identity").to_string();
        let strf = dict_name(encrypt, "StrF").unwrap_or("Identity").to_string();
        let filter_name = if stmf != "Identity" { stmf.clone() } else { strf.clone() };
        let cf = dict_get(encrypt, "CF").and_then(|cf| cf.dict_get(&filter_name));
        let cfm = cf.and_then(|cf| cf.dict_get("CFM")).and_then(PdfObject::as_name).unwrap_or("None").to_string();
        let cf_length = cf.and_then(|cf| cf.dict_get("Length")).and_then(PdfObject::as_i64).map(|length| if length <= 40 { length as usize * 8 } else { length as usize });
        (stmf, strf, cfm.starts_with("AESV"), cf_length)
    } else {
        ("StdCF".into(), "StdCF".into(), false, None)
    };
    let key_bits = crypt_length.unwrap_or(length_bits);
    let algorithm = match (r, crypt_aes, key_bits) {
        (6, _, _) | (5, _, _) => PdfEncryptionAlgorithm::Aes256,
        (_, true, _) => PdfEncryptionAlgorithm::Aes128,
        (_, false, bits) if bits <= 40 => PdfEncryptionAlgorithm::Rc4_40,
        _ => PdfEncryptionAlgorithm::Rc4_128,
    };
    let password_bytes = password.as_bytes();
    let file_key = if r >= 5 {
        if user_entry.len() < 48 || owner_entry.len() < 48 {
            return Err(PdfEngineError::Malformed("revision 6 /U or /O shorter than 48 bytes".into()));
        }
        let user_hash = &user_entry[..32];
        let user_validation_salt = &user_entry[32..40];
        let user_key_salt = &user_entry[40..48];
        let ue = dict_get(encrypt, "UE").and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
        let oe = dict_get(encrypt, "OE").and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
        let truncated: Vec<u8> = password_bytes.iter().copied().take(127).collect();
        let hash = |salt: &[u8], user_data: &[u8]| -> Vec<u8> {
            if r == 5 {
                let mut input = truncated.clone();
                input.extend_from_slice(salt);
                input.extend_from_slice(user_data);
                sha256(&input)
            } else {
                revision6_hash(&truncated, salt, user_data)
            }
        };
        if hash(user_validation_salt, &[]) == user_hash {
            let intermediate = hash(user_key_salt, &[]);
            aes_cbc_no_pad(&intermediate, [0u8; 16], &ue[..ue.len().min(32)], false)
        } else {
            let owner_hash = &owner_entry[..32];
            let owner_validation_salt = &owner_entry[32..40];
            let owner_key_salt = &owner_entry[40..48];
            if hash(owner_validation_salt, &user_entry[..48]) != owner_hash {
                return Err(PdfEngineError::Unsupported("encrypted document: the password does not open it".into()));
            }
            let intermediate = hash(owner_key_salt, &user_entry[..48]);
            aes_cbc_no_pad(&intermediate, [0u8; 16], &oe[..oe.len().min(32)], false)
        }
    } else {
        let key_length = key_bits / 8;
        let candidate = legacy_file_key(password_bytes, &owner_entry, permissions, document_id, r, key_length, encrypt_metadata);
        let expected = legacy_user_entry(&candidate, document_id, r);
        let matches = if r == 2 { expected == user_entry } else { expected[..16] == user_entry[..user_entry.len().min(16)] };
        if matches {
            candidate
        } else {
            // 🔑 Algorithm 7: try the password as the owner password.
            let mut hash = md5(&padded_password(password_bytes)).to_vec();
            let n = if r == 2 { 5 } else { key_length.clamp(5, 16) };
            if r >= 3 {
                for _ in 0..50 {
                    hash = md5(&hash[..n]).to_vec();
                }
            }
            let owner_key = &hash[..n];
            let mut user_password = owner_entry[..owner_entry.len().min(32)].to_vec();
            if r == 2 {
                user_password = rc4(owner_key, &user_password);
            } else {
                for i in (0..=19u8).rev() {
                    let round_key: Vec<u8> = owner_key.iter().map(|byte| byte ^ i).collect();
                    user_password = rc4(&round_key, &user_password);
                }
            }
            let candidate = legacy_file_key(&user_password, &owner_entry, permissions, document_id, r, key_length, encrypt_metadata);
            let expected = legacy_user_entry(&candidate, document_id, r);
            let matches = if r == 2 { expected == user_entry } else { expected[..16] == user_entry[..user_entry.len().min(16)] };
            if !matches {
                return Err(PdfEngineError::Unsupported("encrypted document: the password does not open it".into()));
            }
            candidate
        }
    };
    let decryptor = Decryptor { file_key, aes: matches!(algorithm, PdfEncryptionAlgorithm::Aes128 | PdfEncryptionAlgorithm::Aes256), identity_strings: v >= 4 && string_filter == "Identity", identity_streams: v >= 4 && stream_filter == "Identity", revision: r };
    Ok((decryptor, PdfEncryption { algorithm, permissions, user_password: password.to_string(), owner_password: None, encrypt_metadata }))
}

/// 🔐 Builds the `/Encrypt` dictionary and cipher for writing a document with `parameters`.
/// `document_id` is the first element of the trailer `/ID`; `seed` makes the salts and file key
/// deterministic for a given document (the writer derives it from content, never from a clock).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn seal_standard_security(parameters: &PdfEncryption, document_id: &[u8], seed: &[u8]) -> (Decryptor, Vec<PdfDictEntry>) {
    let user_password = parameters.user_password.as_bytes();
    let owner_password = parameters.owner_password.as_deref().map_or(user_password, str::as_bytes);
    let permissions = (parameters.permissions as u32 | 0xFFFF_F0C0) as i32;
    let mut entries = vec![PdfDictEntry::new("Filter", PdfObject::name("Standard"))];
    let (decryptor, tail) = match parameters.algorithm {
        PdfEncryptionAlgorithm::Rc4_40 | PdfEncryptionAlgorithm::Rc4_128 | PdfEncryptionAlgorithm::Aes128 => {
            let (v, r, key_length, aes) = match parameters.algorithm {
                PdfEncryptionAlgorithm::Rc4_40 => (1, 2, 5, false),
                PdfEncryptionAlgorithm::Rc4_128 => (2, 3, 16, false),
                _ => (4, 4, 16, true),
            };
            let owner_entry = legacy_owner_entry(owner_password, user_password, r, key_length);
            let file_key = legacy_file_key(user_password, &owner_entry, permissions, document_id, r, key_length, parameters.encrypt_metadata);
            let user_entry = legacy_user_entry(&file_key, document_id, r);
            let mut tail = vec![PdfDictEntry::new("V", PdfObject::Int(v)), PdfDictEntry::new("R", PdfObject::Int(r as i64)), PdfDictEntry::new("Length", PdfObject::Int(key_length as i64 * 8)), PdfDictEntry::new("P", PdfObject::Int(permissions as i64)), PdfDictEntry::new("O", PdfObject::Str(owner_entry)), PdfDictEntry::new("U", PdfObject::Str(user_entry))];
            if aes {
                tail.push(PdfDictEntry::new("CF", PdfObject::Dict(vec![PdfDictEntry::new("StdCF", PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("CryptFilter")), PdfDictEntry::new("CFM", PdfObject::name("AESV2")), PdfDictEntry::new("AuthEvent", PdfObject::name("DocOpen")), PdfDictEntry::new("Length", PdfObject::Int(16))]))])));
                tail.push(PdfDictEntry::new("StmF", PdfObject::name("StdCF")));
                tail.push(PdfDictEntry::new("StrF", PdfObject::name("StdCF")));
                if !parameters.encrypt_metadata {
                    tail.push(PdfDictEntry::new("EncryptMetadata", PdfObject::Bool(false)));
                }
            }
            (Decryptor { file_key, aes, identity_strings: false, identity_streams: false, revision: r }, tail)
        }
        PdfEncryptionAlgorithm::Aes256 => {
            let file_key = sha256(&[b"semio.pdf.file-key".as_slice(), seed, document_id].concat());
            let salts = sha512(&[b"semio.pdf.salts".as_slice(), seed, document_id].concat());
            let truncated_user: Vec<u8> = user_password.iter().copied().take(127).collect();
            let truncated_owner: Vec<u8> = owner_password.iter().copied().take(127).collect();
            let user_validation_salt = &salts[0..8];
            let user_key_salt = &salts[8..16];
            let owner_validation_salt = &salts[16..24];
            let owner_key_salt = &salts[24..32];
            let mut user_entry = revision6_hash(&truncated_user, user_validation_salt, &[]);
            user_entry.extend_from_slice(user_validation_salt);
            user_entry.extend_from_slice(user_key_salt);
            let user_intermediate = revision6_hash(&truncated_user, user_key_salt, &[]);
            let ue = aes_cbc_no_pad(&user_intermediate, [0u8; 16], &file_key, true);
            let mut owner_entry = revision6_hash(&truncated_owner, owner_validation_salt, &user_entry);
            owner_entry.extend_from_slice(owner_validation_salt);
            owner_entry.extend_from_slice(owner_key_salt);
            let owner_intermediate = revision6_hash(&truncated_owner, owner_key_salt, &user_entry);
            let oe = aes_cbc_no_pad(&owner_intermediate, [0u8; 16], &file_key, true);
            let mut perms_block = Vec::with_capacity(16);
            perms_block.extend_from_slice(&(permissions as u32).to_le_bytes());
            perms_block.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
            perms_block.push(if parameters.encrypt_metadata { b'T' } else { b'F' });
            perms_block.extend_from_slice(b"adb");
            perms_block.extend_from_slice(&salts[32..36]);
            let perms = AesKey::new(&file_key).encrypt_block(&perms_block.try_into().expect("16 bytes")).to_vec();
            let mut tail = vec![
                PdfDictEntry::new("V", PdfObject::Int(5)),
                PdfDictEntry::new("R", PdfObject::Int(6)),
                PdfDictEntry::new("Length", PdfObject::Int(256)),
                PdfDictEntry::new("P", PdfObject::Int(permissions as i64)),
                PdfDictEntry::new("O", PdfObject::Str(owner_entry)),
                PdfDictEntry::new("U", PdfObject::Str(user_entry)),
                PdfDictEntry::new("OE", PdfObject::Str(oe)),
                PdfDictEntry::new("UE", PdfObject::Str(ue)),
                PdfDictEntry::new("Perms", PdfObject::Str(perms)),
                PdfDictEntry::new("CF", PdfObject::Dict(vec![PdfDictEntry::new("StdCF", PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("CryptFilter")), PdfDictEntry::new("CFM", PdfObject::name("AESV3")), PdfDictEntry::new("AuthEvent", PdfObject::name("DocOpen")), PdfDictEntry::new("Length", PdfObject::Int(32))]))])),
                PdfDictEntry::new("StmF", PdfObject::name("StdCF")),
                PdfDictEntry::new("StrF", PdfObject::name("StdCF")),
            ];
            if !parameters.encrypt_metadata {
                tail.push(PdfDictEntry::new("EncryptMetadata", PdfObject::Bool(false)));
            }
            (Decryptor { file_key, aes: true, identity_strings: false, identity_streams: false, revision: 6 }, tail)
        }
    };
    entries.extend(tail);
    (decryptor, entries)
}
//#endregion 🔖️StandardSecurityHandler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
