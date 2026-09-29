//! 🎒️ First-party ZIP container (APPNOTE 6.3.10) over this crate's raw DEFLATE: a deterministic writer (every entry
//! deflated, fixed 1980-01-01 00:00 timestamp, UTF-8 names, no extra fields) and a bounded reader of stored (0) and
//! deflated (8) entries located through the central directory, each checked against its CRC-32. Zip64, encryption and
//! multi-disk archives are refused, not guessed. The `zip` crate survives only as the dev-dependency oracle.
//! See <https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT>.

use crate::{deflate, inflate, DeflateError};

const LOCAL_HEADER: u32 = 0x0403_4b50;
const CENTRAL_HEADER: u32 = 0x0201_4b50;
const END_OF_CENTRAL_DIRECTORY: u32 = 0x0605_4b50;
const METHOD_STORED: u16 = 0;
const METHOD_DEFLATED: u16 = 8;
const FLAG_ENCRYPTED: u16 = 1;
const FLAG_UTF8: u16 = 1 << 11;
const VERSION: u16 = 20;
const DOS_TIME: u16 = 0;
const DOS_DATE: u16 = (1 << 5) | 1;
const END_RECORD_LEN: usize = 22;
const MAX_COMMENT_LEN: usize = 0xFFFF;

/// 🚨️ Why an archive could not be written or read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZipArchiveError {
    Truncated(&'static str),
    Signature(&'static str),
    Unsupported(&'static str),
    UnsupportedMethod(u16),
    NameNotUtf8,
    MissingEntry(String),
    TooLarge(String),
    Crc(String),
    Inflate(String, DeflateError),
}

impl std::fmt::Display for ZipArchiveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated(what) => write!(formatter, "zip: truncated {what}"),
            Self::Signature(what) => write!(formatter, "zip: bad {what} signature"),
            Self::Unsupported(what) => write!(formatter, "zip: unsupported {what}"),
            Self::UnsupportedMethod(method) => write!(formatter, "zip: unsupported compression method {method}"),
            Self::NameNotUtf8 => write!(formatter, "zip: entry name is not UTF-8"),
            Self::MissingEntry(name) => write!(formatter, "zip: missing entry {name}"),
            Self::TooLarge(name) => write!(formatter, "zip: entry {name} exceeds its size bound"),
            Self::Crc(name) => write!(formatter, "zip: CRC-32 mismatch in {name}"),
            Self::Inflate(name, error) => write!(formatter, "zip: cannot inflate {name}: {error:?}"),
        }
    }
}

impl std::error::Error for ZipArchiveError {}

/// 🧮️ The 256-entry ISO 3309 CRC-32 table, built at compile time.
const CRC_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 { 0xEDB8_8320 ^ (value >> 1) } else { value >> 1 };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
};

/// 🧮️ ISO 3309 / ITU-T V.42 CRC-32 of `data`, the checksum every ZIP entry carries.
pub fn crc32(data: &[u8]) -> u32 {
    !data.iter().fold(u32::MAX, |crc, byte| CRC_TABLE[((crc ^ u32::from(*byte)) & 0xFF) as usize] ^ (crc >> 8))
}

fn put_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn get_u16(bytes: &[u8], at: usize, what: &'static str) -> Result<u16, ZipArchiveError> {
    bytes.get(at..at + 2).map(|slice| u16::from_le_bytes([slice[0], slice[1]])).ok_or(ZipArchiveError::Truncated(what))
}

fn get_u32(bytes: &[u8], at: usize, what: &'static str) -> Result<u32, ZipArchiveError> {
    bytes.get(at..at + 4).map(|slice| u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])).ok_or(ZipArchiveError::Truncated(what))
}

fn narrow_u32(value: usize, name: &str) -> Result<u32, ZipArchiveError> {
    u32::try_from(value).ok().filter(|narrow| *narrow != u32::MAX).ok_or_else(|| ZipArchiveError::TooLarge(name.into()))
}

