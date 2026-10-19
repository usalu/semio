//! 🧱️ Header and footer of the pack format: the magic, version and flag constants, the header record with its codec and the footer record with its checksum. See [`super`].

use super::*;

//#region 🔖️Header
/// 🧲️ The 8-byte magic every `.spk` pack file begins with.
pub const MAGIC: [u8; 8] = [0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];

/// 📏️ Fixed wire size of the header, in bytes.
pub const HEADER_SIZE: usize = 32;

/// 🔢️ The container format version this crate writes and reads.
pub const FORMAT_VERSION_MAJOR: u16 = 1;

/// 🔢️ The container format minor version this crate writes.
pub const FORMAT_VERSION_MINOR: u16 = 0;

/// 🗜️ Required flag bit: at least one segment/chunk in this file uses compression.
pub const REQUIRED_COMPRESSED: u32 = 1 << 0;

/// 🧱️ Required flag bit: this file contains a chunk table.
pub const REQUIRED_CHUNKED: u32 = 1 << 1;

/// 🔒️ Required flag bit: reserved for encryption, never set by this crate.
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;

/// ⛓️ Required flag bit: reserved for footer chaining.
pub const REQUIRED_FOOTER_CHAIN: u32 = 1 << 3;

pub const REQUIRED_KNOWN_MASK: u32 = REQUIRED_COMPRESSED | REQUIRED_CHUNKED | REQUIRED_ENCRYPTED | REQUIRED_FOOTER_CHAIN;

pub fn chunk_custody_matches(required_flags:u32,chunk_count:u64,table_span:ByteRange)->bool{(required_flags&REQUIRED_CHUNKED!=0)==(table_span.len!=0)&&(chunk_count!=0)==(table_span.len!=0)}
fn validate_chunk_custody(required_flags:u32,chunk_count:u64,table_span:ByteRange)->Result<(),PackRefusal>{if !chunk_custody_matches(required_flags,chunk_count,table_span){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"required_chunked",offset:12,detail:"REQUIRED_CHUNKED disagrees with declared chunk table custody"});}Ok(())}
/// 🧮️ Optional flag bit: the document body was encoded in canonical form.
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;

/// 🌊️ Optional flag bit: the file was produced by a streaming writer.
pub const OPTIONAL_STREAMED: u32 = 1 << 1;

/// 🧬️ Optional flag bit: a schema segment is present.
pub const OPTIONAL_HAS_SCHEMA: u32 = 1 << 2;

/// 🪪️ The fixed 32-byte superblock header: magic, version, feature flags, self-CRC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub optional_flags: u32,
}

impl Header {
    /// ✍️ Serializes to the exact 32-byte wire form, computing `header_crc32` over
    /// bytes `0..20` and zeroing the 8 reserved bytes.
    // 🧮️ Every helper this fn calls (`crc32c`, `PackSink::write_all`) is a first-party `async fn`
    // one hop away in `semio-framework-replication`, which this packet does not own; R9 rule 3
    // ("if every consumer can become async, make it async instead") applies, not R9 rule 2 — see
    // 📓️terra-pack-finish-report.md §"pure-computation-made-async: the recipe".
    pub(super) async fn write_bytes(&self) -> [u8; HEADER_SIZE] {
        self.write_bytes_sync()
    }

    pub(super) fn write_bytes_sync(&self)->[u8;HEADER_SIZE]{
        let mut buf = [0u8; HEADER_SIZE];
        buf[0..8].copy_from_slice(&MAGIC);
        buf[8..10].copy_from_slice(&self.version_major.to_le_bytes());
        buf[10..12].copy_from_slice(&self.version_minor.to_le_bytes());
        buf[12..16].copy_from_slice(&self.required_flags.to_le_bytes());
        buf[16..20].copy_from_slice(&self.optional_flags.to_le_bytes());
        let crc = crc32c(&buf[0..20]);
        buf[20..24].copy_from_slice(&crc.to_le_bytes());
        buf
    }

