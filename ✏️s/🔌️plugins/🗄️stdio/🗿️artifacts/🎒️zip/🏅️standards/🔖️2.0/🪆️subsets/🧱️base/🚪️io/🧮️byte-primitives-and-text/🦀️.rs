//! 🧮️ Byte primitives and text of the zip codec: the compression method, the CRC32, the error type, the little-endian readers, the CP437 and UTF-8 member text decoding and the ZIP64 and Unicode extra fields. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
#[derive(Clone, Copy)]
pub(super) enum NativeCompressionMethod {
    Stored,
    Deflate,
}

impl NativeCompressionMethod {
    pub(super) fn from_code(code: u16) -> Option<Self> {
        match code {
            0 => Some(Self::Stored),
            8 => Some(Self::Deflate),
            _ => None,
        }
    }

    pub(super) fn code(self) -> u16 {
        match self {
            Self::Stored => 0,
            Self::Deflate => 8,
        }
    }
}

//#region CRC32
fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        table[i as usize] = c;
    }
    table
}

/// 🧮 CRC-32 (ISO-HDLC / ZIP).
pub fn crc32(data: &[u8]) -> u32 {
    let table = crc32_table();
    let mut c = 0xFFFFFFFFu32;
    for &b in data {
        c = table[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFFFFFF
}
//#endregion CRC32

//#region Error
/// ⚠️ Typed ZIP decode/encode failure — every unsupported-but-observed shape (exotic
/// compression method, multi-disk archive, zip64-required write) surfaces here rather
/// than being silently dropped, truncated, or fabricated.
#[derive(Clone, Debug, PartialEq)]
pub enum ZipError {
    Truncated(&'static str),
    BadSignature { what: &'static str, at: usize },
    Utf8 { what: &'static str, name_hint: String },
    Crc32Mismatch { name: String, expected: u32, actual: u32 },
    MethodMismatch { name: String, local: u16, central: u16 },
    UnsupportedMethod { name: String, method: u16 },
    UnsupportedExtraField { id: u16 },
    UnsupportedMultiDisk,
    UnsupportedZip64Write,
    DataDescriptorMismatch { name: String },
    Malformed(String),
}

impl std::fmt::Display for ZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated(what) => write!(f, "zip: truncated ({what})"),
            Self::BadSignature { what, at } => write!(f, "zip: bad {what} signature at offset {at}"),
            Self::Utf8 { what, name_hint } => write!(f, "zip: invalid utf-8 in {what} ({name_hint})"),
            Self::Crc32Mismatch { name, expected, actual } => {
                write!(f, "zip: crc32 mismatch for {name}: expected {expected:#010x}, got {actual:#010x}")
            }
            Self::MethodMismatch { name, local, central } => {
                write!(f, "zip: method mismatch for {name}: local header says {local}, central directory says {central}")
            }
            Self::UnsupportedMethod { name, method } => write!(f, "zip: unsupported compression method {method} for {name}"),
            Self::UnsupportedExtraField { id } => write!(f, "zip: unsupported extra field 0x{id:04x}"),
            Self::UnsupportedMultiDisk => write!(f, "zip: multi-disk archives are unsupported"),
            Self::UnsupportedZip64Write => write!(f, "zip: snapshot requires ZIP64 (>4GiB entry, >65535 entries, or >4GiB archive) which this writer does not emit"),
            Self::DataDescriptorMismatch { name } => write!(f, "zip: trailing data descriptor disagrees with central directory for {name}"),
            Self::Malformed(msg) => write!(f, "zip: malformed archive: {msg}"),
        }
    }
}

impl std::error::Error for ZipError {}

impl ZipError {
    /// 🧷️ Unsupported methods, extra fields, multi-disk and ZIP64 writes are unsupported owners; every other archive refusal is invalid input.
    pub const fn refusal_kind(&self) -> semio_framework_value::ValueRefusalKind {
        match self {
            Self::UnsupportedMethod { .. } | Self::UnsupportedExtraField { .. } | Self::UnsupportedMultiDisk | Self::UnsupportedZip64Write => semio_framework_value::ValueRefusalKind::UnsupportedOwner,
            _ => semio_framework_value::ValueRefusalKind::InvalidValue,
        }
    }
}

/// 🪢️ Moves the archive refusal into the canonical Value refusal with its own kind.
impl From<ZipError> for semio_framework_value::ValueError {
    fn from(error: ZipError) -> Self {
        Self::new(error.refusal_kind(), error.to_string())
    }
}

impl ZipError {
    /// 🧭️ Preserves the authored ZIP failure category at typed artifact boundaries.
    pub fn into_value_error(self) -> semio_framework_value::ValueError {
        use semio_framework_value::ValueRefusalKind;
        let kind = match &self {
            Self::UnsupportedMethod { .. } | Self::UnsupportedExtraField { .. } | Self::UnsupportedMultiDisk | Self::UnsupportedZip64Write => ValueRefusalKind::UnsupportedOwner,
            Self::Truncated(_) | Self::BadSignature { .. } | Self::Utf8 { .. } | Self::Crc32Mismatch { .. } | Self::MethodMismatch { .. } | Self::DataDescriptorMismatch { .. } | Self::Malformed(_) => ValueRefusalKind::InvalidValue,
        };
        semio_framework_value::ValueError::new(kind, self.to_string())
    }
}
//#endregion Error

//#region ByteReaders
pub fn u16_le(n: u16) -> [u8; 2] {
    n.to_le_bytes()
}

pub fn u32_le(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

#[cfg(any(test, feature = "testkit"))]
pub fn u64_le(n: u64) -> [u8; 8] {
    n.to_le_bytes()
}

pub(super) fn read_u16(data: &[u8], off: usize) -> Result<u16, ZipError> {
    if off + 2 > data.len() {
        return Err(ZipError::Truncated("u16 field"));
    }
    Ok(u16::from_le_bytes([data[off], data[off + 1]]))
}

pub(super) fn read_u32(data: &[u8], off: usize) -> Result<u32, ZipError> {
    if off + 4 > data.len() {
        return Err(ZipError::Truncated("u32 field"));
    }
    Ok(u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]))
}

pub(super) fn read_u64(data: &[u8], off: usize) -> Result<u64, ZipError> {
    if off + 8 > data.len() {
        return Err(ZipError::Truncated("u64 field"));
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[off..off + 8]);
    Ok(u64::from_le_bytes(buf))
}

pub(super) fn usize_from_u64(value: u64, what: &'static str) -> Result<usize, ZipError> {
    usize::try_from(value).map_err(|_| ZipError::Malformed(format!("{what} exceeds the current platform address space")))
}
//#endregion ByteReaders

//#region Cp437
/// 🔤️ Upper half (0x80-0xFF) of code page 437 → Unicode scalar. Bytes 0x00-0x7F map to
/// themselves (ASCII), exactly like every legacy zip tool's fallback when general-purpose
/// bit 11 is unset — this is a real decode difference from UTF-8, not a cosmetic one.
const CP437_HIGH: [char; 128] = [
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', 'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', 'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', '░',
    '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', 'α', 'ß',
    'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00a0}',
];

fn cp437_decode(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| if b < 0x80 { b as char } else { CP437_HIGH[(b - 0x80) as usize] }).collect()
}