/// 📤️ Builds one archive entry by entry; `finish` appends the central directory. Output is a pure function of the
/// entries and their order, so the same entries always produce the same bytes.
#[derive(Debug, Default)]
pub struct ZipWriter {
    body: Vec<u8>,
    central: Vec<u8>,
    entries: u16,
}

impl ZipWriter {
    /// 📤️ An empty archive.
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Appends `bytes` deflated under the UTF-8 `name`.
    pub fn add(&mut self, name: &str, bytes: &[u8]) -> Result<(), ZipArchiveError> {
        let compressed = deflate(bytes);
        let name_len = u16::try_from(name.len()).map_err(|_| ZipArchiveError::TooLarge(name.into()))?;
        let crc = crc32(bytes);
        let (compressed_len, raw_len, offset) = (narrow_u32(compressed.len(), name)?, narrow_u32(bytes.len(), name)?, narrow_u32(self.body.len(), name)?);
        self.entries = self.entries.checked_add(1).filter(|count| *count != u16::MAX).ok_or(ZipArchiveError::Unsupported("entry count beyond 65534"))?;
        put_u32(&mut self.body, LOCAL_HEADER);
        for value in [VERSION, FLAG_UTF8, METHOD_DEFLATED, DOS_TIME, DOS_DATE] {
            put_u16(&mut self.body, value);
        }
        for value in [crc, compressed_len, raw_len] {
            put_u32(&mut self.body, value);
        }
        put_u16(&mut self.body, name_len);
        put_u16(&mut self.body, 0);
        self.body.extend_from_slice(name.as_bytes());
        self.body.extend_from_slice(&compressed);
        put_u32(&mut self.central, CENTRAL_HEADER);
        for value in [VERSION, VERSION, FLAG_UTF8, METHOD_DEFLATED, DOS_TIME, DOS_DATE] {
            put_u16(&mut self.central, value);
        }
        for value in [crc, compressed_len, raw_len] {
            put_u32(&mut self.central, value);
        }
        for value in [name_len, 0, 0, 0, 0] {
            put_u16(&mut self.central, value);
        }
        put_u32(&mut self.central, 0);
        put_u32(&mut self.central, offset);
        self.central.extend_from_slice(name.as_bytes());
        Ok(())
    }

    /// 🏁️ The finished archive: the entries, their central directory and the end record.
    pub fn finish(mut self) -> Result<Vec<u8>, ZipArchiveError> {
        let central_offset = narrow_u32(self.body.len(), "central directory")?;
        let central_len = narrow_u32(self.central.len(), "central directory")?;
        self.body.extend_from_slice(&self.central);
        put_u32(&mut self.body, END_OF_CENTRAL_DIRECTORY);
        for value in [0, 0, self.entries, self.entries] {
            put_u16(&mut self.body, value);
        }
        put_u32(&mut self.body, central_len);
        put_u32(&mut self.body, central_offset);
        put_u16(&mut self.body, 0);
        Ok(self.body)
    }
}

/// 🗂️ One entry of a parsed central directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZipEntryHeader {
    pub name: String,
    pub method: u16,
    pub crc: u32,
    pub compressed_len: u32,
    pub raw_len: u32,
    local_offset: u32,
}

/// 📥️ A read-only view of an archive's bytes, located through its central directory.
#[derive(Clone, Debug)]
pub struct ZipArchive<'bytes> {
    bytes: &'bytes [u8],
    entries: Vec<ZipEntryHeader>,
}