    /// 📖️ Parses and validates a 32-byte header: magic, self-CRC, and that
    /// `required_flags` sets no bit outside the known `0..=3` range.
    pub(super) async fn parse(bytes: &[u8]) -> Result<Self, PackRefusal> {
        if bytes.len() < HEADER_SIZE {
            return Err(PackRefusal::Truncated(bytes.len() as u64));
        }
        if bytes[0..8] != MAGIC {
            return Err(PackRefusal::BadMagic);
        }
        let version_major = u16::from_le_bytes([bytes[8], bytes[9]]);
        let version_minor = u16::from_le_bytes([bytes[10], bytes[11]]);
        let required_flags = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let optional_flags = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let stored_crc = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        let computed_crc = crc32c(&bytes[0..20]);
        if stored_crc != computed_crc {
            return Err(PackRefusal::ChecksumMismatch { segment: "header", offset: 20 });
        }
        let unknown = required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err(PackRefusal::UnknownRequiredFlags(unknown));
        }
        if version_major != FORMAT_VERSION_MAJOR {
            return Err(PackRefusal::UnsupportedVersion { major: version_major, minor: version_minor });
        }
        Ok(Self { version_major, version_minor, required_flags, optional_flags })
    }
}
//#endregion 🔖️Header

//#region 🔖️Footer
/// 🧲️ The 8-byte magic the footer begins with.
pub const FOOTER_MAGIC: [u8; 8] = *b"SPKFOOT1";

/// 📏️ Fixed wire size of the footer, in bytes, at the end of the file.
/// See the module doc for why this is 84, not the contract prose's arithmetically-inconsistent
/// "80" (the prose sum omits the trailing `footer_crc32` field's own 4 bytes).
pub const FOOTER_SIZE: usize = 84;

/// 🪶️ The fixed-size trailer every pack file ends with — the single root of trust a
/// reader locates by seeking to `file_len - FOOTER_SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Footer {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub manifest_offset: u64,
    pub manifest_len: u64,
    pub file_len: u64,
    pub content_hash: ContentHash,
    pub prev_footer_offset: u64,
}

impl Footer {
    /// ✍️ Serializes to the exact 84-byte wire form, computing `footer_crc32` over the
    /// preceding 80 bytes.
    // 🧮️ Same R9-rule-3 reasoning as `Header::write_bytes` above — `crc32c` is a pure but
    // externally-owned `async fn`, so this fn propagates `async` rather than fighting it.
    pub(super) async fn write_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(FOOTER_SIZE);
        buf.extend_from_slice(&FOOTER_MAGIC);
        buf.extend_from_slice(&self.version_major.to_le_bytes());
        buf.extend_from_slice(&self.version_minor.to_le_bytes());
        buf.extend_from_slice(&self.required_flags.to_le_bytes());
        buf.extend_from_slice(&self.manifest_offset.to_le_bytes());
        buf.extend_from_slice(&self.manifest_len.to_le_bytes());
        buf.extend_from_slice(&self.file_len.to_le_bytes());
        buf.extend_from_slice(&self.content_hash.0);
        buf.extend_from_slice(&self.prev_footer_offset.to_le_bytes());
        let crc = crc32c(&buf);
        buf.extend_from_slice(&crc.to_le_bytes());
        buf
    }

    /// 📖️ Parses and validates an 84-byte footer: magic and self-CRC over the first 80
    /// bytes. Does not cross-check `file_len` against an actual source — callers that have one
    /// should do so themselves (see `PackFile::open_superblock`).
    pub(super) async fn parse(bytes: &[u8]) -> Result<Self, PackRefusal> {
        if bytes.len() < FOOTER_SIZE {
            return Err(PackRefusal::Truncated(bytes.len() as u64));
        }
        if bytes[0..8] != FOOTER_MAGIC {
            return Err(PackRefusal::BadMagic);
        }
        let version_major = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        let version_minor = u16::from_le_bytes(bytes[10..12].try_into().unwrap());
        let required_flags = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let manifest_offset = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let manifest_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
        let file_len = u64::from_le_bytes(bytes[32..40].try_into().unwrap());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes[40..72]);
        let prev_footer_offset = u64::from_le_bytes(bytes[72..80].try_into().unwrap());
        let stored_crc = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        let computed_crc = crc32c(&bytes[0..80]);
        if stored_crc != computed_crc {
            return Err(PackRefusal::ChecksumMismatch { segment: "footer", offset: 80 });
        }
        Ok(Self { version_major, version_minor, required_flags, manifest_offset, manifest_len, file_len, content_hash: ContentHash(hash), prev_footer_offset })
    }
}
//#endregion 🔖️Footer
