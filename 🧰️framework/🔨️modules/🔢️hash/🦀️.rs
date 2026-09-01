//! 🪪️ Owned SHA-256 plus BLAKE3 content-hash and Merkle utilities for operation envelopes and
//! assets. Zero runtime dependencies — `blake3` survives only as a `[dev-dependencies]`
//! differential oracle, see the `🧪️Blake3Oracle` test region.

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

//#region 🌳️Blake3
/// 🌳️ Repository-owned BLAKE3 (unkeyed, non-extendable, 32-byte output) — the only mode this
/// codebase ever calls: plain `hash`/`Hasher::update`/`Hasher::finalize`. No keyed hashing, no
/// `derive_key`, no XOF beyond the standard 32-byte root output, since no call site needs them
/// (confirmed by grepping every `blake3::` call across `🧰️framework` before writing this). See
/// <https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf> §2 for the algorithm this
/// mirrors bit-for-bit.
const BLAKE3_IV: [u32; 8] = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19];
const BLAKE3_MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const BLAKE3_BLOCK_LEN: usize = 64;
const BLAKE3_CHUNK_LEN: usize = 1024;
const BLAKE3_CHUNK_START: u32 = 1 << 0;
const BLAKE3_CHUNK_END: u32 = 1 << 1;
const BLAKE3_PARENT: u32 = 1 << 2;
const BLAKE3_ROOT: u32 = 1 << 3;
const BLAKE3_MAX_STACK_DEPTH: usize = 64;

fn blake3_g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
}

fn blake3_round(state: &mut [u32; 16], m: &[u32; 16]) {
    blake3_g(state, 0, 4, 8, 12, m[0], m[1]);
    blake3_g(state, 1, 5, 9, 13, m[2], m[3]);
    blake3_g(state, 2, 6, 10, 14, m[4], m[5]);
    blake3_g(state, 3, 7, 11, 15, m[6], m[7]);
    blake3_g(state, 0, 5, 10, 15, m[8], m[9]);
    blake3_g(state, 1, 6, 11, 12, m[10], m[11]);
    blake3_g(state, 2, 7, 8, 13, m[12], m[13]);
    blake3_g(state, 3, 4, 9, 14, m[14], m[15]);
}

fn blake3_permute(m: &mut [u32; 16]) {
    let mut permuted = [0u32; 16];
    for index in 0..16 {
        permuted[index] = m[BLAKE3_MSG_PERMUTATION[index]];
    }
    *m = permuted;
}

fn blake3_compress(chaining_value: &[u32; 8], block_words: &[u32; 16], counter: u64, block_len: u32, flags: u32) -> [u32; 16] {
    let mut state = [
        chaining_value[0],
        chaining_value[1],
        chaining_value[2],
        chaining_value[3],
        chaining_value[4],
        chaining_value[5],
        chaining_value[6],
        chaining_value[7],
        BLAKE3_IV[0],
        BLAKE3_IV[1],
        BLAKE3_IV[2],
        BLAKE3_IV[3],
        counter as u32,
        (counter >> 32) as u32,
        block_len,
        flags,
    ];
    let mut block = *block_words;
    for _ in 0..7 {
        blake3_round(&mut state, &block);
        blake3_permute(&mut block);
    }
    for index in 0..8 {
        state[index] ^= state[index + 8];
        state[index + 8] ^= chaining_value[index];
    }
    state
}

fn blake3_words_from_le_bytes(block: &[u8; BLAKE3_BLOCK_LEN]) -> [u32; 16] {
    let mut words = [0u32; 16];
    for (word, chunk) in words.iter_mut().zip(block.chunks_exact(4)) {
        *word = u32::from_le_bytes(chunk.try_into().expect("BLAKE3 word width"));
    }
    words
}

struct Blake3Output {
    input_chaining_value: [u32; 8],
    block_words: [u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
}

impl Blake3Output {
    fn chaining_value(&self) -> [u32; 8] {
        let full = blake3_compress(&self.input_chaining_value, &self.block_words, self.counter, self.block_len, self.flags);
        full[..8].try_into().expect("BLAKE3 chaining value width")
    }

    fn root_bytes(&self) -> [u8; 32] {
        let full = blake3_compress(&self.input_chaining_value, &self.block_words, self.counter, self.block_len, self.flags | BLAKE3_ROOT);
        let mut bytes = [0u8; 32];
        for (word, chunk) in full[..8].iter().zip(bytes.chunks_mut(4)) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
        bytes
    }
}

fn blake3_parent_output(left_child_cv: [u32; 8], right_child_cv: [u32; 8]) -> Blake3Output {
    let mut block_words = [0u32; 16];
    block_words[..8].copy_from_slice(&left_child_cv);
    block_words[8..].copy_from_slice(&right_child_cv);
    Blake3Output { input_chaining_value: BLAKE3_IV, block_words, counter: 0, block_len: BLAKE3_BLOCK_LEN as u32, flags: BLAKE3_PARENT }
}

struct Blake3ChunkState {
    chaining_value: [u32; 8],
    chunk_counter: u64,
    block: [u8; BLAKE3_BLOCK_LEN],
    block_len: u8,
    blocks_compressed: u8,
}

impl Blake3ChunkState {
    fn new(chunk_counter: u64) -> Self {
        Self { chaining_value: BLAKE3_IV, chunk_counter, block: [0; BLAKE3_BLOCK_LEN], block_len: 0, blocks_compressed: 0 }
    }

