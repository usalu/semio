//! 🚪️ IO stdio.zip (2.0/🧱️base) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_0::subsets::base::io::ZipAnalyzer;
    use crate::ZipSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct ZipComposerComposition;

    impl ArtifactComposition for ZipComposerComposition {
        type Snapshot = ZipSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "ZipComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = ZipAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "ZipComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🦑️DissolvedEngineCodec
// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
// MACHINES) — byte-level ZIP local-header/central-directory/EOCD parsing + reconstruction, kept
// together as one codec (rule 2). CRC32 is hand-rolled here (a pure format algorithm with no
// `ZipSnapshot` dependency of its own, kept with its only caller per rule 6 — also reused
// byte-for-byte by `📷️png`'s own `png_crc32`, since PNG's CRC is the identical ISO-HDLC
// polynomial); real compression is reused from the deflate artifact's own codec
// (`semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::{deflate_raw,
// inflate_raw}`) — never reimplemented here.
use crate::schema::snapshot::{ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata};
use crate::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

#[derive(Clone, Copy)]
enum NativeCompressionMethod {
    Stored,
    Deflate,
}

impl NativeCompressionMethod {
    fn from_code(code: u16) -> Option<Self> {
        match code {
            0 => Some(Self::Stored),
            8 => Some(Self::Deflate),
            _ => None,
        }
    }

    fn code(self) -> u16 {
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
fn u16_le(n: u16) -> [u8; 2] {
    n.to_le_bytes()
}
fn u32_le(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}
#[cfg(test)]
fn u64_le(n: u64) -> [u8; 8] {
    n.to_le_bytes()
}

fn read_u16(data: &[u8], off: usize) -> Result<u16, ZipError> {
    if off + 2 > data.len() {
        return Err(ZipError::Truncated("u16 field"));
    }
    Ok(u16::from_le_bytes([data[off], data[off + 1]]))
}
fn read_u32(data: &[u8], off: usize) -> Result<u32, ZipError> {
    if off + 4 > data.len() {
        return Err(ZipError::Truncated("u32 field"));
    }
    Ok(u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]))
}
fn read_u64(data: &[u8], off: usize) -> Result<u64, ZipError> {
    if off + 8 > data.len() {
        return Err(ZipError::Truncated("u64 field"));
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[off..off + 8]);
    Ok(u64::from_le_bytes(buf))
}
fn usize_from_u64(value: u64, what: &'static str) -> Result<usize, ZipError> {
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

fn cp437_encode(value: &str, what: &'static str) -> Result<Vec<u8>, ZipError> {
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
fn decode_zip_text(bytes: &[u8], utf8: bool, what: &'static str) -> Result<String, ZipError> {
    if utf8 {
        String::from_utf8(bytes.to_vec()).map_err(|_| ZipError::Utf8 { what, name_hint: cp437_decode(bytes) })
    } else {
        Ok(cp437_decode(bytes))
    }
}

/// 🔤️ Best-effort archive-comment decode (EOCD comment has no per-record encoding flag):
/// valid UTF-8 first, CP437 fallback otherwise.
fn decode_best_effort_text(bytes: &[u8]) -> (String, bool) {
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
const EXTRA_ZIP64: u16 = 0x0001;
const EXTRA_UNICODE_COMMENT: u16 = 0x6375;
const EXTRA_UNICODE_PATH: u16 = 0x7075;

fn parse_extra_fields(bytes: &[u8]) -> Result<Vec<ZipExtraField>, ZipError> {
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
struct Zip64Fields {
    uncomp_size: Option<u64>,
    comp_size: Option<u64>,
    local_offset: Option<u64>,
    disk_start: Option<u32>,
}

fn parse_zip64_extra(fields: &[ZipExtraField], need_uncomp: bool, need_comp: bool, need_offset: bool, need_disk: bool) -> Result<Zip64Fields, ZipError> {
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

fn extra_payload<'a>(fields: &'a [ZipExtraField], id: u16, label: &'static str) -> Result<Option<&'a [u8]>, ZipError> {
    let mut matches = fields.iter().filter(|field| field.id == id);
    let payload = matches.next().map(|field| field.data.as_slice());
    if matches.next().is_some() {
        return Err(ZipError::Malformed(format!("duplicate {label} extra records")));
    }
    Ok(payload)
}

fn decode_unicode_extra(payload: &[u8], legacy: &[u8], what: &'static str) -> Result<String, ZipError> {
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

fn normalized_extra_fields(fields: Vec<ZipExtraField>, derived_ids: &[u16]) -> Vec<ZipExtraField> {
    fields
        .into_iter()
        .map(|field| if derived_ids.contains(&field.id) { ZipExtraField { id: field.id, data: Vec::new() } } else { field })
        .collect()
}
//#endregion ExtraFields

//#region CentralInspect
/// 📇 Central-directory header fields surfaced for subset conformance checks without decoding
/// member payloads; the complete persisted header state remains on `ZipSnapshot` entries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZipCentralEntryHeader {
    pub name: String,
    pub flags: u16,
    pub version_needed: u16,
    pub compression_method:u16,
}

/// 🔎 Walks the central directory and returns per-entry general-purpose flags and version-needed
/// values. Does not decompress payloads — only enough structure to validate ISO/IEC 21320-1 header
/// policy against wire bytes.
pub fn inspect_zip_central_entry_headers(data: &[u8]) -> Result<Vec<ZipCentralEntryHeader>, ZipError> {
    let eocd = find_eocd(data)?;
    let loc = resolve_central_directory(data, eocd)?;
    let cd_end = loc.cd_offset.checked_add(loc.cd_size).ok_or_else(|| ZipError::Malformed("central-directory range overflows".into()))?;
    if cd_end > data.len() {
        return Err(ZipError::Malformed("central directory out of range".into()));
    }

    let mut out = Vec::with_capacity(loc.count);
    let mut pos = loc.cd_offset;
    for _ in 0..loc.count {
        if read_u32(data, pos)? != SIG_CENTRAL {
            return Err(ZipError::BadSignature { what: "central directory header", at: pos });
        }
        let version_needed = read_u16(data, pos + 6)?;
        let flags = read_u16(data, pos + 8)?;
        let name_len = read_u16(data, pos + 28)? as usize;
        let extra_len = read_u16(data, pos + 30)? as usize;
        let comment_len = read_u16(data, pos + 32)? as usize;
        let name_start = pos + 46;
        let name_end = name_start + name_len;
        let extra_end = name_end + extra_len;
        let comment_end = extra_end + comment_len;
        if comment_end > data.len() {
            return Err(ZipError::Truncated("central directory record (name/extra/comment)"));
        }
        let utf8 = flags & 0x0800 != 0;
        let name = decode_zip_text(&data[name_start..name_end], utf8, "central directory filename")?;
        out.push(ZipCentralEntryHeader { name, flags, version_needed,compression_method:read_u16(data,pos+10)? });
        pos = comment_end;
    }
    Ok(out)
}
//#endregion CentralInspect

//#region Eocd
const SIG_LOCAL: u32 = 0x0403_4b50;
const SIG_CENTRAL: u32 = 0x0201_4b50;
const SIG_EOCD: u32 = 0x0605_4b50;
const SIG_EOCD64_LOCATOR: u32 = 0x0706_4b50;
const SIG_EOCD64_RECORD: u32 = 0x0606_4b50;
const SIG_DATA_DESCRIPTOR: u32 = 0x0807_4b50;

fn find_eocd(data: &[u8]) -> Result<usize, ZipError> {
    if data.len() < 22 {
        return Err(ZipError::Truncated("archive shorter than a bare EOCD record"));
    }
    let max_comment = 65535usize;
    let start = data.len().saturating_sub(22 + max_comment);
    for i in (start..=data.len() - 22).rev() {
        if read_u32(data, i)? == SIG_EOCD && i + 22 + read_u16(data, i + 20)? as usize == data.len() {
            return Ok(i);
        }
    }
    Err(ZipError::BadSignature { what: "EOCD", at: data.len() })
}

/// 📍 Resolved central-directory location, disambiguated from ZIP64 records when the classic
/// EOCD fields carry sentinel values (count `0xFFFF`, size/offset `0xFFFFFFFF`).
struct CentralDirLocation {
    count: usize,
    cd_size: usize,
    cd_offset: usize,
    comment: String,
    comment_utf8: bool,
}

fn resolve_central_directory(data: &[u8], eocd: usize) -> Result<CentralDirLocation, ZipError> {
    let disk = read_u16(data, eocd + 4)?;
    let central_disk = read_u16(data, eocd + 6)?;
    let disk_count = read_u16(data, eocd + 8)?;
    let count16 = read_u16(data, eocd + 10)?;
    let cd_size32 = read_u32(data, eocd + 12)?;
    let cd_offset32 = read_u32(data, eocd + 16)?;
    let comment_len = read_u16(data, eocd + 20)? as usize;
    let comment_start = eocd + 22;
    let comment_end = comment_start + comment_len;
    if comment_end > data.len() {
        return Err(ZipError::Truncated("EOCD comment"));
    }
    let (comment, comment_utf8) = decode_best_effort_text(&data[comment_start..comment_end]);
    if disk != 0 || central_disk != 0 || (disk_count != count16 && disk_count != 0xFFFF) {
        return Err(ZipError::UnsupportedMultiDisk);
    }

    let needs_zip64 = count16 == 0xFFFF || cd_size32 == 0xFFFF_FFFF || cd_offset32 == 0xFFFF_FFFF;
    if !needs_zip64 {
        return Ok(CentralDirLocation { count: count16 as usize, cd_size: cd_size32 as usize, cd_offset: cd_offset32 as usize, comment, comment_utf8 });
    }

    if eocd < 20 || read_u32(data, eocd - 20)? != SIG_EOCD64_LOCATOR {
        return Err(ZipError::Malformed("ZIP64 sentinel in EOCD but no ZIP64 locator record precedes it".into()));
    }
    let locator = eocd - 20;
    let record_offset = read_u64(data, locator + 8)? as usize;
    if read_u32(data, record_offset)? != SIG_EOCD64_RECORD {
        return Err(ZipError::BadSignature { what: "EOCD64 record", at: record_offset });
    }
    if read_u32(data, record_offset + 16)? != 0 || read_u32(data, record_offset + 20)? != 0 || read_u32(data, locator + 4)? != 0 || read_u32(data, locator + 16)? != 1 {
        return Err(ZipError::UnsupportedMultiDisk);
    }
    let disk_entries = read_u64(data, record_offset + 24)?;
    let total_entries = read_u64(data, record_offset + 32)?;
    let cd_size = read_u64(data, record_offset + 40)?;
    let cd_offset = read_u64(data, record_offset + 48)?;
    if disk_entries != total_entries || total_entries > usize::MAX as u64 || cd_size > usize::MAX as u64 || cd_offset > usize::MAX as u64 {
        return Err(ZipError::UnsupportedZip64Write);
    }
    Ok(CentralDirLocation { count: total_entries as usize, cd_size: cd_size as usize, cd_offset: cd_offset as usize, comment, comment_utf8 })
}

//#endregion Eocd

//#region Decode
/// 🎒️ Decode ZIP container bytes into a name-keyed logical `ZipSnapshot`.
pub fn decode_zip(data: &[u8]) -> Result<ZipSnapshot, ZipError> {
    let eocd = find_eocd(data)?;
    let loc = resolve_central_directory(data, eocd)?;
    let cd_end = loc.cd_offset.checked_add(loc.cd_size).ok_or_else(|| ZipError::Malformed("central-directory range overflows".into()))?;
    if cd_end > data.len() {
        return Err(ZipError::Malformed("central directory out of range".into()));
    }

    let mut entries = Vec::with_capacity(loc.count);
    let mut pos = loc.cd_offset;
    for _ in 0..loc.count {
        if read_u32(data, pos)? != SIG_CENTRAL {
            return Err(ZipError::BadSignature { what: "central directory header", at: pos });
        }
        let version_made_by = read_u16(data, pos + 4)?;
        let version_needed = read_u16(data, pos + 6)?;
        let flags = read_u16(data, pos + 8)?;
        let utf8 = flags & 0x0800 != 0;
        let uses_descriptor = flags & 0x0008 != 0;
        let method_code = read_u16(data, pos + 10)?;
        let dos_time = read_u16(data, pos + 12)?;
        let dos_date = read_u16(data, pos + 14)?;
        let crc = read_u32(data, pos + 16)?;
        let comp_size32 = read_u32(data, pos + 20)?;
        let uncomp_size32 = read_u32(data, pos + 24)?;
        let name_len = read_u16(data, pos + 28)? as usize;
        let extra_len = read_u16(data, pos + 30)? as usize;
        let comment_len = read_u16(data, pos + 32)? as usize;
        let disk_start16 = read_u16(data, pos + 34)?;
        let internal_attrs = read_u16(data, pos + 36)?;
        let external_attrs = read_u32(data, pos + 38)?;
        let local_off32 = read_u32(data, pos + 42)?;

        let name_start = pos + 46;
        let name_end = name_start + name_len;
        let extra_start = name_end;
        let extra_end = extra_start + extra_len;
        let comment_start = extra_end;
        let comment_end = comment_start + comment_len;
        if comment_end > data.len() {
            return Err(ZipError::Truncated("central directory record (name/extra/comment)"));
        }

        let central_name_bytes = &data[name_start..name_end];
        let central_comment_bytes = &data[comment_start..comment_end];
        let central_extra = parse_extra_fields(&data[extra_start..extra_end])?;
        let central_unicode_path = extra_payload(&central_extra, EXTRA_UNICODE_PATH, "central Unicode path")?;
        let central_unicode_comment = extra_payload(&central_extra, EXTRA_UNICODE_COMMENT, "central Unicode comment")?;
        let name = match central_unicode_path {
            Some(payload) => decode_unicode_extra(payload, central_name_bytes, "central directory filename")?,
            None => decode_zip_text(central_name_bytes, utf8, "central directory filename")?,
        };
        let comment = match central_unicode_comment {
            Some(payload) => decode_unicode_extra(payload, central_comment_bytes, "central directory comment")?,
            None => decode_zip_text(central_comment_bytes, utf8, "central directory comment")?,
        };

        let zip64 = parse_zip64_extra(&central_extra, uncomp_size32 == 0xFFFF_FFFF, comp_size32 == 0xFFFF_FFFF, local_off32 == 0xFFFF_FFFF, disk_start16 == 0xFFFF)?;
        let uncomp_size = usize_from_u64(zip64.uncomp_size.unwrap_or(uncomp_size32 as u64), "uncompressed size")?;
        let comp_size = usize_from_u64(zip64.comp_size.unwrap_or(comp_size32 as u64), "compressed size")?;
        let local_off = usize_from_u64(zip64.local_offset.unwrap_or(local_off32 as u64), "local-header offset")?;
        let disk_start = zip64.disk_start.unwrap_or(disk_start16 as u32);
        if disk_start != 0 {
            return Err(ZipError::UnsupportedMultiDisk);
        }

        pos = comment_end;

        // ---- Local header ----
        if read_u32(data, local_off)? != SIG_LOCAL {
            return Err(ZipError::BadSignature { what: "local file header", at: local_off });
        }
        let l_version_needed = read_u16(data, local_off + 4)?;
        let l_flags = read_u16(data, local_off + 6)?;
        let l_method = read_u16(data, local_off + 8)?;
        let l_dos_time = read_u16(data, local_off + 10)?;
        let l_dos_date = read_u16(data, local_off + 12)?;
        let l_crc = read_u32(data, local_off + 14)?;
        let l_comp_size32 = read_u32(data, local_off + 18)?;
        let l_uncomp_size32 = read_u32(data, local_off + 22)?;
        if l_method != method_code {
            return Err(ZipError::MethodMismatch { name: name.clone(), local: l_method, central: method_code });
        }
        if l_flags & 0x0008 != flags & 0x0008 {
            return Err(ZipError::Malformed(format!("{name}: local and central data-descriptor flags disagree")));
        }
        let l_name_len = read_u16(data, local_off + 26)? as usize;
        let l_extra_len = read_u16(data, local_off + 28)? as usize;
        let l_name_start = local_off + 30;
        let l_name_end = l_name_start + l_name_len;
        let l_extra_start = l_name_end;
        let l_extra_end = l_extra_start + l_extra_len;
        if l_extra_end > data.len() {
            return Err(ZipError::Truncated("local file header name/extra"));
        }
        let local_extra = parse_extra_fields(&data[l_extra_start..l_extra_end])?;
        let local_name_bytes = &data[l_name_start..l_name_end];
        let local_utf8 = l_flags & 0x0800 != 0;
        let local_unicode_path = extra_payload(&local_extra, EXTRA_UNICODE_PATH, "local Unicode path")?;
        let local_name = match local_unicode_path {
            Some(payload) => decode_unicode_extra(payload, local_name_bytes, "local filename")?,
            None => decode_zip_text(local_name_bytes, local_utf8, "local filename")?,
        };
        if local_name != name {
            return Err(ZipError::Malformed(format!("{name}: local and central filenames disagree")));
        }
        let local_zip64 = parse_zip64_extra(&local_extra, l_uncomp_size32 == 0xFFFF_FFFF, l_comp_size32 == 0xFFFF_FFFF, false, false)?;
        if !uses_descriptor {
            let local_uncomp_size = usize_from_u64(local_zip64.uncomp_size.unwrap_or(l_uncomp_size32 as u64), "local uncompressed size")?;
            let local_comp_size = usize_from_u64(local_zip64.comp_size.unwrap_or(l_comp_size32 as u64), "local compressed size")?;
            if l_crc != crc || local_uncomp_size != uncomp_size || local_comp_size != comp_size {
                return Err(ZipError::Malformed(format!("{name}: local and central CRC/sizes disagree")));
            }
        }

        let method = NativeCompressionMethod::from_code(method_code).ok_or_else(|| ZipError::UnsupportedMethod { name: name.clone(), method: method_code })?;

        let data_off = l_extra_end;
        let data_end = data_off.checked_add(comp_size).ok_or_else(|| ZipError::Malformed("entry payload offset overflows".into()))?;
        if data_end > data.len() {
            return Err(ZipError::Truncated("entry payload"));
        }
        let payload = &data[data_off..data_end];

        // ---- Optional trailing data descriptor (general-purpose bit 3) ----
        let mut data_descriptor_signature = false;
        if uses_descriptor {
            let is_zip64_entry = zip64.uncomp_size.is_some() || zip64.comp_size.is_some() || local_zip64.uncomp_size.is_some() || local_zip64.comp_size.is_some();
            let descriptor_matches = |at: usize| -> Result<bool, ZipError> {
                let (d_crc, d_comp, d_uncomp) = if is_zip64_entry {
                    (read_u32(data, at)?, usize_from_u64(read_u64(data, at + 4)?, "descriptor compressed size")?, usize_from_u64(read_u64(data, at + 12)?, "descriptor uncompressed size")?)
                } else {
                    (read_u32(data, at)?, read_u32(data, at + 4)? as usize, read_u32(data, at + 8)? as usize)
                };
                Ok(d_crc == crc && d_comp == comp_size && d_uncomp == uncomp_size)
            };
            let first = read_u32(data, data_end)?;
            let signed = first == SIG_DATA_DESCRIPTOR && descriptor_matches(data_end + 4)?;
            let unsigned = descriptor_matches(data_end)?;
            if signed {
                data_descriptor_signature = true;
            } else if !unsigned {
                return Err(ZipError::DataDescriptorMismatch { name: name.clone() });
            }
        }

        let raw = match method {
            NativeCompressionMethod::Stored => payload.to_vec(),
            NativeCompressionMethod::Deflate => semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::inflate_raw(payload).map_err(ZipError::Malformed)?,
        };
        if raw.len() != uncomp_size {
            return Err(ZipError::Malformed(format!("{name}: decompressed size {} != declared uncompressed size {uncomp_size}", raw.len())));
        }
        let got_crc = crc32(&raw);
        if got_crc != crc {
            return Err(ZipError::Crc32Mismatch { name, expected: crc, actual: got_crc });
        }

        let local_unicode_path_legacy_name = local_unicode_path.map(|_| local_name_bytes.to_vec());
        let central_unicode_path_legacy_name = central_unicode_path.map(|_| central_name_bytes.to_vec());
        let unicode_comment_legacy = central_unicode_comment.map(|_| central_comment_bytes.to_vec());
        let metadata = ZipEntryMetadata {
            compression_method: method_code,
            local: ZipLocalHeaderMetadata {
                version_needed: l_version_needed,
                flags: l_flags,
                modified_time: l_dos_time,
                modified_date: l_dos_date,
                extra_fields: normalized_extra_fields(local_extra, &[EXTRA_ZIP64, EXTRA_UNICODE_PATH]),
                unicode_path_legacy_name: local_unicode_path_legacy_name,
            },
            central: ZipCentralHeaderMetadata {
                version_made_by,
                version_needed,
                flags,
                modified_time: dos_time,
                modified_date: dos_date,
                extra_fields: normalized_extra_fields(central_extra, &[EXTRA_ZIP64, EXTRA_UNICODE_PATH, EXTRA_UNICODE_COMMENT]),
                unicode_path_legacy_name: central_unicode_path_legacy_name,
                comment,
                unicode_comment_legacy,
                internal_attributes: internal_attrs,
                external_attributes: external_attrs,
            },
            data_descriptor_signature,
        };
        entries.push(ZipEntry { name, data: raw, metadata });
    }

    if pos != cd_end {
        return Err(ZipError::Malformed("central-directory size does not match its records".into()));
    }
    let snapshot = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: loc.comment, comment_utf8: loc.comment_utf8 };
    validate_zip_snapshot_serialization(&snapshot)?;
    Ok(snapshot)
}
//#endregion Decode

//#region Encode
fn encode_zip_text(value: &str, utf8: bool, what: &'static str) -> Result<Vec<u8>, ZipError> {
    if utf8 { Ok(value.as_bytes().to_vec()) } else { cp437_encode(value, what) }
}

fn contains_extra(fields: &[ZipExtraField], id: u16) -> bool {
    fields.iter().any(|field| field.id == id)
}

fn encoded_header_text(value: &str, flags: u16, fields: &[ZipExtraField], derived_id: u16, legacy: &Option<Vec<u8>>, what: &'static str) -> Result<Vec<u8>, ZipError> {
    let marked = contains_extra(fields, derived_id);
    match (marked, legacy) {
        (true, Some(bytes)) => Ok(bytes.clone()),
        (false, None) => encode_zip_text(value, flags & 0x0800 != 0, what),
        (true, None) => Err(ZipError::Malformed(format!("{what} Unicode extra marker requires explicit legacy bytes"))),
        (false, Some(_)) => Err(ZipError::Malformed(format!("{what} legacy bytes require a Unicode extra marker"))),
    }
}

fn unicode_extra_payload(legacy: &[u8], value: &str) -> Vec<u8> {
    let mut payload = Vec::with_capacity(5 + value.len());
    payload.push(1);
    payload.extend_from_slice(&u32_le(crc32(legacy)));
    payload.extend_from_slice(value.as_bytes());
    payload
}

fn encode_extra_fields(fields: &[ZipExtraField], derived: &[(u16, Vec<u8>)]) -> Result<Vec<u8>, ZipError> {
    let mut seen = std::collections::HashSet::new();
    let mut output = Vec::new();
    for field in fields {
        if !seen.insert(field.id) && derived.iter().any(|(id, _)| *id == field.id) {
            return Err(ZipError::Malformed(format!("duplicate derived extra field 0x{:04x}", field.id)));
        }
        let data = if let Some((_, value)) = derived.iter().find(|(id, _)| *id == field.id) {
            if !field.data.is_empty() {
                return Err(ZipError::Malformed(format!("derived extra field 0x{:04x} must have an empty persisted payload", field.id)));
            }
            value.as_slice()
        } else {
            field.data.as_slice()
        };
        if data.len() > u16::MAX as usize || output.len().saturating_add(4).saturating_add(data.len()) > u16::MAX as usize {
            return Err(ZipError::Malformed("extra-field block exceeds 65535 bytes".into()));
        }
        output.extend_from_slice(&u16_le(field.id));
        output.extend_from_slice(&u16_le(data.len() as u16));
        output.extend_from_slice(data);
    }
    Ok(output)
}

fn validate_flags(flags: u16, name: &str) -> Result<(), ZipError> {
    const UNSUPPORTED: u16 = 0x0001 | 0x0010 | 0x0020 | 0x0040 | 0x2000;
    if flags & UNSUPPORTED != 0 {
        return Err(ZipError::Malformed(format!("{name}: encrypted or masked headers cannot be regenerated")));
    }
    Ok(())
}

/// 🛡️ Refuses serialization states that cannot save and reopen as the same logical snapshot.
pub fn validate_zip_snapshot_serialization(snapshot: &ZipSnapshot) -> Result<(), ZipError> {
    let archive_comment = encode_zip_text(&snapshot.comment, snapshot.comment_utf8, "archive comment")?;
    if archive_comment.len() > u16::MAX as usize {
        return Err(ZipError::Malformed("archive comment too long".into()));
    }
    if !snapshot.comment_utf8 && std::str::from_utf8(&archive_comment).is_ok() {
        return Err(ZipError::Malformed("CP437 archive comment bytes are valid UTF-8 and would reopen ambiguously".into()));
    }
    let mut names = std::collections::HashSet::new();
    for entry in &snapshot.entries {
        if entry.name.is_empty() || !names.insert(entry.name.as_str()) {
            return Err(ZipError::Malformed("ZIP member names must be nonempty and unique".into()));
        }
        let metadata = &entry.metadata;
        NativeCompressionMethod::from_code(metadata.compression_method).ok_or_else(|| ZipError::UnsupportedMethod { name: entry.name.clone(), method: metadata.compression_method })?;
        validate_flags(metadata.local.flags, &entry.name)?;
        validate_flags(metadata.central.flags, &entry.name)?;
        if metadata.local.flags & 0x0008 != metadata.central.flags & 0x0008 {
            return Err(ZipError::Malformed(format!("{}: local and central data-descriptor flags disagree", entry.name)));
        }
        if metadata.data_descriptor_signature && metadata.local.flags & 0x0008 == 0 {
            return Err(ZipError::Malformed(format!("{}: descriptor signature requested without data-descriptor flags", entry.name)));
        }
        if contains_extra(&metadata.local.extra_fields, EXTRA_ZIP64) && metadata.local.version_needed < 45 {
            return Err(ZipError::Malformed(format!("{}: local ZIP64 marker requires version-needed 45 or newer", entry.name)));
        }
        if contains_extra(&metadata.central.extra_fields, EXTRA_ZIP64) && metadata.central.version_needed < 45 {
            return Err(ZipError::Malformed(format!("{}: central ZIP64 marker requires version-needed 45 or newer", entry.name)));
        }
        let local_name = encoded_header_text(
            &entry.name,
            metadata.local.flags,
            &metadata.local.extra_fields,
            EXTRA_UNICODE_PATH,
            &metadata.local.unicode_path_legacy_name,
            "local filename",
        )?;
        let central_name = encoded_header_text(
            &entry.name,
            metadata.central.flags,
            &metadata.central.extra_fields,
            EXTRA_UNICODE_PATH,
            &metadata.central.unicode_path_legacy_name,
            "central filename",
        )?;
        let central_comment = encoded_header_text(
            &metadata.central.comment,
            metadata.central.flags,
            &metadata.central.extra_fields,
            EXTRA_UNICODE_COMMENT,
            &metadata.central.unicode_comment_legacy,
            "central comment",
        )?;
        if local_name.len() > u16::MAX as usize || central_name.len() > u16::MAX as usize || central_comment.len() > u16::MAX as usize {
            return Err(ZipError::Malformed(format!("{}: encoded name or comment exceeds 65535 bytes", entry.name)));
        }
        let mut local_derived = Vec::new();
        if contains_extra(&metadata.local.extra_fields, EXTRA_ZIP64) {
            local_derived.push((EXTRA_ZIP64, vec![0; 16]));
        }
        if let Some(legacy) = &metadata.local.unicode_path_legacy_name {
            local_derived.push((EXTRA_UNICODE_PATH, unicode_extra_payload(legacy, &entry.name)));
        }
        encode_extra_fields(&metadata.local.extra_fields, &local_derived)?;
        let mut central_derived = Vec::new();
        if contains_extra(&metadata.central.extra_fields, EXTRA_ZIP64) {
            central_derived.push((EXTRA_ZIP64, vec![0; 24]));
        }
        if let Some(legacy) = &metadata.central.unicode_path_legacy_name {
            central_derived.push((EXTRA_UNICODE_PATH, unicode_extra_payload(legacy, &entry.name)));
        }
        if let Some(legacy) = &metadata.central.unicode_comment_legacy {
            central_derived.push((EXTRA_UNICODE_COMMENT, unicode_extra_payload(legacy, &metadata.central.comment)));
        }
        encode_extra_fields(&metadata.central.extra_fields, &central_derived)?;
    }
    Ok(())
}

/// 🎒️ Deterministically materializes logical members as local headers, central directory, and EOCD
/// while retaining every persisted header choice and regenerating CRC, sizes, offsets, ZIP64 and
/// Unicode-extra payloads from their semantic owners.
pub fn encode_zip(snapshot: &ZipSnapshot) -> Result<Vec<u8>, ZipError> {
    validate_zip_snapshot_serialization(snapshot)?;
    encode_zip_ordered(snapshot, snapshot.entries.iter().collect())
}

pub(crate) fn encode_zip_with_entry_names(snapshot: &ZipSnapshot, names: &[String]) -> Result<Vec<u8>, ZipError> {
    if names.len() != snapshot.entries.len() {
        return Err(ZipError::Malformed("derived entry order does not cover every logical member".into()));
    }
    let mut seen = std::collections::HashSet::new();
    let mut ordered = Vec::with_capacity(names.len());
    for name in names {
        if !seen.insert(name.as_str()) {
            return Err(ZipError::Malformed(format!("derived entry order repeats {name}")));
        }
        let entry = snapshot.entries.iter().find(|entry| &entry.name == name).ok_or_else(|| ZipError::Malformed(format!("derived entry order references missing member {name}")))?;
        ordered.push(entry);
    }
    encode_zip_ordered(snapshot, ordered)
}

fn encode_zip_ordered(snapshot: &ZipSnapshot, ordered: Vec<&ZipEntry>) -> Result<Vec<u8>, ZipError> {
    if snapshot.entries.len() > 0xFFFF {
        return Err(ZipError::UnsupportedZip64Write);
    }

    let mut locals = Vec::new();
    let mut central = Vec::new();

    for entry in ordered {
        let metadata = &entry.metadata;
        validate_flags(metadata.local.flags, &entry.name)?;
        validate_flags(metadata.central.flags, &entry.name)?;
        if metadata.local.flags & 0x0008 != metadata.central.flags & 0x0008 {
            return Err(ZipError::Malformed(format!("{}: local and central data-descriptor flags disagree", entry.name)));
        }
        let uses_descriptor = metadata.local.flags & 0x0008 != 0;
        if metadata.data_descriptor_signature && !uses_descriptor {
            return Err(ZipError::Malformed(format!("{}: descriptor signature requested without data-descriptor flags", entry.name)));
        }
        let local_name = encoded_header_text(
            &entry.name,
            metadata.local.flags,
            &metadata.local.extra_fields,
            EXTRA_UNICODE_PATH,
            &metadata.local.unicode_path_legacy_name,
            "local filename",
        )?;
        let central_name = encoded_header_text(
            &entry.name,
            metadata.central.flags,
            &metadata.central.extra_fields,
            EXTRA_UNICODE_PATH,
            &metadata.central.unicode_path_legacy_name,
            "central filename",
        )?;
        let central_comment = encoded_header_text(
            &metadata.central.comment,
            metadata.central.flags,
            &metadata.central.extra_fields,
            EXTRA_UNICODE_COMMENT,
            &metadata.central.unicode_comment_legacy,
            "central comment",
        )?;
        if local_name.len() > u16::MAX as usize || central_name.len() > u16::MAX as usize {
            return Err(ZipError::Malformed("entry name too long".into()));
        }
        if central_comment.len() > u16::MAX as usize {
            return Err(ZipError::Malformed("entry comment too long".into()));
        }
        let crc = crc32(&entry.data);
        let method = NativeCompressionMethod::from_code(metadata.compression_method).ok_or_else(|| ZipError::UnsupportedMethod { name: entry.name.clone(), method: metadata.compression_method })?;
        let payload = match method {
            NativeCompressionMethod::Stored => entry.data.clone(),
            NativeCompressionMethod::Deflate => {
                if entry.name.to_ascii_lowercase().ends_with(".bin") {
                    semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::deflate_raw_deterministic_compact_high_search(&entry.data)
                } else if entry.name.to_ascii_lowercase().ends_with(".emf") {
                    semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::deflate_raw_deterministic_high_search(&entry.data)
                } else {
                    semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::deflate_raw_deterministic(&entry.data)
                }
                .map_err(ZipError::Malformed)?
            }
        };
        if payload.len() > u32::MAX as usize || entry.data.len() > u32::MAX as usize {
            return Err(ZipError::UnsupportedZip64Write);
        }
        let comp_size = payload.len() as u32;
        let uncomp_size = entry.data.len() as u32;
        let offset = locals.len() as u64;
        let local_zip64 = contains_extra(&metadata.local.extra_fields, EXTRA_ZIP64);
        let central_zip64 = contains_extra(&metadata.central.extra_fields, EXTRA_ZIP64);
        if offset > u32::MAX as u64 && !central_zip64 {
            return Err(ZipError::UnsupportedZip64Write);
        }

        let mut local_zip64_payload = Vec::new();
        local_zip64_payload.extend_from_slice(&(entry.data.len() as u64).to_le_bytes());
        local_zip64_payload.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        let mut local_derived = Vec::new();
        if local_zip64 {
            local_derived.push((EXTRA_ZIP64, local_zip64_payload));
        }
        if let Some(legacy) = &metadata.local.unicode_path_legacy_name {
            local_derived.push((EXTRA_UNICODE_PATH, unicode_extra_payload(legacy, &entry.name)));
        }
        let local_extra = encode_extra_fields(&metadata.local.extra_fields, &local_derived)?;

        let mut central_zip64_payload = Vec::new();
        central_zip64_payload.extend_from_slice(&(entry.data.len() as u64).to_le_bytes());
        central_zip64_payload.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        central_zip64_payload.extend_from_slice(&offset.to_le_bytes());
        let mut central_derived = Vec::new();
        if central_zip64 {
            central_derived.push((EXTRA_ZIP64, central_zip64_payload));
        }
        if let Some(legacy) = &metadata.central.unicode_path_legacy_name {
            central_derived.push((EXTRA_UNICODE_PATH, unicode_extra_payload(legacy, &entry.name)));
        }
        if let Some(legacy) = &metadata.central.unicode_comment_legacy {
            central_derived.push((EXTRA_UNICODE_COMMENT, unicode_extra_payload(legacy, &metadata.central.comment)));
        }
        let central_extra = encode_extra_fields(&metadata.central.extra_fields, &central_derived)?;

        let local_crc = if uses_descriptor { 0 } else { crc };
        let local_comp_size = if local_zip64 { 0xFFFF_FFFF } else if uses_descriptor { 0 } else { comp_size };
        let local_uncomp_size = if local_zip64 { 0xFFFF_FFFF } else if uses_descriptor { 0 } else { uncomp_size };
        let central_comp_size = if central_zip64 { 0xFFFF_FFFF } else { comp_size };
        let central_uncomp_size = if central_zip64 { 0xFFFF_FFFF } else { uncomp_size };
        let central_offset = if central_zip64 { 0xFFFF_FFFF } else { offset as u32 };

        let mut local = Vec::new();
        local.extend_from_slice(&u32_le(SIG_LOCAL));
        local.extend_from_slice(&u16_le(metadata.local.version_needed));
        local.extend_from_slice(&u16_le(metadata.local.flags));
        local.extend_from_slice(&u16_le(method.code()));
        local.extend_from_slice(&u16_le(metadata.local.modified_time));
        local.extend_from_slice(&u16_le(metadata.local.modified_date));
        local.extend_from_slice(&u32_le(local_crc));
        local.extend_from_slice(&u32_le(local_comp_size));
        local.extend_from_slice(&u32_le(local_uncomp_size));
        local.extend_from_slice(&u16_le(local_name.len() as u16));
        local.extend_from_slice(&u16_le(local_extra.len() as u16));
        local.extend_from_slice(&local_name);
        local.extend_from_slice(&local_extra);
        local.extend_from_slice(&payload);
        if uses_descriptor {
            if metadata.data_descriptor_signature {
                local.extend_from_slice(&u32_le(SIG_DATA_DESCRIPTOR));
            }
            local.extend_from_slice(&u32_le(crc));
            if local_zip64 || central_zip64 {
                local.extend_from_slice(&(payload.len() as u64).to_le_bytes());
                local.extend_from_slice(&(entry.data.len() as u64).to_le_bytes());
            } else {
                local.extend_from_slice(&u32_le(comp_size));
                local.extend_from_slice(&u32_le(uncomp_size));
            }
        }

        let mut cen = Vec::new();
        cen.extend_from_slice(&u32_le(SIG_CENTRAL));
        cen.extend_from_slice(&u16_le(metadata.central.version_made_by));
        cen.extend_from_slice(&u16_le(metadata.central.version_needed));
        cen.extend_from_slice(&u16_le(metadata.central.flags));
        cen.extend_from_slice(&u16_le(method.code()));
        cen.extend_from_slice(&u16_le(metadata.central.modified_time));
        cen.extend_from_slice(&u16_le(metadata.central.modified_date));
        cen.extend_from_slice(&u32_le(crc));
        cen.extend_from_slice(&u32_le(central_comp_size));
        cen.extend_from_slice(&u32_le(central_uncomp_size));
        cen.extend_from_slice(&u16_le(central_name.len() as u16));
        cen.extend_from_slice(&u16_le(central_extra.len() as u16));
        cen.extend_from_slice(&u16_le(central_comment.len() as u16));
        cen.extend_from_slice(&u16_le(0)); // disk number start — single-disk archives only
        cen.extend_from_slice(&u16_le(metadata.central.internal_attributes));
        cen.extend_from_slice(&u32_le(metadata.central.external_attributes));
        cen.extend_from_slice(&u32_le(central_offset));
        cen.extend_from_slice(&central_name);
        cen.extend_from_slice(&central_extra);
        cen.extend_from_slice(&central_comment);

        locals.extend_from_slice(&local);
        central.extend_from_slice(&cen);
    }

    let cd_offset = locals.len() as u64;
    let cd_size = central.len() as u64;
    let count = snapshot.entries.len() as u16;
    if cd_offset > u32::MAX as u64 || cd_size > u32::MAX as u64 {
        return Err(ZipError::UnsupportedZip64Write);
    }
    let archive_comment = encode_zip_text(&snapshot.comment, snapshot.comment_utf8, "archive comment")?;
    if archive_comment.len() > u16::MAX as usize {
        return Err(ZipError::Malformed("archive comment too long".into()));
    }

    let mut eocd = Vec::new();
    eocd.extend_from_slice(&u32_le(SIG_EOCD));
    eocd.extend_from_slice(&u16_le(0)); // disk number
    eocd.extend_from_slice(&u16_le(0)); // disk with central directory start
    eocd.extend_from_slice(&u16_le(count));
    eocd.extend_from_slice(&u16_le(count));
    eocd.extend_from_slice(&u32_le(cd_size as u32));
    eocd.extend_from_slice(&u32_le(cd_offset as u32));
    eocd.extend_from_slice(&u16_le(archive_comment.len() as u16));
    eocd.extend_from_slice(&archive_comment);

    let mut out = locals;
    out.extend_from_slice(&central);
    out.extend_from_slice(&eocd);
    Ok(out)
}

/// 🎒️ The member a document archive carries its authoritative DSL text in.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn document_archive_member<S: store::ArtifactDsl>() -> String {
    format!("snapshot.{}.semio", S::EXTENSION)
}

/// 🎒️ A zip 2.0 archive of one artifact document: its canonical DSL text as
/// [`document_archive_member`] (authoritative, read back by [`decode_document_archive`]) and its
/// rfc8259 rendition as `snapshot.json` for readers without a semio parser. Both are lossless, so
/// the hop is `IoFidelity::Exact` — the shared zip carrier every artifact whose own shape is its
/// archive content uses, instead of each owner restating it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_document_archive<S: store::ArtifactDsl + semio_framework_value::ToValue>(document: &S) -> Result<Vec<u8>, ZipError> {
    let entries = vec![
        ZipEntry { name: document_archive_member::<S>(), data: document.print_dsl().into_bytes(), ..Default::default() },
        ZipEntry { name: "snapshot.json".into(), data: semio_framework_pack_json::to_json_string(document).into_bytes(), ..Default::default() },
    ];
    encode_zip(&ZipSnapshot { entries, ..ZipSnapshot::default() })
}

/// 🎒️ The document inside an archive written by [`encode_document_archive`]: its DSL member parsed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_document_archive<S: store::ArtifactDsl>(bytes: &[u8]) -> Result<S, String> {
    let archive = decode_zip(bytes).map_err(|error| error.to_string())?;
    let member = document_archive_member::<S>();
    let entry = archive.entries.iter().find(|entry| entry.name == member).ok_or_else(|| format!("the archive has no {member} member"))?;
    S::parse_dsl(std::str::from_utf8(&entry.data).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}
//#endregion Encode

//#region Sniff
/// 🎚️ Byte-level sniff confidence — kept local to the codec (no framework dependency here);
/// the analyzer maps this onto `semio_framework_plugin::io::Confidence` at the layer that owns that type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SniffConfidence {
    High,
    Medium,
    Low,
}

/// 🕵️ Structural sniff: recognizes ZIP local-file-header (`PK\x03\x04`), empty-archive EOCD
/// (`PK\x05\x06`), and spanned-archive (`PK\x07\x08`) magics, corroborated by actually finding
/// a well-formed EOCD record — never a constant, always a function of `data`.
pub fn sniff_zip_bytes(data: &[u8]) -> SniffConfidence {
    if data.len() < 4 {
        return SniffConfidence::Low;
    }
    let magic = &data[0..4];
    let starts_recognized = magic == [0x50, 0x4b, 0x03, 0x04] || magic == [0x50, 0x4b, 0x05, 0x06] || magic == [0x50, 0x4b, 0x07, 0x08];
    let eocd_ok = find_eocd(data).is_ok();
    match (starts_recognized, eocd_ok) {
        (true, true) => SniffConfidence::High,
        (true, false) => SniffConfidence::Medium,
        (false, true) => SniffConfidence::Medium,
        (false, false) => SniffConfidence::Low,
    }
}
//#endregion Sniff

//#region 🧪️CodecTests
#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#endregion 🧪️CodecTests
//#endregion 🦑️DissolvedEngineCodec

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — pure `ComposerEntry` aggregation, no engine needed. Aggregates BOTH the `🧱️base` and
/// `🌐️iso21320` `ComposerEntry` rows (this standard has two subsets). NOTE: always reach this via a
/// fully-qualified path (`standards::v2_0::subsets::any::io::io_registry::entries()`) — the
/// artifact root's OWN `io_registry` (`🗿️artifacts/🎒️zip/🦀️.rs`) shadows this name with a
/// DIFFERENT return type (`&'static [&'static ComposerEntry]` vs this module's
/// `&'static [ComposerEntry]`); a bare `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::standards::v2_0::subsets::base::io::ZipComposer as ZipRawAnyComposer;
    use crate::standards::v2_0::subsets::iso21320::io::ZipIso21320Composer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<ZipRawAnyComposer>(), composer_entry_of::<ZipIso21320Composer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::snapshot::ZipEntry;
    use crate::{ZipDiff, ZipMutation, ZipSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.zip` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct ZipBuilderConstruction {
        snapshot: ZipSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for ZipBuilderConstruction {
        type Snapshot = ZipSnapshot;
        type Mutation = ZipMutation;
        type Diff = ZipDiff;
        fn empty() -> Self {
            Self { snapshot: ZipSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    impl ZipBuilderConstruction {
        /// ➕️ Adds a logical member; native compression is deterministic serializer policy.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            let mut entry = ZipEntry { name: name.into(), data, ..Default::default() };
            entry.metadata.compression_method = 0;
            entry.metadata.local.version_needed = 10;
            entry.metadata.central.version_needed = 10;
            self.snapshot.entries.push(entry);
            self
        }

        /// ➕️ Adds a logical member; native compression is deterministic serializer policy.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            self.snapshot.entries.push(ZipEntry { name: name.into(), data, ..Default::default() });
            self
        }

        /// ➕️ Adds a fully-specified member (metadata-faithful construction path).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_entry(mut self, entry: ZipEntry) -> Self {
            self.snapshot.entries.push(entry);
            self
        }

        /// 💬️ Sets the archive-level (EOCD) comment.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.snapshot.comment = comment.into();
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::ZipSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.zip` parts.
    #[derive(Clone, Debug, Default)]
    pub struct ZipParts {
        pub snapshot: Option<ZipSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.zip` (2.0/🧱️base) sources.
    pub struct ZipAnalyzerAnalysis;

    impl ArtifactAnalysis for ZipAnalyzerAnalysis {
        type Parts = ZipParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            // 🕵️ Real sniff: inspects the argument's bytes (magic + a well-formed EOCD), never a
            // constant. `AnalyzeSource::Text` is the hex-envelope DSL form, not raw container bytes,
            // so it can't be magic-sniffed the same way — treated as low confidence here (the DSL
            // envelope preamble, not this sniff, is what actually recognizes it).
            use crate::standards::v2_0::subsets::base::io::{sniff_zip_bytes, SniffConfidence};
            match source {
                AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                    SniffConfidence::High => semio_framework_plugin::io::Confidence::High,
                    SniffConfidence::Medium => semio_framework_plugin::io::Confidence::Medium,
                    SniffConfidence::Low => semio_framework_plugin::io::Confidence::Low,
                },
                AnalyzeSource::Text(_) => semio_framework_plugin::io::Confidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ZipParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ZipSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        let result = if matches!(crate::standards::v2_0::subsets::base::io::sniff_zip_bytes(bytes), crate::standards::v2_0::subsets::base::io::SniffConfidence::High) {
                            crate::standards::v2_0::subsets::base::io::decode_zip(bytes).map_err(|err| err.to_string())
                        } else {
                            <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|err| err.to_string())
                        };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = semio_framework_plugin::io::Confidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec ZipBuilderFacets {
        construction: ZipBuilderConstruction,
        analysis: ZipAnalyzerAnalysis,
        composition: crate::standards::v2_0::subsets::base::io::derived_composition::ZipComposerComposition,
    }
    builder: ZipBuilder,
    analyzer: ZipAnalyzer,
    composer: ZipComposer,
);
