//! 🔤️ Strict RFC 4648 §4 standard-alphabet, padded base64 codec — the sole runtime encoding any
//! s-plugin needs, replacing the third-party `base64` crate. See
//! <https://www.rfc-editor.org/rfc/rfc4648#section-4>. Relocated verbatim (algorithm unchanged)
//! from `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`'s `🔤️Base64` region — a
//! product-neutral byte codec has no business living inside the replication wire contract, and
//! seven unrelated s-plugins needed it without pulling in replication's mutation/causal/conflict
//! vocabulary. `📡️replication` now re-exports `base64_standard_encode`/`base64_standard_decode`/
//! `Base64Error` from here so its own `crate::base64_standard_*` callers are untouched.

const BASE64_STANDARD_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔤️ Strict RFC 4648 standard-base64 decoding failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Base64Error {
    InvalidLength,
    InvalidByte { index: usize, byte: u8 },
    InvalidPadding,
    NonCanonicalTrailingBits,
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("base64 length must be a multiple of four"),
            Self::InvalidByte { index, byte } => write!(formatter, "invalid base64 byte {byte:#04x} at index {index}"),
            Self::InvalidPadding => formatter.write_str("invalid base64 padding"),
            Self::NonCanonicalTrailingBits => formatter.write_str("non-canonical base64 trailing bits"),
        }
    }
}

impl std::error::Error for Base64Error {}

fn encode_bytes(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let first = chunk[0];
        let second = chunk.get(1).copied().unwrap_or(0);
        let third = chunk.get(2).copied().unwrap_or(0);
        encoded.push(BASE64_STANDARD_ALPHABET[(first >> 2) as usize] as char);
        encoded.push(BASE64_STANDARD_ALPHABET[(((first & 0x03) << 4) | (second >> 4)) as usize] as char);
        if chunk.len() >= 2 {
            encoded.push(BASE64_STANDARD_ALPHABET[(((second & 0x0f) << 2) | (third >> 6)) as usize] as char);
        } else {
            encoded.push('=');
        }
        if chunk.len() == 3 {
            encoded.push(BASE64_STANDARD_ALPHABET[(third & 0x3f) as usize] as char);
        } else {
            encoded.push('=');
        }
    }
    encoded
}

/// 🔤️ Encodes bytes with the padded RFC 4648 standard alphabet. Generic over `AsRef<[u8]>` (so a
/// bare `&str` works too) purely as an ergonomic front door onto [`encode_bytes`] — the one
/// algorithm this module implements.
pub fn base64_standard_encode(bytes: impl AsRef<[u8]>) -> String {
    encode_bytes(bytes.as_ref())
}

fn sextet(byte: u8, index: usize) -> Result<u8, Base64Error> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(Base64Error::InvalidByte { index, byte }),
    }
}

fn decode_bytes(encoded: &[u8]) -> Result<Vec<u8>, Base64Error> {
    if !encoded.len().is_multiple_of(4) {
        return Err(Base64Error::InvalidLength);
    }
    let mut decoded = Vec::with_capacity(encoded.len() / 4 * 3);
    for (group_index, chunk) in encoded.as_chunks::<4>().0.iter().enumerate() {
        let offset = group_index * 4;
        let last = offset + 4 == encoded.len();
        if chunk[0] == b'=' || chunk[1] == b'=' {
            return Err(Base64Error::InvalidPadding);
        }
        let first = sextet(chunk[0], offset)?;
        let second = sextet(chunk[1], offset + 1)?;
        decoded.push((first << 2) | (second >> 4));
        if chunk[2] == b'=' {
            if !last || chunk[3] != b'=' {
                return Err(Base64Error::InvalidPadding);
            }
            if second & 0x0f != 0 {
                return Err(Base64Error::NonCanonicalTrailingBits);
            }
            continue;
        }
        let third = sextet(chunk[2], offset + 2)?;
        decoded.push((second << 4) | (third >> 2));
        if chunk[3] == b'=' {
            if !last {
                return Err(Base64Error::InvalidPadding);
            }
            if third & 0x03 != 0 {
                return Err(Base64Error::NonCanonicalTrailingBits);
            }
            continue;
        }
        let fourth = sextet(chunk[3], offset + 3)?;
        decoded.push((third << 6) | fourth);
    }
    Ok(decoded)
}

/// 🔤️ Decodes padded RFC 4648 standard base64 and rejects whitespace, misplaced padding, and
/// non-canonical unused bits. Generic over `AsRef<[u8]>` for the same reason as
/// [`base64_standard_encode`] — one algorithm, [`decode_bytes`], behind an ergonomic front door.
pub fn base64_standard_decode(encoded: impl AsRef<[u8]>) -> Result<Vec<u8>, Base64Error> {
    decode_bytes(encoded.as_ref())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
