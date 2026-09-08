//! 🧾 Pack varint, byte I/O, CRC, and compression codec primitives.

use crate::codec::ids::{ByteRange, ChunkId, CodecId, ContentHash};
use crate::diagnostic::FaultOrigin;

//#region 🔖️Errors
/// @emoji 🚨️ The one error type every `pack_*` public fn returns; never leaks `std::io::Error`.
#[derive(Debug, Clone, PartialEq)]
pub enum PackError {
    BadMagic,
    UnsupportedVersion { major: u16, minor: u16 },
    UnknownRequiredFlags(u32),
    Truncated(u64),
    ChecksumMismatch { segment: &'static str, offset: u64 },
    ContentHashMismatch,
    LimitExceeded(&'static str),
    Malformed { what: &'static str, offset: u64, detail: String },
    NonCanonical(&'static str),
    UnsupportedCodec(u8),
    Schema(String),
    Io(String),
}

impl std::fmt::Display for PackError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadMagic => formatter.write_str("bad magic"),
            Self::UnsupportedVersion { major, minor } => write!(formatter, "unsupported version {major}.{minor}"),
            Self::UnknownRequiredFlags(flags) => write!(formatter, "unknown required feature bits {flags:#x}"),
            Self::Truncated(offset) => write!(formatter, "truncated at offset {offset}"),
            Self::ChecksumMismatch { segment, offset } => write!(formatter, "checksum mismatch in {segment} at offset {offset}"),
            Self::ContentHashMismatch => formatter.write_str("content hash mismatch"),
            Self::LimitExceeded(limit) => write!(formatter, "limit exceeded: {limit}"),
            Self::Malformed { what, offset, detail } => write!(formatter, "malformed {what} at offset {offset}: {detail}"),
            Self::NonCanonical(detail) => write!(formatter, "non-canonical encoding: {detail}"),
            Self::UnsupportedCodec(codec) => write!(formatter, "unsupported codec {codec}"),
            Self::Schema(message) => write!(formatter, "schema error: {message}"),
            Self::Io(message) => write!(formatter, "io error: {message}"),
        }
    }
}

impl std::error::Error for PackError {}

crate::fault_from_error!(PackError, FaultOrigin::Module, "module.pack");

//#endregion 🔖️Errors

//#region 🔤️Base64
/// 🔤️ Relocated to `semio-framework-io-base64` (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS
/// wave 1) — a product-neutral byte codec has no business living inside the replication wire
/// contract, and seven unrelated s-plugins needed it without pulling in replication's
/// mutation/causal/conflict vocabulary. Re-exported here so every existing `crate::base64_standard_*`
/// caller in this crate keeps resolving unchanged.
pub use semio_framework_io_base64::{base64_standard_decode, base64_standard_encode, Base64Error};
//#endregion 🔤️Base64

//#region 🔖️Limits
/// @emoji 🛡️ Corruption-hardening ceilings every decoder must validate against before
/// allocating — load-bearing for every other `pack_*` crate's fuzz/corruption tests.
#[derive(Clone, Debug)]
pub struct PackLimits {
    pub max_file_len: u64,
    pub max_segment_len: u64,
    pub max_symbols: u32,
    pub max_depth: u16,
    pub max_items: u64,
    pub max_total_alloc: u64,
}

impl Default for PackLimits {
    fn default() -> Self {
        Self { max_file_len: 16 * 1024 * 1024 * 1024, max_segment_len: 256 * 1024 * 1024, max_symbols: 1_000_000, max_depth: 64, max_items: 64_000_000, max_total_alloc: 4 * 1024 * 1024 * 1024 }
    }
}
//#endregion 🔖️Limits

//#region 🔖️Varint
/// @emoji ➡️ Zigzag-encodes an `i64` into the `u64` domain: small magnitudes stay small
/// regardless of sign. See <https://protobuf.dev/programming-guides/encoding/#signed-ints>.
fn zigzag_encode(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

/// @emoji ⬅️ Inverse of `zigzag_encode`.
fn zigzag_decode(value: u64) -> i64 {
    ((value >> 1) as i64) ^ -((value & 1) as i64)
}

/// @emoji ✏️ Writes `value` as an unsigned LEB128 varint (minimal length, max 10 bytes).
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    let mut remaining = value;
    loop {
        let byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if remaining == 0 {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}

/// @emoji 📖️ Reads an unsigned LEB128 varint starting at `*pos`, advancing `*pos` past it.
/// Errors `Malformed` on a >10-byte (overlong) encoding, `Truncated` on running out of bytes.
pub fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, PackError> {
    let start = *pos;
    let mut result: u64 = 0;
    for i in 0..10usize {
        let idx = *pos;
        if idx >= bytes.len() {
            return Err(PackError::Truncated(idx as u64));
        }
        let byte = bytes[idx];
        *pos += 1;
        let more = byte & 0x80 != 0;
        let payload = (byte & 0x7F) as u64;
        if i == 9 && (more || payload > 1) {
            return Err(PackError::Malformed { what: "varint", offset: start as u64, detail: "overlong varint (exceeds 10 bytes / 64 bits)".to_string() });
        }
        result |= payload << (i as u32 * 7);
        if !more {
            return Ok(result);
        }
    }
    Err(PackError::Malformed { what: "varint", offset: start as u64, detail: "overlong varint (exceeds 10 bytes)".to_string() })
}

/// @emoji ✏️ Writes `value` as a zigzag-encoded signed varint.
pub fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    write_varint_u64(out, zigzag_encode(value));
}

/// @emoji 📖️ Reads a zigzag-encoded signed varint starting at `*pos`.
pub fn read_varint_i64(bytes: &[u8], pos: &mut usize) -> Result<i64, PackError> {
    let raw = read_varint_u64(bytes, pos)?;
    Ok(zigzag_decode(raw))
}

/// @emoji ✅️ True iff `bytes` is exactly one minimal-length varint (decoding then re-encoding
/// reproduces the input byte-for-byte, with nothing left over).
pub fn is_minimal_varint(bytes: &[u8]) -> bool {
    let mut pos = 0usize;
    let value = match read_varint_u64(bytes, &mut pos) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if pos != bytes.len() {
        return false;
    }
    let mut reencoded = Vec::new();
    write_varint_u64(&mut reencoded, value);
    reencoded == bytes
}
//#endregion 🔖️Varint

//#region 🔖️Bytes
/// @emoji 👓️ A bounds-checked cursor over a borrowed byte slice — every read either succeeds
/// or returns `PackError` (`Truncated`), it never panics or reads out of bounds.
pub struct ByteReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn read_u8(&mut self) -> Result<u8, PackError> {
        Ok(self.read_bytes(1)?[0])
    }

    pub fn read_u16_le(&mut self) -> Result<u16, PackError> {
        let bytes = self.read_bytes(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_u32_le(&mut self) -> Result<u32, PackError> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_u64_le(&mut self) -> Result<u64, PackError> {
        let bytes = self.read_bytes(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(array))
    }

    pub fn read_f64_le(&mut self) -> Result<f64, PackError> {
        let bytes = self.read_bytes(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(f64::from_le_bytes(array))
    }

    pub fn read_varint_u64(&mut self) -> Result<u64, PackError> {
        read_varint_u64(self.bytes, &mut self.pos)
    }

    pub fn read_varint_i64(&mut self) -> Result<i64, PackError> {
        read_varint_i64(self.bytes, &mut self.pos)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], PackError> {
        if len > self.remaining() {
            return Err(PackError::Truncated(self.pos as u64));
        }
        let slice = &self.bytes[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_array32(&mut self) -> Result<[u8; 32], PackError> {
        let slice = self.read_bytes(32)?;
        let mut array = [0u8; 32];
        array.copy_from_slice(slice);
        Ok(array)
    }
}

/// @emoji ✍️ An append-only byte buffer with typed little-endian and varint writers.
#[derive(Default)]
pub struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn write_u16_le(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32_le(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u64_le(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_f64_le(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_varint_u64(&mut self, v: u64) {
        write_varint_u64(&mut self.buf, v);
    }

    pub fn write_varint_i64(&mut self, v: i64) {
        write_varint_i64(&mut self.buf, v);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }
}

//#endregion 🔖️Bytes

//#region 🔖️Crc
/// @emoji 🌀️ CRC-32C (Castagnoli) polynomial, reflected form, as used by iSCSI/ext4/pack.
const CRC32C_POLY: u32 = 0x82F6_3B78;

/// @emoji 📐️ Builds the 256-entry CRC-32C lookup table at compile time — no runtime init, no
/// dependency on the `crc`/`crc32c` crates.
const fn build_crc32c_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ CRC32C_POLY } else { crc >> 1 };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

const CRC32C_TABLE: [u32; 256] = build_crc32c_table();

/// @emoji 🧮️ Computes the CRC-32C (Castagnoli) checksum of `bytes`.
pub fn crc32c(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in bytes {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32C_TABLE[index];
    }
    crc ^ 0xFFFF_FFFF
}

/// @emoji 🧮️ Retained CRC-32C state for fixed-page readers that must yield between input pages.
pub struct Crc32cCursor {
    crc: u32,
}

impl Crc32cCursor {
    pub const fn new() -> Self {
        Self { crc: 0xffff_ffff }
    }

    pub fn update_page(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            let index = ((self.crc ^ u32::from(byte)) & 0xff) as usize;
            self.crc = CRC32C_TABLE[index] ^ (self.crc >> 8);
        }
    }

    pub const fn finish(&self) -> u32 {
        !self.crc
    }
}

impl Default for Crc32cCursor {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️Crc

//#region 🔖️Codec
/// @emoji 🗜️ A pluggable segment payload compressor/decompressor, identified by `CodecId`.
/// `decompress` must validate `raw_len` against `limit` BEFORE allocating the output buffer.
pub trait CompressionCodec {
    fn id(&self) -> CodecId;

    fn compress(&self, raw: &[u8]) -> Result<Vec<u8>, PackError>;

    fn decompress(&self, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError>;
}

/// @emoji 🚫️ The identity codec (`CodecId(0)`) — no compression, used as the default and as
/// the fallback when a segment's compressed flag bit is unset.
pub struct NoCompression;

impl CompressionCodec for NoCompression {
    fn id(&self) -> CodecId {
        CodecId(0)
    }

    fn compress(&self, raw: &[u8]) -> Result<Vec<u8>, PackError> {
        Ok(raw.to_vec())
    }

    fn decompress(&self, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError> {
        if raw_len > limit {
            return Err(PackError::LimitExceeded("NoCompression::decompress raw_len exceeds limit"));
        }
        if stored.len() as u64 != raw_len {
            return Err(PackError::Malformed { what: "codec", offset: 0, detail: "identity codec stored length does not match raw_len".to_string() });
        }
        Ok(stored.to_vec())
    }
}
//#endregion 🔖️Codec

//#region 🔖️Deflate
/// @emoji 🗜️ Deflate compression (first-party `semio-framework-deflate`, RFC 1951 raw DEFLATE) as
/// a `CodecId(1)` `CompressionCodec`.
#[cfg(feature = "deflate")]
pub struct DeflateCodec;

#[allow(clippy::unnecessary_wraps)] // the `not(feature = "deflate")` arm below also returns `Result`
pub fn deflate_compress(raw: &[u8]) -> Result<Vec<u8>, PackError> {
    #[cfg(feature = "deflate")]
    {
        Ok(semio_framework_deflate::deflate(raw))
    }
    #[cfg(not(feature = "deflate"))]
    {
        let _ = raw;
        Err(PackError::UnsupportedCodec(1))
    }
}

pub fn deflate_decompress(stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError> {
    if raw_len > limit {
        return Err(PackError::LimitExceeded("DeflateCodec::decompress raw_len exceeds limit"));
    }
    #[cfg(feature = "deflate")]
    {
        let out = semio_framework_deflate::inflate(stored, raw_len as usize).map_err(|_| PackError::Malformed { what: "deflate", offset: 0, detail: "decompression failed".to_string() })?;
        if out.len() as u64 != raw_len {
            return Err(PackError::Malformed { what: "deflate", offset: 0, detail: "decompressed length mismatch".to_string() });
        }
        Ok(out)
    }
    #[cfg(not(feature = "deflate"))]
    {
        let _ = stored;
        Err(PackError::UnsupportedCodec(1))
    }
}

/// 🌊️ One-byte retained DEFLATE result used by mounted pack readers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeflateRetainedStep {
    NeedInput,
    Byte(u8),
    Complete,
}

/// 🧵️ Incremental raw-DEFLATE decoder with exact producer handback and one output byte per grant.
#[cfg(feature = "deflate")]
pub struct DeflateRetainedCursor {
    inflater: Option<semio_framework_deflate::Inflater>,
    pending: Option<u8>,
    expected: u64,
    produced: u64,
    complete: bool,
}

#[cfg(feature = "deflate")]
impl DeflateRetainedCursor {
    pub fn try_new(expected: u64, limit: u64) -> Result<Self, PackError> {
        if expected > limit {
            return Err(PackError::LimitExceeded("retained deflate raw length exceeds limit"));
        }
        Ok(Self { inflater: Some(semio_framework_deflate::Inflater::new()), pending: None, expected, produced: 0, complete: false })
    }

    pub fn admit_byte(&mut self, byte: u8) -> Result<(), u8> {
        if self.complete || self.pending.is_some() {
            return Err(byte);
        }
        self.pending = Some(byte);
        Ok(())
    }

    pub fn grant(&mut self, input_complete: bool) -> Result<DeflateRetainedStep, PackError> {
        if self.complete {
            return Ok(DeflateRetainedStep::Complete);
        }
        let inflater = self.inflater.as_mut().ok_or(PackError::Malformed { what: "deflate", offset: self.produced, detail: "decoder is closed".into() })?;
        match inflater.advance(&mut self.pending, input_complete) {
            Ok(semio_framework_deflate::InflateOutcome::Wrote(byte)) => {
                self.produced = self.produced.checked_add(1).ok_or(PackError::LimitExceeded("retained deflate output overflow"))?;
                if self.produced > self.expected {
                    return Err(PackError::Malformed { what: "deflate", offset: self.produced, detail: "decompressed length exceeds declared raw length".into() });
                }
                Ok(DeflateRetainedStep::Byte(byte))
            }
            Ok(semio_framework_deflate::InflateOutcome::Done) => {
                if self.pending.is_some() || self.produced != self.expected {
                    return Err(PackError::Malformed { what: "deflate", offset: self.produced, detail: "decompressed length mismatch or trailing input".into() });
                }
                self.complete = true;
                Ok(DeflateRetainedStep::Complete)
            }
            Ok(semio_framework_deflate::InflateOutcome::NeedInput) => Ok(DeflateRetainedStep::NeedInput),
            Err(_) => Err(PackError::Malformed { what: "deflate", offset: self.produced, detail: "incremental decompression failed".into() }),
        }
    }

    pub fn can_admit(&self) -> bool {
        !self.complete && self.pending.is_none()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && self.pending.is_none() && self.inflater.is_none()
    }

    pub fn close(&mut self) {
        self.pending = None;
        self.complete = true;
        self.inflater = None;
    }
}

#[cfg(feature = "deflate")]
impl Drop for DeflateRetainedCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained deflate cursor reached Drop before terminal-empty close");
    }
}

#[cfg(feature = "deflate")]
impl CompressionCodec for DeflateCodec {
    fn id(&self) -> CodecId {
        CodecId(1)
    }

    fn compress(&self, raw: &[u8]) -> Result<Vec<u8>, PackError> {
        deflate_compress(raw)
    }

    fn decompress(&self, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError> {
        deflate_decompress(stored, raw_len, limit)
    }
}
//#endregion 🔖️Deflate

// 🧪️ Restores the `#[cfg(test)] mod tests` wrapper this region had lost — without it these fns
// compiled unconditionally as part of the plain `--lib` build, where the `#[async_test]` proc
// macro's dev-dependency is never linked.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
