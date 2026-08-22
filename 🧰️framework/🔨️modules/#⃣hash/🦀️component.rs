//! 🪪️ Owned SHA-256 plus Blake3 content-hash and Merkle utilities for operation envelopes and assets.

//#region 🔐️Sha256
const SHA256_INITIAL: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
const SHA256_ROUNDS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// 🔐️ Incremental repository-owned SHA-256 state with no runtime dependency.
#[derive(Clone)]
pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// 🆕️ Creates an empty SHA-256 stream.
    pub const fn new() -> Self {
        Self { state: SHA256_INITIAL, buffer: [0; 64], buffer_len: 0, total_len: 0 }
    }

    /// 📥️ Absorbs the next byte segment without retaining caller memory.
    pub fn update(&mut self, mut bytes: &[u8]) {
        self.total_len = self.total_len.checked_add(bytes.len() as u64).expect("SHA-256 input length overflow");
        if self.buffer_len != 0 {
            let count = (64 - self.buffer_len).min(bytes.len());
            self.buffer[self.buffer_len..self.buffer_len + count].copy_from_slice(&bytes[..count]);
            self.buffer_len += count;
            bytes = &bytes[count..];
            if self.buffer_len == 64 {
                let block = self.buffer;
                self.transform(&block);
                self.buffer_len = 0;
            } else {
                return;
            }
        }
        while bytes.len() >= 64 {
            let block: &[u8; 64] = bytes[..64].try_into().expect("SHA-256 block width");
            self.transform(block);
            bytes = &bytes[64..];
        }
        self.buffer[..bytes.len()].copy_from_slice(bytes);
        self.buffer_len = bytes.len();
    }

    /// ✅️ Finalizes the stream into the canonical 32-byte digest.
    pub fn finalize(mut self) -> [u8; 32] {
        let bit_len = self.total_len.checked_mul(8).expect("SHA-256 bit length overflow");
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;
        if self.buffer_len > 56 {
            self.buffer[self.buffer_len..].fill(0);
            let block = self.buffer;
            self.transform(&block);
            self.buffer = [0; 64];
        } else {
            self.buffer[self.buffer_len..56].fill(0);
        }
        self.buffer[56..].copy_from_slice(&bit_len.to_be_bytes());
        let block = self.buffer;
        self.transform(&block);
        let mut digest = [0; 32];
        for (chunk, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(self.state) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        digest
    }

    /// 🧮️ Hashes one complete byte slice.
    pub fn digest(bytes: &[u8]) -> [u8; 32] {
        let mut hash = Self::new();
        hash.update(bytes);
        hash.finalize()
    }

    fn transform(&mut self, block: &[u8; 64]) {
        let mut words = [0u32; 64];
        for (word, bytes) in words.iter_mut().take(16).zip(block.as_chunks::<4>().0) {
            *word = u32::from_be_bytes(*bytes);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7) ^ words[index - 15].rotate_right(18) ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17) ^ words[index - 2].rotate_right(19) ^ (words[index - 2] >> 10);
            words[index] = words[index - 16].wrapping_add(s0).wrapping_add(words[index - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ (!e & g);
            let temporary1 = h.wrapping_add(sum1).wrapping_add(choice).wrapping_add(SHA256_ROUNDS[index]).wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temporary2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temporary1);
            d = c;
            c = b;
            b = a;
            a = temporary1.wrapping_add(temporary2);
        }
        for (state, value) in self.state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *state = state.wrapping_add(value);
        }
    }
}

/// #️⃣ Computes canonical lowercase SHA-256 hex for a complete byte slice.
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex_lower(&Sha256::digest(bytes))
}

/// 🔡 Encodes bytes as lowercase hexadecimal without a formatting dependency.
pub fn hex_lower(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}
//#endregion 🔐️Sha256

//#region 🔖️Hash
pub fn hash_parts<S: AsRef<[u8]>>(parts: &[S]) -> String {
    let mut hasher = blake3::Hasher::new();
    for part in parts {
        hasher.update(part.as_ref());
        hasher.update(b"\x1f");
    }
    hasher.finalize().to_hex().to_string()
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

pub fn format_number_for_hash(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_positive() { "inf".into() } else { "-inf".into() };
    }
    if value == 0.0 {
        return "0".into();
    }
    if (value - value.round()).abs() < 1e-9 && value.abs() < 1e15 {
        return format!("{:.0}", value);
    }
    let mut text = format!("{value:.12}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        "0".into()
    } else {
        text
    }
}

pub fn merkle_node(own: &[&str], mut children: Vec<String>) -> String {
    children.sort();
    let mut hasher = blake3::Hasher::new();
    for entry in own {
        hasher.update(entry.as_bytes());
        hasher.update(b"\x1f");
    }
    for child in &children {
        hasher.update(child.as_bytes());
        hasher.update(b"\x1f");
    }
    hasher.finalize().to_hex().to_string()
}

pub fn merkle_collection(children: Vec<String>) -> String {
    merkle_node(&["RelayCollection"], children)
}
//#endregion 🔖️Hash

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_bytes_deterministically() {
        let first = hash_bytes(b"hello");
        let second = hash_bytes(b"hello");
        assert_eq!(first, second);
        assert_ne!(first, hash_bytes(b"world"));
    }

    #[test]
    fn sha256_matches_nist_vectors_and_segmented_input() {
        assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        let mut segmented = Sha256::new();
        segmented.update(b"a");
        segmented.update(b"b");
        segmented.update(b"c");
        assert_eq!(hex_lower(&segmented.finalize()), sha256_hex(b"abc"));
        let long = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        assert_eq!(sha256_hex(long), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
        let mut segmented_long = Sha256::new();
        for chunk in long.chunks(7) {
            segmented_long.update(chunk);
        }
        assert_eq!(hex_lower(&segmented_long.finalize()), sha256_hex(long));
    }

    #[test]
    fn normalizes_hash_numbers() {
        assert_eq!(format_number_for_hash(-0.0), "0");
        assert_eq!(format_number_for_hash(42.0), "42");
        assert_eq!(format_number_for_hash(1.25), "1.25");
    }

    #[test]
    fn separates_hash_parts_with_a_delimiter() {
        assert_ne!(hash_parts(&["ab", "c"]), hash_parts(&["a", "bc"]));
    }

    #[test]
    fn orders_merkle_children_deterministically() {
        assert_eq!(merkle_node(&["root"], vec!["child-b".into(), "child-a".into()]), merkle_node(&["root"], vec!["child-a".into(), "child-b".into()]),);
    }

    #[test]
    fn normalizes_special_hash_numbers() {
        assert_eq!(format_number_for_hash(f64::NAN), "nan");
        assert_eq!(format_number_for_hash(f64::INFINITY), "inf");
        assert_eq!(format_number_for_hash(f64::NEG_INFINITY), "-inf");
    }
}