impl<'bytes> ZipArchive<'bytes> {
    /// 🔎️ Finds the end record (searching back over a trailing comment), then parses every central directory entry.
    pub fn parse(bytes: &'bytes [u8]) -> Result<Self, ZipArchiveError> {
        if bytes.len() < END_RECORD_LEN {
            return Err(ZipArchiveError::Truncated("end of central directory"));
        }
        let lowest = bytes.len().saturating_sub(END_RECORD_LEN + MAX_COMMENT_LEN);
        let end = (lowest..=bytes.len() - END_RECORD_LEN).rev().find(|at| get_u32(bytes, *at, "end record").ok() == Some(END_OF_CENTRAL_DIRECTORY)).ok_or(ZipArchiveError::Signature("end of central directory"))?;
        if get_u16(bytes, end + 4, "end record")? != 0 || get_u16(bytes, end + 6, "end record")? != 0 {
            return Err(ZipArchiveError::Unsupported("multi-disk archive"));
        }
        let count = get_u16(bytes, end + 10, "end record")?;
        let central_offset = get_u32(bytes, end + 16, "end record")?;
        if count == u16::MAX || central_offset == u32::MAX {
            return Err(ZipArchiveError::Unsupported("zip64 archive"));
        }
        let mut at = central_offset as usize;
        let mut entries = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            if get_u32(bytes, at, "central directory")? != CENTRAL_HEADER {
                return Err(ZipArchiveError::Signature("central directory"));
            }
            if get_u16(bytes, at + 8, "central directory")? & FLAG_ENCRYPTED != 0 {
                return Err(ZipArchiveError::Unsupported("encrypted entry"));
            }
            let (name_len, extra_len, comment_len) = (usize::from(get_u16(bytes, at + 28, "central directory")?), usize::from(get_u16(bytes, at + 30, "central directory")?), usize::from(get_u16(bytes, at + 32, "central directory")?));
            let name = bytes.get(at + 46..at + 46 + name_len).ok_or(ZipArchiveError::Truncated("entry name"))?;
            let header = ZipEntryHeader {
                name: String::from_utf8(name.to_vec()).map_err(|_| ZipArchiveError::NameNotUtf8)?,
                method: get_u16(bytes, at + 10, "central directory")?,
                crc: get_u32(bytes, at + 16, "central directory")?,
                compressed_len: get_u32(bytes, at + 20, "central directory")?,
                raw_len: get_u32(bytes, at + 24, "central directory")?,
                local_offset: get_u32(bytes, at + 42, "central directory")?,
            };
            if [header.compressed_len, header.raw_len, header.local_offset].contains(&u32::MAX) {
                return Err(ZipArchiveError::Unsupported("zip64 entry"));
            }
            entries.push(header);
            at += 46 + name_len + extra_len + comment_len;
        }
        Ok(Self { bytes, entries })
    }

    /// 🗂️ Every entry in central-directory order.
    pub fn entries(&self) -> &[ZipEntryHeader] {
        &self.entries
    }

    /// 📥️ The bytes of the entry named `name`, refused beyond `max_len` bytes and checked against its CRC-32.
    pub fn read(&self, name: &str, max_len: usize) -> Result<Vec<u8>, ZipArchiveError> {
        let header = self.entries.iter().find(|entry| entry.name == name).ok_or_else(|| ZipArchiveError::MissingEntry(name.into()))?;
        self.read_entry(header, max_len)
    }

    /// 📥️ The bytes of one parsed entry, refused beyond `max_len` bytes and checked against its CRC-32.
    pub fn read_entry(&self, header: &ZipEntryHeader, max_len: usize) -> Result<Vec<u8>, ZipArchiveError> {
        if header.raw_len as usize > max_len {
            return Err(ZipArchiveError::TooLarge(header.name.clone()));
        }
        let at = header.local_offset as usize;
        if get_u32(self.bytes, at, "local header")? != LOCAL_HEADER {
            return Err(ZipArchiveError::Signature("local header"));
        }
        let data_at = at + 30 + usize::from(get_u16(self.bytes, at + 26, "local header")?) + usize::from(get_u16(self.bytes, at + 28, "local header")?);
        let stored = self.bytes.get(data_at..data_at + header.compressed_len as usize).ok_or(ZipArchiveError::Truncated("entry data"))?;
        let bytes = match header.method {
            METHOD_STORED => stored.to_vec(),
            METHOD_DEFLATED => inflate(stored, header.raw_len as usize).map_err(|error| ZipArchiveError::Inflate(header.name.clone(), error))?,
            method => return Err(ZipArchiveError::UnsupportedMethod(method)),
        };
        if bytes.len() != header.raw_len as usize || crc32(&bytes) != header.crc {
            return Err(ZipArchiveError::Crc(header.name.clone()));
        }
        Ok(bytes)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