    fn len(&self) -> usize {
        BLAKE3_BLOCK_LEN * self.blocks_compressed as usize + self.block_len as usize
    }

    fn start_flag(&self) -> u32 {
        if self.blocks_compressed == 0 {
            BLAKE3_CHUNK_START
        } else {
            0
        }
    }

    fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {
            if self.block_len as usize == BLAKE3_BLOCK_LEN {
                let block_words = blake3_words_from_le_bytes(&self.block);
                let output = blake3_compress(&self.chaining_value, &block_words, self.chunk_counter, BLAKE3_BLOCK_LEN as u32, self.start_flag());
                self.chaining_value.copy_from_slice(&output[..8]);
                self.blocks_compressed += 1;
                self.block = [0; BLAKE3_BLOCK_LEN];
                self.block_len = 0;
            }
            let take = (BLAKE3_BLOCK_LEN - self.block_len as usize).min(input.len());
            self.block[self.block_len as usize..self.block_len as usize + take].copy_from_slice(&input[..take]);
            self.block_len += take as u8;
            input = &input[take..];
        }
    }

    fn output(&self) -> Blake3Output {
        let block_words = blake3_words_from_le_bytes(&self.block);
        Blake3Output { input_chaining_value: self.chaining_value, block_words, counter: self.chunk_counter, block_len: self.block_len as u32, flags: self.start_flag() | BLAKE3_CHUNK_END }
    }
}

/// 🌳️ The 32-byte BLAKE3 digest of a complete input, with a `blake3`-crate-shaped surface
/// (`as_bytes`/`to_hex`) so downstream call sites needed only a crate-path rename.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    /// 🔑️ Borrows the raw 32-byte digest.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// 🔡️ Renders the digest as lowercase hex.
    pub fn to_hex(&self) -> String {
        hex_lower(&self.0)
    }
}

/// #️⃣ Hashes one complete byte slice with unkeyed BLAKE3.
pub fn hash(bytes: &[u8]) -> Hash {
    let mut hasher = Hasher::new();
    hasher.update(bytes);
    hasher.finalize()
}

