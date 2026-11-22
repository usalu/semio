//! 🕳️ Encodes canonical absent sample ordinals for rectangular terrain grids.

/// 📦️ Largest encoded mask at the maximum supported grid size.
pub const MAX_MASK_BYTES: usize = (65_usize * 65).div_ceil(8);
pub const MAX_MASK_HEX_BYTES: usize = MAX_MASK_BYTES * 2;

const HEX: &[u8; 16] = b"0123456789abcdef";

fn nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

fn byte_at(hex: &str, index: usize) -> Option<u8> {
    let pair = hex.as_bytes().get(2 * index..2 * index + 2)?;
    Some((nibble(pair[0])? << 4) | nibble(pair[1])?)
}

/// 🧩 Stores a bounded south-first bitset without an allocation during per-row import.
pub struct MissingSamples {
    bytes: [u8; MAX_MASK_BYTES],
    sample_count: usize,
}

impl MissingSamples {
    /// ➕️ Creates an entirely present bounded grid.
    pub fn new(sample_count: usize) -> Option<Self> {
        (sample_count <= 65 * 65).then_some(Self { bytes: [0; MAX_MASK_BYTES], sample_count })
    }

    /// 🪫 Marks one south-first sample ordinal as missing.
    pub fn mark_missing(&mut self, ordinal: usize) {
        assert!(ordinal < self.sample_count);
        self.bytes[ordinal / 8] |= 1 << (ordinal % 8);
    }

    fn bytes(&self) -> &[u8] { &self.bytes[..self.sample_count.div_ceil(8)] }

    /// 🔢 Reports the exact length of the canonical wire value, zero when all samples are present.
    pub fn encoded_len(&self) -> usize {
        if self.bytes().iter().any(|&byte| byte != 0) { 2 * self.bytes().len() } else { 0 }
    }

    /// 🟰 Compares to a canonical mask without constructing a second retained value.
    pub fn same_encoding(&self, value: Option<&str>) -> bool {
        match value {
            None => self.encoded_len() == 0,
            Some(hex) => self.encoded_len() == hex.len()
                && self.bytes().iter().enumerate().all(|(index, &value)| byte_at(hex, index) == Some(value)),
        }
    }

    /// 💾️ Returns the optional canonical lowercase hexadecimal mask.
    pub fn encode(&self) -> Option<String> {
        let length = self.encoded_len();
        if length == 0 { return None; }
        let mut result = String::with_capacity(length);
        for &byte in self.bytes() {
            result.push(char::from(HEX[(byte >> 4) as usize]));
            result.push(char::from(HEX[(byte & 15) as usize]));
        }
        Some(result)
    }
}

/// 🎯️ Queries the validity of an ordinal in a checked encoding.
pub fn is_missing(hex: Option<&str>, ordinal: usize) -> bool {
    hex.and_then(|hex| byte_at(hex, ordinal / 8)).is_some_and(|byte| byte & (1 << (ordinal % 8)) != 0)
}

/// ✅️ Checks mask spelling, extent, padding and canonical zero payloads.
pub fn is_valid(hex: Option<&str>, count: usize, mut samples: impl Iterator<Item = i32>) -> bool {
    let Some(hex) = hex else { return true; };
    let bytes = count.div_ceil(8);
    if bytes == 0 || bytes > MAX_MASK_BYTES || hex.len() != 2 * bytes { return false; }
    let mut any_missing = false;
    let mut current = 0;
    for ordinal in 0..count {
        let Some(sample) = samples.next() else { return false; };
        if ordinal % 8 == 0 {
            let Some(byte) = byte_at(hex, ordinal / 8) else { return false; };
            current = byte;
            any_missing |= byte != 0;
        }
        if current & (1 << (ordinal % 8)) != 0 && sample != 0 { return false; }
    }
    samples.next().is_none() && any_missing && (count % 8 == 0 || current >> (count % 8) == 0)
}