pub(super) fn cp437_encode(value: &str, what: &'static str) -> Result<Vec<u8>, ZipError> {
    value
        .chars()
        .map(|character| {
            if (character as u32) < 0x80 {
                Ok(character as u8)
            } else {
                CP437_HIGH
                    .iter()
                    .position(|candidate| *candidate == character)
                    .map(|index| index as u8 + 0x80)
                    .ok_or_else(|| ZipError::Utf8 { what, name_hint: value.into() })
            }
        })
        .collect()
}

/// 🔤️ Decodes a filename/comment field per general-purpose bit 11: UTF-8 when set, CP437 otherwise.
pub(super) fn decode_zip_text(bytes: &[u8], utf8: bool, what: &'static str) -> Result<String, ZipError> {
    if utf8 {
        String::from_utf8(bytes.to_vec()).map_err(|_| ZipError::Utf8 { what, name_hint: cp437_decode(bytes) })
    } else {
        Ok(cp437_decode(bytes))
    }
}

/// 🔤️ Best-effort archive-comment decode (EOCD comment has no per-record encoding flag):
/// valid UTF-8 first, CP437 fallback otherwise.
pub(super) fn decode_best_effort_text(bytes: &[u8]) -> (String, bool) {
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => (value, true),
        Err(_) => (cp437_decode(bytes), false),
    }
}

/// 🔤️ Keeps a legacy EOCD comment encoding only when the replacement has an unambiguous CP437
/// wire representation; otherwise the atomic text edit promotes the comment to UTF-8.
pub fn archive_comment_utf8_after_edit(current_utf8: bool, replacement: &str) -> bool {
    if current_utf8 {
        return true;
    }
    match cp437_encode(replacement, "archive comment") {
        Ok(bytes) => std::str::from_utf8(&bytes).is_ok(),
        Err(_) => true,
    }
}
//#endregion Cp437