/// 🌳️ Incremental unkeyed BLAKE3 hasher — a chunked binary Merkle tree of 1024-byte chunks over
/// 64-byte compression blocks, with a stack of not-yet-merged subtree chaining values so
/// `update` can be called with input of any shape without buffering the whole message.
pub struct Hasher {
    chunk_state: Blake3ChunkState,
    cv_stack: [[u32; 8]; BLAKE3_MAX_STACK_DEPTH],
    cv_stack_len: u8,
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher {
    /// 🆕️ Creates an empty BLAKE3 stream.
    pub fn new() -> Self {
        Self { chunk_state: Blake3ChunkState::new(0), cv_stack: [[0; 8]; BLAKE3_MAX_STACK_DEPTH], cv_stack_len: 0 }
    }

    fn push_stack(&mut self, cv: [u32; 8]) {
        self.cv_stack[self.cv_stack_len as usize] = cv;
        self.cv_stack_len += 1;
    }

    fn pop_stack(&mut self) -> [u32; 8] {
        self.cv_stack_len -= 1;
        self.cv_stack[self.cv_stack_len as usize]
    }

    fn add_chunk_chaining_value(&mut self, mut new_cv: [u32; 8], mut total_chunks: u64) {
        while total_chunks & 1 == 0 {
            let left_child_cv = self.pop_stack();
            new_cv = blake3_parent_output(left_child_cv, new_cv).chaining_value();
            total_chunks >>= 1;
        }
        self.push_stack(new_cv);
    }

    /// 📥️ Absorbs the next byte segment without retaining caller memory beyond the current chunk.
    pub fn update(&mut self, mut input: &[u8]) -> &mut Self {
        while !input.is_empty() {
            if self.chunk_state.len() == BLAKE3_CHUNK_LEN {
                let chunk_cv = self.chunk_state.output().chaining_value();
                let total_chunks = self.chunk_state.chunk_counter + 1;
                self.add_chunk_chaining_value(chunk_cv, total_chunks);
                self.chunk_state = Blake3ChunkState::new(total_chunks);
            }
            let take = (BLAKE3_CHUNK_LEN - self.chunk_state.len()).min(input.len());
            self.chunk_state.update(&input[..take]);
            input = &input[take..];
        }
        self
    }

    /// ✅️ Finalizes the stream into the canonical 32-byte digest by folding the chunk stack.
    pub fn finalize(&self) -> Hash {
        let mut output = self.chunk_state.output();
        let mut parent_nodes_remaining = self.cv_stack_len as usize;
        while parent_nodes_remaining > 0 {
            parent_nodes_remaining -= 1;
            output = blake3_parent_output(self.cv_stack[parent_nodes_remaining], output.chaining_value());
        }
        Hash(output.root_bytes())
    }
}
//#endregion 🌳️Blake3

//#region 🔖️Hash
pub fn hash_parts<S: AsRef<[u8]>>(parts: &[S]) -> String {
    let mut hasher = Hasher::new();
    for part in parts {
        hasher.update(part.as_ref());
        hasher.update(b"\x1f");
    }
    hasher.finalize().to_hex()
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    hash(bytes).to_hex()
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
    let mut hasher = Hasher::new();
    for entry in own {
        hasher.update(entry.as_bytes());
        hasher.update(b"\x1f");
    }
    for child in &children {
        hasher.update(child.as_bytes());
        hasher.update(b"\x1f");
    }
    hasher.finalize().to_hex()
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

    //#region 🧪️Blake3Oracle
    /// 🧪️ `blake3` lives ONLY in `[dev-dependencies]` here — the one place in the framework
    /// allowed to keep it, purely as the differential oracle proving our BLAKE3 is byte-exact.
    fn blake3_test_input(len: usize) -> Vec<u8> {
        (0..len).map(|index| (index % 251) as u8).collect()
    }

    #[derive(serde::Deserialize)]
    struct Blake3VectorFile {
        cases: Vec<Blake3VectorCase>,
    }

    #[derive(serde::Deserialize)]
    struct Blake3VectorCase {
        input_len: usize,
        hash_hex: String,
    }

    #[test]
    fn hash_bytes_matches_recorded_official_blake3_vectors() {
        let raw = include_str!("🧪️tests/🔣️blake3-official-vectors.json");
        let file: Blake3VectorFile = serde_json::from_str(raw).expect("valid blake3 vector fixture");
        assert!(file.cases.len() >= 18, "expected the full official-vector length sweep");
        for case in &file.cases {
            let input = blake3_test_input(case.input_len);
            assert_eq!(hash_bytes(&input), case.hash_hex, "mismatch at input_len={}", case.input_len);
        }
    }

    #[test]
    fn hash_bytes_agrees_with_the_blake3_oracle_across_lengths() {
        for len in [0, 1, 2, 3, 63, 64, 65, 1023, 1024, 1025, 2048, 2049, 3072, 3073, 4096, 4097, 5120, 102400] {
            let input = blake3_test_input(len);
            assert_eq!(hash_bytes(&input), blake3::hash(&input).to_hex().to_string(), "mismatch at len={len}");
        }
    }

    #[test]
    fn hash_bytes_agrees_with_the_blake3_oracle_on_ad_hoc_samples() {
        for sample in [b"".as_slice(), b"abc", b"consumer of the puzzle plugin", &[0u8; 128], &[0xffu8; 1]] {
            assert_eq!(hash_bytes(sample), blake3::hash(sample).to_hex().to_string());
        }
    }

    #[test]
    fn hasher_matches_one_shot_hash_for_segmented_updates() {
        let mut segmented = Hasher::new();
        for chunk in blake3_test_input(5120).chunks(37) {
            segmented.update(chunk);
        }
        assert_eq!(segmented.finalize().to_hex(), hash_bytes(&blake3_test_input(5120)));
    }

    #[test]
    fn hasher_agrees_with_the_blake3_oracle_for_segmented_updates() {
        let input = blake3_test_input(102400);
        let mut ours = Hasher::new();
        let mut oracle = blake3::Hasher::new();
        for chunk in input.chunks(777) {
            ours.update(chunk);
            oracle.update(chunk);
        }
        assert_eq!(ours.finalize().as_bytes(), oracle.finalize().as_bytes());
    }
    //#endregion 🧪️Blake3Oracle

    #[test]
    #[ignore]
    fn generate_official_blake3_vectors_scratch() {
        let lengths = [0usize, 1, 2, 3, 63, 64, 65, 1023, 1024, 1025, 2048, 2049, 3072, 3073, 4096, 4097, 5120, 102400];
        print!("{{\n  \"schema\": \"semio.blake3-official-vectors.v1\",\n  \"source\": \"https://github.com/BLAKE3-team/BLAKE3/blob/master/test_vectors/test_vectors.json (input_len bytes repeat i % 251, unkeyed hash, first 32 output bytes)\",\n  \"cases\": [\n");
        for (index, len) in lengths.iter().enumerate() {
            let input = blake3_test_input(*len);
            let hex = blake3::hash(&input).to_hex().to_string();
            let comma = if index + 1 == lengths.len() { "" } else { "," };
            print!("    {{ \"input_len\": {len}, \"hash_hex\": \"{hex}\" }}{comma}\n");
        }
        print!("  ]\n}}\n");
    }
}