//#region ExtraFields
pub const EXTRA_ZIP64: u16 = 0x0001;

pub const EXTRA_UNICODE_COMMENT: u16 = 0x6375;

pub const EXTRA_UNICODE_PATH: u16 = 0x7075;

pub(super) fn parse_extra_fields(bytes: &[u8]) -> Result<Vec<ZipExtraField>, ZipError> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 4 <= bytes.len() {
        let id = u16::from_le_bytes([bytes[i], bytes[i + 1]]);
        let size = u16::from_le_bytes([bytes[i + 2], bytes[i + 3]]) as usize;
        let start = i + 4;
        let end = start + size;
        if end > bytes.len() {
            return Err(ZipError::Malformed("extra field record overruns its block".into()));
        }
        out.push(ZipExtraField { id, data: bytes[start..end].to_vec() });
        i = end;
    }
    if i != bytes.len() {
        return Err(ZipError::Malformed("extra-field block has trailing bytes".into()));
    }
    Ok(out)
}

/// 🐘️ ZIP64 extended-info (0x0001) field values, consumed in APPNOTE 4.5.3 order: only the
/// sub-fields whose classic 32/16-bit counterpart is the sentinel value are present, in the
/// fixed order uncompressed-size, compressed-size, local-header-offset, disk-start-number.
pub(super) struct Zip64Fields {
    pub(super) uncomp_size: Option<u64>,
    pub(super) comp_size: Option<u64>,
    pub(super) local_offset: Option<u64>,
    pub(super) disk_start: Option<u32>,
}

pub(super) fn parse_zip64_extra(fields: &[ZipExtraField], need_uncomp: bool, need_comp: bool, need_offset: bool, need_disk: bool) -> Result<Zip64Fields, ZipError> {
    if !(need_uncomp || need_comp || need_offset || need_disk) {
        return Ok(Zip64Fields { uncomp_size: None, comp_size: None, local_offset: None, disk_start: None });
    }
    let mut records = fields.iter().filter(|field| field.id == EXTRA_ZIP64);
    let record = records.next().ok_or_else(|| ZipError::Malformed("32-bit sentinel field present without a ZIP64 extra record".into()))?;
    if records.next().is_some() {
        return Err(ZipError::Malformed("duplicate ZIP64 extra records".into()));
    }
    let mut pos = 0usize;
    let mut out = Zip64Fields { uncomp_size: None, comp_size: None, local_offset: None, disk_start: None };
    if need_uncomp {
        out.uncomp_size = Some(read_u64(&record.data, pos)?);
        pos += 8;
    }
    if need_comp {
        out.comp_size = Some(read_u64(&record.data, pos)?);
        pos += 8;
    }
    if need_offset {
        out.local_offset = Some(read_u64(&record.data, pos)?);
        pos += 8;
    }
    if need_disk {
        out.disk_start = Some(read_u32(&record.data, pos)?);
    }
    Ok(out)
}

pub(super) fn extra_payload<'a>(fields: &'a [ZipExtraField], id: u16, label: &'static str) -> Result<Option<&'a [u8]>, ZipError> {
    let mut matches = fields.iter().filter(|field| field.id == id);
    let payload = matches.next().map(|field| field.data.as_slice());
    if matches.next().is_some() {
        return Err(ZipError::Malformed(format!("duplicate {label} extra records")));
    }
    Ok(payload)
}

pub(super) fn decode_unicode_extra(payload: &[u8], legacy: &[u8], what: &'static str) -> Result<String, ZipError> {
    if payload.len() < 5 || payload[0] != 1 {
        return Err(ZipError::Malformed(format!("{what} Unicode extra has an unsupported shape")));
    }
    let expected = read_u32(payload, 1)?;
    let actual = crc32(legacy);
    if expected != actual {
        return Err(ZipError::Malformed(format!("{what} Unicode extra CRC disagrees with its legacy bytes")));
    }
    String::from_utf8(payload[5..].to_vec()).map_err(|_| ZipError::Utf8 { what, name_hint: cp437_decode(legacy) })
}

pub(super) fn normalized_extra_fields(fields: Vec<ZipExtraField>, derived_ids: &[u16]) -> Vec<ZipExtraField> {
    fields
        .into_iter()
        .map(|field| if derived_ids.contains(&field.id) { ZipExtraField { id: field.id, data: Vec::new() } } else { field })
        .collect()
}
//#endregion ExtraFields
//#endregion 🦑️DissolvedEngineCodec
