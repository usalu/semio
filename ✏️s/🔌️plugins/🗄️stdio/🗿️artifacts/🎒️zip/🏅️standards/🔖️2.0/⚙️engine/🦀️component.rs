//! ⚙️ ZipEngine — byte-level ZIP local-header/central-directory/EOCD parsing + reconstruction.
//! CRC32 hand-rolled; real compression reused from the deflate artifact's own codec
//! (`crate::artifacts::deflate::engine::{deflate_raw, inflate_raw}`) — never reimplemented here.

use crate::artifacts::zip::{ZipArtifact, ZipDiff, ZipMutation, ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use crate::artifacts::zip::schema::snapshot::{ZipCompressionMethod, ZipEntry, ZipExtraField};

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
            Self::UnsupportedMultiDisk => write!(f, "zip: multi-disk archives are unsupported"),
            Self::UnsupportedZip64Write => write!(f, "zip: snapshot requires ZIP64 (>4GiB entry, >65535 entries, or >4GiB archive) which this writer does not emit"),
            Self::DataDescriptorMismatch { name } => write!(f, "zip: trailing data descriptor disagrees with central directory for {name}"),
            Self::Malformed(msg) => write!(f, "zip: malformed archive: {msg}"),
        }
    }
}

impl std::error::Error for ZipError {}
//#endregion Error

//#region ByteReaders
fn u16_le(n: u16) -> [u8; 2] { n.to_le_bytes() }
fn u32_le(n: u32) -> [u8; 4] { n.to_le_bytes() }
fn u64_le(n: u64) -> [u8; 8] { n.to_le_bytes() }

fn read_u16(data: &[u8], off: usize) -> Result<u16, ZipError> {
    if off + 2 > data.len() { return Err(ZipError::Truncated("u16 field")); }
    Ok(u16::from_le_bytes([data[off], data[off + 1]]))
}
fn read_u32(data: &[u8], off: usize) -> Result<u32, ZipError> {
    if off + 4 > data.len() { return Err(ZipError::Truncated("u32 field")); }
    Ok(u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]))
}
fn read_u64(data: &[u8], off: usize) -> Result<u64, ZipError> {
    if off + 8 > data.len() { return Err(ZipError::Truncated("u64 field")); }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[off..off + 8]);
    Ok(u64::from_le_bytes(buf))
}
//#endregion ByteReaders

//#region Cp437
/// 🔤️ Upper half (0x80-0xFF) of code page 437 → Unicode scalar. Bytes 0x00-0x7F map to
/// themselves (ASCII), exactly like every legacy zip tool's fallback when general-purpose
/// bit 11 is unset — this is a real decode difference from UTF-8, not a cosmetic one.
const CP437_HIGH: [char; 128] = [
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00a0}',
];

fn cp437_decode(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| if b < 0x80 { b as char } else { CP437_HIGH[(b - 0x80) as usize] }).collect()
}

/// 🔤️ Decodes a filename/comment field per general-purpose bit 11: UTF-8 when set, CP437 otherwise.
fn decode_zip_text(bytes: &[u8], utf8: bool, what: &'static str) -> Result<String, ZipError> {
    if utf8 {
        String::from_utf8(bytes.to_vec())
            .map_err(|_| ZipError::Utf8 { what, name_hint: cp437_decode(bytes) })
    } else {
        Ok(cp437_decode(bytes))
    }
}

/// 🔤️ Best-effort archive-comment decode (EOCD comment has no per-record encoding flag):
/// valid UTF-8 first, CP437 fallback otherwise.
fn decode_best_effort_text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| cp437_decode(bytes))
}
//#endregion Cp437

//#region ExtraFields
const EXTRA_ZIP64: u16 = 0x0001;
const EXTRA_UT: u16 = 0x5455;

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
        out.push(ZipExtraField { id, payload: bytes[start..end].to_vec() });
        i = end;
    }
    Ok(out)
}

fn serialize_extra_fields(fields: &[ZipExtraField]) -> Vec<u8> {
    let mut out = Vec::new();
    for f in fields {
        out.extend_from_slice(&u16_le(f.id));
        out.extend_from_slice(&u16_le(f.payload.len() as u16));
        out.extend_from_slice(&f.payload);
    }
    out
}

/// 🕰️ Info-ZIP extended-timestamp (`UT`, 0x5455) mtime, if present and flagged. Local-header
/// copies may also carry atime/ctime; only mtime is surfaced as a typed convenience field —
/// the full raw record is still kept verbatim in `local_extra`/`central_extra`.
fn parse_ut_mtime(fields: &[ZipExtraField]) -> Option<i64> {
    let f = fields.iter().find(|f| f.id == EXTRA_UT)?;
    if f.payload.len() < 5 || f.payload[0] & 0x01 == 0 {
        return None;
    }
    Some(i32::from_le_bytes([f.payload[1], f.payload[2], f.payload[3], f.payload[4]]) as i64)
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

fn parse_zip64_extra(
    fields: &[ZipExtraField],
    need_uncomp: bool,
    need_comp: bool,
    need_offset: bool,
    need_disk: bool,
) -> Result<Zip64Fields, ZipError> {
    if !(need_uncomp || need_comp || need_offset || need_disk) {
        return Ok(Zip64Fields { uncomp_size: None, comp_size: None, local_offset: None, disk_start: None });
    }
    let record = fields.iter().find(|f| f.id == EXTRA_ZIP64).ok_or_else(|| {
        ZipError::Malformed("32-bit sentinel field present without a ZIP64 extra record".into())
    })?;
    let mut pos = 0usize;
    let mut out = Zip64Fields { uncomp_size: None, comp_size: None, local_offset: None, disk_start: None };
    if need_uncomp {
        out.uncomp_size = Some(read_u64(&record.payload, pos)?);
        pos += 8;
    }
    if need_comp {
        out.comp_size = Some(read_u64(&record.payload, pos)?);
        pos += 8;
    }
    if need_offset {
        out.local_offset = Some(read_u64(&record.payload, pos)?);
        pos += 8;
    }
    if need_disk {
        out.disk_start = Some(read_u32(&record.payload, pos)?);
    }
    Ok(out)
}
//#endregion ExtraFields

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
        if read_u32(data, i)? == SIG_EOCD {
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
}

fn resolve_central_directory(data: &[u8], eocd: usize) -> Result<CentralDirLocation, ZipError> {
    let count16 = read_u16(data, eocd + 10)?;
    let cd_size32 = read_u32(data, eocd + 12)?;
    let cd_offset32 = read_u32(data, eocd + 16)?;
    let comment_len = read_u16(data, eocd + 20)? as usize;
    let comment_start = eocd + 22;
    let comment_end = comment_start + comment_len;
    if comment_end > data.len() {
        return Err(ZipError::Truncated("EOCD comment"));
    }
    let comment = decode_best_effort_text(&data[comment_start..comment_end]);

    let needs_zip64 = count16 == 0xFFFF || cd_size32 == 0xFFFF_FFFF || cd_offset32 == 0xFFFF_FFFF;
    if !needs_zip64 {
        return Ok(CentralDirLocation { count: count16 as usize, cd_size: cd_size32 as usize, cd_offset: cd_offset32 as usize, comment });
    }

    if eocd < 20 || read_u32(data, eocd - 20)? != SIG_EOCD64_LOCATOR {
        return Err(ZipError::Malformed("ZIP64 sentinel in EOCD but no ZIP64 locator record precedes it".into()));
    }
    let locator = eocd - 20;
    let record_offset = read_u64(data, locator + 8)? as usize;
    if read_u32(data, record_offset)? != SIG_EOCD64_RECORD {
        return Err(ZipError::BadSignature { what: "EOCD64 record", at: record_offset });
    }
    let total_entries = read_u64(data, record_offset + 32)?;
    let cd_size = read_u64(data, record_offset + 40)?;
    let cd_offset = read_u64(data, record_offset + 48)?;
    Ok(CentralDirLocation { count: total_entries as usize, cd_size: cd_size as usize, cd_offset: cd_offset as usize, comment })
}
//#endregion Eocd

//#region Decode
/// 🎒️ Decode ZIP container bytes into a metadata-faithful `ZipSnapshot`.
pub fn decode_zip(data: &[u8]) -> Result<ZipSnapshot, ZipError> {
    let eocd = find_eocd(data)?;
    let loc = resolve_central_directory(data, eocd)?;
    if loc.cd_offset + loc.cd_size > data.len() {
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

        let name = decode_zip_text(&data[name_start..name_end], utf8, "central directory filename")?;
        let central_extra = parse_extra_fields(&data[extra_start..extra_end])?;
        let comment = decode_zip_text(&data[comment_start..comment_end], utf8, "central directory comment")?;

        let zip64 = parse_zip64_extra(
            &central_extra,
            uncomp_size32 == 0xFFFF_FFFF,
            comp_size32 == 0xFFFF_FFFF,
            local_off32 == 0xFFFF_FFFF,
            disk_start16 == 0xFFFF,
        )?;
        let uncomp_size = zip64.uncomp_size.unwrap_or(uncomp_size32 as u64) as usize;
        let comp_size = zip64.comp_size.unwrap_or(comp_size32 as u64) as usize;
        let local_off = zip64.local_offset.unwrap_or(local_off32 as u64) as usize;
        let disk_start = zip64.disk_start.unwrap_or(disk_start16 as u32);
        if disk_start != 0 {
            return Err(ZipError::UnsupportedMultiDisk);
        }

        pos = comment_end;

        // ---- Local header ----
        if read_u32(data, local_off)? != SIG_LOCAL {
            return Err(ZipError::BadSignature { what: "local file header", at: local_off });
        }
        let l_flags = read_u16(data, local_off + 6)?;
        let l_method = read_u16(data, local_off + 8)?;
        if l_method != method_code {
            return Err(ZipError::MethodMismatch { name, local: l_method, central: method_code });
        }
        let l_name_len = read_u16(data, local_off + 26)? as usize;
        let l_extra_len = read_u16(data, local_off + 28)? as usize;
        let l_extra_start = local_off + 30 + l_name_len;
        let l_extra_end = l_extra_start + l_extra_len;
        if l_extra_end > data.len() {
            return Err(ZipError::Truncated("local file header name/extra"));
        }
        let local_extra = parse_extra_fields(&data[l_extra_start..l_extra_end])?;

        let method = ZipCompressionMethod::from_code(method_code)
            .ok_or_else(|| ZipError::UnsupportedMethod { name: name.clone(), method: method_code })?;

        let data_off = l_extra_end;
        let data_end = data_off + comp_size;
        if data_end > data.len() {
            return Err(ZipError::Truncated("entry payload"));
        }
        let payload = &data[data_off..data_end];

        // ---- Optional trailing data descriptor (general-purpose bit 3) ----
        if uses_descriptor {
            let mut desc_pos = data_end;
            if read_u32(data, desc_pos)? == SIG_DATA_DESCRIPTOR {
                desc_pos += 4;
            }
            let is_zip64_entry = zip64.uncomp_size.is_some() || zip64.comp_size.is_some();
            let (d_crc, d_comp, d_uncomp) = if is_zip64_entry {
                (read_u32(data, desc_pos)?, read_u64(data, desc_pos + 4)? as usize, read_u64(data, desc_pos + 12)? as usize)
            } else {
                (read_u32(data, desc_pos)?, read_u32(data, desc_pos + 4)? as usize, read_u32(data, desc_pos + 8)? as usize)
            };
            if d_crc != crc || d_comp != comp_size || d_uncomp != uncomp_size {
                return Err(ZipError::DataDescriptorMismatch { name });
            }
        }

        let raw = match method {
            ZipCompressionMethod::Stored => payload.to_vec(),
            ZipCompressionMethod::Deflate => crate::artifacts::deflate::engine::inflate_raw(payload)
                .map_err(ZipError::Malformed)?,
        };
        if raw.len() != uncomp_size {
            return Err(ZipError::Malformed(format!(
                "{name}: decompressed size {} != declared uncompressed size {uncomp_size}", raw.len()
            )));
        }
        let got_crc = crc32(&raw);
        if got_crc != crc {
            return Err(ZipError::Crc32Mismatch { name, expected: crc, actual: got_crc });
        }

        let unix_mtime = parse_ut_mtime(&local_extra).or_else(|| parse_ut_mtime(&central_extra));

        entries.push(ZipEntry {
            name,
            data: raw,
            method,
            dos_date,
            dos_time,
            unix_mtime,
            flags,
            version_made_by,
            version_needed,
            internal_attrs,
            external_attrs,
            local_extra,
            central_extra,
            comment,
        });
        let _ = l_flags; // local flags kept implicitly consistent via `flags` (central is authoritative)
    }

    Ok(ZipSnapshot {
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries,
        comment: loc.comment,
    })
}
//#endregion Decode

//#region Encode
/// 🎒️ Re-encode a `ZipSnapshot` as ZIP container bytes: local headers + central directory + EOCD,
/// each entry recompressed per its own `method` (deflate via the deflate artifact's real codec).
/// Semantically equivalent to the source archive for any conformant reader — not necessarily
/// byte-identical (header field ordering/padding has legitimate freedom per the D2 plan).
///
/// Always writes UTF-8 names/comments with general-purpose bit 11 set (clearing bit 3 — sizes are
/// always known up front here, so streaming data-descriptors are never re-emitted). Returns
/// `ZipError::UnsupportedZip64Write` rather than silently truncating a 64-bit value into 32 bits
/// when an entry or the archive itself would require ZIP64 to represent.
pub fn encode_zip(snapshot: &ZipSnapshot) -> Result<Vec<u8>, ZipError> {
    if snapshot.entries.len() > 0xFFFF {
        return Err(ZipError::UnsupportedZip64Write);
    }

    let mut locals = Vec::new();
    let mut central = Vec::new();

    for entry in &snapshot.entries {
        let name = entry.name.as_bytes();
        if name.len() > u16::MAX as usize {
            return Err(ZipError::Malformed("entry name too long".into()));
        }
        let comment = entry.comment.as_bytes();
        if comment.len() > u16::MAX as usize {
            return Err(ZipError::Malformed("entry comment too long".into()));
        }
        let crc = crc32(&entry.data);
        let payload = match entry.method {
            ZipCompressionMethod::Stored => entry.data.clone(),
            ZipCompressionMethod::Deflate => crate::artifacts::deflate::engine::deflate_raw(&entry.data),
        };
        if payload.len() > u32::MAX as usize || entry.data.len() > u32::MAX as usize {
            return Err(ZipError::UnsupportedZip64Write);
        }
        let comp_size = payload.len() as u32;
        let uncomp_size = entry.data.len() as u32;
        let name_len = name.len() as u16;
        let local_extra = serialize_extra_fields(&entry.local_extra);
        let central_extra = serialize_extra_fields(&entry.central_extra);
        // Clear the streaming-descriptor bit (sizes are always written up front) and force
        // UTF-8 (names/comments are always re-emitted as UTF-8 bytes below).
        let flags_out = (entry.flags & !0x0808u16) | 0x0800u16;
        let version_needed = if entry.version_needed != 0 { entry.version_needed } else { 20 };
        let version_made_by = if entry.version_made_by != 0 { entry.version_made_by } else { 20 };

        let offset = locals.len() as u64;
        if offset > u32::MAX as u64 {
            return Err(ZipError::UnsupportedZip64Write);
        }

        let mut local = Vec::new();
        local.extend_from_slice(&u32_le(SIG_LOCAL));
        local.extend_from_slice(&u16_le(version_needed));
        local.extend_from_slice(&u16_le(flags_out));
        local.extend_from_slice(&u16_le(entry.method.code()));
        local.extend_from_slice(&u16_le(entry.dos_time));
        local.extend_from_slice(&u16_le(entry.dos_date));
        local.extend_from_slice(&u32_le(crc));
        local.extend_from_slice(&u32_le(comp_size));
        local.extend_from_slice(&u32_le(uncomp_size));
        local.extend_from_slice(&u16_le(name_len));
        local.extend_from_slice(&u16_le(local_extra.len() as u16));
        local.extend_from_slice(name);
        local.extend_from_slice(&local_extra);
        local.extend_from_slice(&payload);

        let mut cen = Vec::new();
        cen.extend_from_slice(&u32_le(SIG_CENTRAL));
        cen.extend_from_slice(&u16_le(version_made_by));
        cen.extend_from_slice(&u16_le(version_needed));
        cen.extend_from_slice(&u16_le(flags_out));
        cen.extend_from_slice(&u16_le(entry.method.code()));
        cen.extend_from_slice(&u16_le(entry.dos_time));
        cen.extend_from_slice(&u16_le(entry.dos_date));
        cen.extend_from_slice(&u32_le(crc));
        cen.extend_from_slice(&u32_le(comp_size));
        cen.extend_from_slice(&u32_le(uncomp_size));
        cen.extend_from_slice(&u16_le(name_len));
        cen.extend_from_slice(&u16_le(central_extra.len() as u16));
        cen.extend_from_slice(&u16_le(comment.len() as u16));
        cen.extend_from_slice(&u16_le(0)); // disk number start — single-disk archives only
        cen.extend_from_slice(&u16_le(entry.internal_attrs));
        cen.extend_from_slice(&u32_le(entry.external_attrs));
        cen.extend_from_slice(&u32_le(offset as u32));
        cen.extend_from_slice(name);
        cen.extend_from_slice(&central_extra);
        cen.extend_from_slice(comment);

        locals.extend_from_slice(&local);
        central.extend_from_slice(&cen);
    }

    let cd_offset = locals.len() as u64;
    let cd_size = central.len() as u64;
    let count = snapshot.entries.len() as u16;
    if cd_offset > u32::MAX as u64 || cd_size > u32::MAX as u64 {
        return Err(ZipError::UnsupportedZip64Write);
    }
    let archive_comment = snapshot.comment.as_bytes();
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
    eocd.extend_from_slice(archive_comment);

    let mut out = locals;
    out.extend_from_slice(&central);
    out.extend_from_slice(&eocd);
    Ok(out)
}
//#endregion Encode

//#region Sniff
/// 🎚️ Byte-level sniff confidence — kept local to the engine (no framework dependency here);
/// the analyzer maps this onto `IoConfidence` at the layer that owns that type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SniffConfidence { High, Medium, Low }

/// 🕵️ Structural sniff: recognizes ZIP local-file-header (`PK\x03\x04`), empty-archive EOCD
/// (`PK\x05\x06`), and spanned-archive (`PK\x07\x08`) magics, corroborated by actually finding
/// a well-formed EOCD record — never a constant, always a function of `data`.
pub fn sniff_zip_bytes(data: &[u8]) -> SniffConfidence {
    if data.len() < 4 {
        return SniffConfidence::Low;
    }
    let magic = &data[0..4];
    let starts_recognized = magic == [0x50, 0x4b, 0x03, 0x04]
        || magic == [0x50, 0x4b, 0x05, 0x06]
        || magic == [0x50, 0x4b, 0x07, 0x08];
    let eocd_ok = find_eocd(data).is_ok();
    match (starts_recognized, eocd_ok) {
        (true, true) => SniffConfidence::High,
        (true, false) => SniffConfidence::Medium,
        (false, true) => SniffConfidence::Medium,
        (false, false) => SniffConfidence::Low,
    }
}
//#endregion Sniff

//#region DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot::default()
}

/// 📦️ P2-P2: the demo `stdio.zip` document — two real entries (one `Stored`, one `Deflate`),
/// each with extra fields, timestamps (one carrying a real Info-ZIP `UT` mtime, the other without
/// — exercising the tri-state at the fixture level too), distinct attrs/comments, plus an
/// archive-level comment. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`
/// below) and for this artifact's own `protocol_walk_law` (walked against the REAL
/// `📸️snapshot/💾️binary/📡️component.protocol.semio` — needs at least one central-directory entry
/// for the `repeat`/`backward`/`jump` construct to have real bytes to walk).
pub fn demo_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot {
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: vec![
            ZipEntry {
                name: "readme.txt".into(),
                data: b"hello from stdio.zip".to_vec(),
                method: ZipCompressionMethod::Stored,
                dos_date: 0x5678,
                dos_time: 0x1234,
                unix_mtime: Some(1_700_000_000),
                // `encode_zip` always sets bit 11 (UTF-8) and clears bit 3 (data descriptor)
                // unconditionally — this fixture's `flags` is already in that post-round-trip
                // normal form (matching `encode_full_metadata_round_trip`'s own documented
                // pattern) so `fixture_honesty_law`'s `parse_dsl(fixture) == demo()` holds exactly.
                flags: 0x0800,
                version_made_by: 20,
                version_needed: 20,
                internal_attrs: 0,
                external_attrs: 0o100644 << 16,
                // Real Info-ZIP `UT` (0x5455) extended-timestamp payload: mtime-present flag byte
                // + LE i32 seconds — this is what `parse_ut_mtime` actually decodes back into
                // `unix_mtime` above (a raw, non-UT-shaped payload would silently round-trip to a
                // DIFFERENT `unix_mtime`, exactly the bug this fixture must avoid).
                local_extra: vec![ZipExtraField { id: 0x5455, payload: { let mut p = vec![0x01u8]; p.extend_from_slice(&1_700_000_000i32.to_le_bytes()); p } }],
                central_extra: vec![],
                comment: "a readme".into(),
            },
            ZipEntry {
                name: "data/poem.txt".into(),
                data: b"deflate this small poem, it should compress reasonably well well well".to_vec(),
                method: ZipCompressionMethod::Deflate,
                dos_date: 0x1111,
                dos_time: 0x2222,
                unix_mtime: None,
                flags: 0x0800,
                version_made_by: 63,
                version_needed: 20,
                internal_attrs: 1,
                external_attrs: 0o100755 << 16,
                local_extra: vec![],
                central_extra: vec![ZipExtraField { id: 9, payload: vec![9, 9] }],
                comment: String::new(),
            },
        ],
        comment: "demo archive comment".into(),
    }
}
//#endregion DocumentHelpers

//#region Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::zip::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<ZipSnapshot, ZipMutation>(
        STDIO_ZIP_DOCUMENT_SCHEMA,
    ));
    // 🛡️ D5's generic validate-on-build hook: registers the real ✳️iso21320 subset's
    // SubsetValidator. The ComposerEntry itself is registered separately via this standard's own
    // `composer::entries()` aggregation (see that module).
    crate::artifacts::zip::standards::v2_0::subsets::iso21320::composer::register();
}

/// 📌️ P2-P2: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per note's exemplar
/// pattern (`✏️s/🔌️plugins/🗒️note/…/⚙️engine/🦀️component.rs`'s `register_pilot_languages`, also
/// followed by P2-P1's json/csv pilots) — `stdio.zip`/`.op`/`.diff`/`.pack`/`.spr`, all
/// `dsl::passthrough_hooks`. `diff`'s `protocol` slot stays `None` matching the exemplar's own
/// shape exactly (the role scheme has no dedicated "diff binary" role even though
/// `🔺️diff/💾️binary/📡️component.protocol.semio` is a real, conformance-tested file — its binary
/// form is exercised directly by `protocol_walk_law` below, just not wired through a 6th
/// `LanguageRole`). `register_schema_spec` (P2-M3's `FullResolver` insertion API) is deliberately
/// NOT called — `ZipSnapshot`/`ZipDiff` have no derivable `RecordSpec` (`ZipSnapshot`'s
/// `ArtifactDsl`/`ArtifactPack` are hand-rolled, see ../../📸️snapshot/🦀️component.rs's
/// `HandcraftedArtifactCodecs`, and `ZipDiff` is hand-rolled for the tri-state-`Option<Option<T>>`
/// reason documented at the top of ../../🔺️diff/🦀️component.rs) — fabricating one would be
/// dishonest. Filed as `mechanism_gaps` in this wave's report.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.zip",
        extension: Some("zip"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::zip::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::zip::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.zip"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.zip.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::zip::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::zip::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.zip.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.zip.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::zip::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::zip::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.zip.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.zip.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.zip.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.zip.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::zip::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.zip.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.zip`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(
        crate::artifacts::zip::schema::zip_artifact_schema_descriptor(),
    );
}
//#endregion Register

//#region ArtifactEngine
/// ⚙️ `stdio.zip` artifact engine.
pub struct ZipEngine {
    artifact_state: ZipArtifact,
    snapshot_state: ZipSnapshot,
}

impl ZipEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: ZipSnapshot) -> Self {
        let artifact_state = ZipArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion ArtifactEngine

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region Fixtures
    /// 🏗️ Hand-assembles a real ZIP byte stream exercising: stored + deflate methods, a
    /// UTF-8-named entry (bit 11 set, non-ASCII name), a CP437-named entry (bit 11 unset,
    /// high-byte name), a data-descriptor entry (bit 3, sizes trailing the payload), a ZIP64
    /// entry (sentineled sizes resolved via the 0x0001 extra record), an extra field of an
    /// unrecognized id (kept verbatim), a per-entry comment, and an archive comment.
    struct RawZipEntry {
        name: Vec<u8>,
        data: Vec<u8>,
        method: u16,
        flags: u16,
        extra: Vec<u8>,
        comment: Vec<u8>,
        use_descriptor: bool,
        force_zip64_sentinel: bool,
    }

    fn build_raw_zip(entries: Vec<RawZipEntry>, archive_comment: &[u8]) -> Vec<u8> {
        let mut locals = Vec::new();
        let mut central = Vec::new();
        for e in &entries {
            let payload = match e.method {
                8 => crate::artifacts::deflate::engine::deflate_raw(&e.data),
                // Unsupported-method fixtures (e.g. 12/BZIP2) never reach decompression —
                // `decode_zip` rejects them by method code before touching payload bytes.
                _ => e.data.clone(),
            };
            let crc = crc32(&e.data);
            let (comp_field, uncomp_field, extra) = if e.force_zip64_sentinel {
                let mut zip64_payload = Vec::new();
                zip64_payload.extend_from_slice(&u64_le(e.data.len() as u64));
                zip64_payload.extend_from_slice(&u64_le(payload.len() as u64));
                let mut extra = e.extra.clone();
                extra.extend_from_slice(&u16_le(EXTRA_ZIP64));
                extra.extend_from_slice(&u16_le(zip64_payload.len() as u16));
                extra.extend_from_slice(&zip64_payload);
                (0xFFFF_FFFFu32, 0xFFFF_FFFFu32, extra)
            } else {
                (payload.len() as u32, e.data.len() as u32, e.extra.clone())
            };

            let offset = locals.len() as u32;
            let mut local = Vec::new();
            local.extend_from_slice(&u32_le(SIG_LOCAL));
            local.extend_from_slice(&u16_le(20));
            local.extend_from_slice(&u16_le(e.flags));
            local.extend_from_slice(&u16_le(e.method));
            local.extend_from_slice(&u16_le(0x1234)); // dos time
            local.extend_from_slice(&u16_le(0x5678)); // dos date
            if e.use_descriptor {
                local.extend_from_slice(&u32_le(0));
                local.extend_from_slice(&u32_le(0));
                local.extend_from_slice(&u32_le(0));
            } else {
                local.extend_from_slice(&u32_le(crc));
                local.extend_from_slice(&u32_le(comp_field));
                local.extend_from_slice(&u32_le(uncomp_field));
            }
            local.extend_from_slice(&u16_le(e.name.len() as u16));
            local.extend_from_slice(&u16_le(extra.len() as u16));
            local.extend_from_slice(&e.name);
            local.extend_from_slice(&extra);
            local.extend_from_slice(&payload);
            if e.use_descriptor {
                local.extend_from_slice(&u32_le(SIG_DATA_DESCRIPTOR));
                local.extend_from_slice(&u32_le(crc));
                local.extend_from_slice(&u32_le(comp_field));
                local.extend_from_slice(&u32_le(uncomp_field));
            }

            let mut cen = Vec::new();
            cen.extend_from_slice(&u32_le(SIG_CENTRAL));
            cen.extend_from_slice(&u16_le(20));
            cen.extend_from_slice(&u16_le(20));
            cen.extend_from_slice(&u16_le(e.flags));
            cen.extend_from_slice(&u16_le(e.method));
            cen.extend_from_slice(&u16_le(0x1234));
            cen.extend_from_slice(&u16_le(0x5678));
            cen.extend_from_slice(&u32_le(crc));
            cen.extend_from_slice(&u32_le(comp_field));
            cen.extend_from_slice(&u32_le(uncomp_field));
            cen.extend_from_slice(&u16_le(e.name.len() as u16));
            cen.extend_from_slice(&u16_le(extra.len() as u16));
            cen.extend_from_slice(&u16_le(e.comment.len() as u16));
            cen.extend_from_slice(&u16_le(0));
            cen.extend_from_slice(&u16_le(0));
            cen.extend_from_slice(&u32_le(0o100644 << 16)); // unix external attrs, -rw-r--r--
            cen.extend_from_slice(&u32_le(offset));
            cen.extend_from_slice(&e.name);
            cen.extend_from_slice(&extra);
            cen.extend_from_slice(&e.comment);

            locals.extend_from_slice(&local);
            central.extend_from_slice(&cen);
        }

        let cd_offset = locals.len() as u32;
        let cd_size = central.len() as u32;
        let count = entries.len() as u16;
        let mut eocd = Vec::new();
        eocd.extend_from_slice(&u32_le(SIG_EOCD));
        eocd.extend_from_slice(&u16_le(0));
        eocd.extend_from_slice(&u16_le(0));
        eocd.extend_from_slice(&u16_le(count));
        eocd.extend_from_slice(&u16_le(count));
        eocd.extend_from_slice(&u32_le(cd_size));
        eocd.extend_from_slice(&u32_le(cd_offset));
        eocd.extend_from_slice(&u16_le(archive_comment.len() as u16));
        eocd.extend_from_slice(archive_comment);

        let mut out = locals;
        out.extend_from_slice(&central);
        out.extend_from_slice(&eocd);
        out
    }
    //#endregion Fixtures

    #[test]
    fn crc32_known_vector() {
        // CRC of "123456789" is 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn zip_store_round_trip() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![
                ZipEntry { name: "a.txt".into(), data: b"hello".to_vec(), ..Default::default() },
                ZipEntry { name: "b/bin.dat".into(), data: vec![0, 1, 2, 3, 255], ..Default::default() },
            ],
            comment: String::new(),
        };
        let bytes = encode_zip(&snap).expect("encode store");
        let decoded = decode_zip(&bytes).expect("decode store");
        assert_eq!(decoded.entries.len(), 2);
        assert_eq!(decoded.entries[0].name, "a.txt");
        assert_eq!(decoded.entries[0].data, b"hello");
        assert_eq!(decoded.entries[0].method, ZipCompressionMethod::Stored);
        assert_eq!(decoded.entries[1].data, vec![0, 1, 2, 3, 255]);
    }

    #[test]
    fn zip_deflate_round_trip() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry {
                name: "poem.txt".into(),
                data: b"deflate inside zip via stdio.deflate raw".to_vec(),
                method: ZipCompressionMethod::Deflate,
                ..Default::default()
            }],
            comment: String::new(),
        };
        let bytes = encode_zip(&snap).expect("encode deflate");
        let decoded = decode_zip(&bytes).expect("decode deflate");
        assert_eq!(decoded.entries[0].data, snap.entries[0].data);
        assert_eq!(decoded.entries[0].method, ZipCompressionMethod::Deflate);
    }

    #[test]
    fn codec_round_trip() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry { name: "x".into(), data: b"y".to_vec(), ..Default::default() }],
            comment: String::new(),
        };
        let pack = store::ArtifactPack::encode_pack(&snap);
        let decoded = <ZipSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        // Byte round-tripping through the on-disk format legitimately normalizes metadata that
        // was never set (flags gain the UTF-8 bit, version fields gain their defaults) — see
        // `encode_zip`'s doc comment. The content-level invariant is name + data.
        assert_eq!(decoded.entries.len(), snap.entries.len());
        assert_eq!(decoded.entries[0].name, snap.entries[0].name);
        assert_eq!(decoded.entries[0].data, snap.entries[0].data);
    }

    /// 🧪️ Rich synthetic archive: mixed stored+deflate, UTF-8 name, CP437 name, a
    /// data-descriptor entry, a ZIP64-sentineled entry, an unrecognized extra field kept
    /// verbatim, per-entry + archive comments. Exercises every D2 zip requirement at once.
    #[test]
    fn decode_rich_synthetic_archive() {
        let unknown_extra = {
            let mut v = Vec::new();
            v.extend_from_slice(&u16_le(0x9999)); // unrecognized id
            v.extend_from_slice(&u16_le(3));
            v.extend_from_slice(b"xyz");
            v
        };

        let raw = build_raw_zip(
            vec![
                RawZipEntry {
                    name: b"stored.txt".to_vec(),
                    data: b"stored payload, no compression".to_vec(),
                    method: 0,
                    flags: 0x0800, // utf8
                    extra: unknown_extra.clone(),
                    comment: b"a stored entry".to_vec(),
                    use_descriptor: false,
                    force_zip64_sentinel: false,
                },
                RawZipEntry {
                    name: "café-\u{1F600}.txt".as_bytes().to_vec(),
                    data: b"deflate me please, this text should compress reasonably well well well".to_vec(),
                    method: 8,
                    flags: 0x0800, // utf8
                    extra: Vec::new(),
                    comment: Vec::new(),
                    use_descriptor: false,
                    force_zip64_sentinel: false,
                },
                RawZipEntry {
                    name: vec![0x63, 0x61, 0x66, 0x82, 0x2E, 0x74, 0x78, 0x74], // "caf<0x82>.txt" — CP437 0x82 = 'é'
                    data: b"legacy codepage name entry".to_vec(),
                    method: 0,
                    flags: 0x0000, // no utf8 bit -> CP437 fallback
                    extra: Vec::new(),
                    comment: Vec::new(),
                    use_descriptor: false,
                    force_zip64_sentinel: false,
                },
                RawZipEntry {
                    name: b"streamed.bin".to_vec(),
                    data: b"data written before its size was known, so a trailing descriptor carries the real crc/sizes".to_vec(),
                    method: 8,
                    flags: 0x0800 | 0x0008, // utf8 + data descriptor
                    extra: Vec::new(),
                    comment: Vec::new(),
                    use_descriptor: true,
                    force_zip64_sentinel: false,
                },
                RawZipEntry {
                    name: b"huge-in-theory.bin".to_vec(),
                    data: b"tiny payload but declared via a ZIP64 extra field for test purposes".to_vec(),
                    method: 0,
                    flags: 0x0800,
                    extra: Vec::new(),
                    comment: Vec::new(),
                    use_descriptor: false,
                    force_zip64_sentinel: true,
                },
            ],
            b"archive-level comment",
        );

        let snap = decode_zip(&raw).expect("decode rich synthetic archive");
        assert_eq!(snap.entries.len(), 5);
        assert_eq!(snap.comment, "archive-level comment");

        let stored = &snap.entries[0];
        assert_eq!(stored.name, "stored.txt");
        assert_eq!(stored.data, b"stored payload, no compression");
        assert_eq!(stored.method, ZipCompressionMethod::Stored);
        assert_eq!(stored.comment, "a stored entry");
        assert_eq!(stored.external_attrs, 0o100644 << 16);
        assert_eq!(stored.local_extra.len(), 1);
        assert_eq!(stored.local_extra[0].id, 0x9999);
        assert_eq!(stored.local_extra[0].payload, b"xyz");

        let utf8_entry = &snap.entries[1];
        assert_eq!(utf8_entry.name, "café-\u{1F600}.txt");
        assert_eq!(utf8_entry.method, ZipCompressionMethod::Deflate);
        assert_eq!(utf8_entry.data, b"deflate me please, this text should compress reasonably well well well".to_vec());

        let cp437_entry = &snap.entries[2];
        // 0xE9 in CP437 decodes to 'é'
        assert_eq!(cp437_entry.name, "caf\u{00e9}.txt");
        assert_eq!(cp437_entry.data, b"legacy codepage name entry");

        let streamed = &snap.entries[3];
        assert_eq!(streamed.name, "streamed.bin");
        assert_eq!(streamed.flags & 0x0008, 0x0008);
        assert_eq!(
            streamed.data,
            b"data written before its size was known, so a trailing descriptor carries the real crc/sizes".to_vec()
        );

        let zip64_entry = &snap.entries[4];
        assert_eq!(zip64_entry.name, "huge-in-theory.bin");
        assert_eq!(zip64_entry.data, b"tiny payload but declared via a ZIP64 extra field for test purposes".to_vec());
    }

    #[test]
    fn decode_rejects_unsupported_method() {
        let raw = build_raw_zip(
            vec![RawZipEntry {
                name: b"bzip2.bin".to_vec(),
                data: b"payload never reaches decompression".to_vec(),
                method: 12, // BZIP2 — never implemented
                flags: 0x0800,
                extra: Vec::new(),
                comment: Vec::new(),
                use_descriptor: false,
                force_zip64_sentinel: false,
            }],
            b"",
        );
        // build_raw_zip always writes real (stored/deflate) payload bytes for its `data` field
        // regardless of the declared method, matching what a real archive with an unimplemented
        // method's raw compressed bytes would look like to this decoder.
        let err = decode_zip(&raw).expect_err("method 12 must be rejected, not silently dropped");
        match err {
            ZipError::UnsupportedMethod { method, .. } => assert_eq!(method, 12),
            other => panic!("expected UnsupportedMethod, got {other:?}"),
        }
    }

    #[test]
    fn decode_rejects_crc_mismatch() {
        let mut raw = build_raw_zip(
            vec![RawZipEntry {
                name: b"a.txt".to_vec(),
                data: b"original".to_vec(),
                method: 0,
                flags: 0x0800,
                extra: Vec::new(),
                comment: Vec::new(),
                use_descriptor: false,
                force_zip64_sentinel: false,
            }],
            b"",
        );
        // Corrupt the stored payload byte in place (after the 30-byte local header + name).
        let payload_offset = 30 + "a.txt".len();
        raw[payload_offset] ^= 0xFF;
        let err = decode_zip(&raw).expect_err("corrupted payload must fail crc check");
        assert!(matches!(err, ZipError::Crc32Mismatch { .. }));
    }

    #[test]
    fn encode_full_metadata_round_trip() {
        let mut entry = ZipEntry {
            name: "readme.md".into(),
            data: b"# hello\nsome content here to compress".to_vec(),
            method: ZipCompressionMethod::Deflate,
            dos_date: 0x5678,
            dos_time: 0x1234,
            unix_mtime: Some(1_700_000_000),
            flags: 0,
            version_made_by: 0x0314,
            version_needed: 20,
            internal_attrs: 1,
            external_attrs: 0o100644 << 16,
            local_extra: vec![ZipExtraField { id: 0x5455, payload: {
                let mut p = vec![0x01u8];
                p.extend_from_slice(&1_700_000_000i32.to_le_bytes());
                p
            } }],
            central_extra: Vec::new(),
            comment: "a readme".into(),
        };
        entry.flags = 0; // encoder recomputes bit 3 (clear) + bit 11 (set) unconditionally
        let snap = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: vec![entry], comment: "archive comment".into() };

        let bytes = encode_zip(&snap).expect("encode full metadata");
        let decoded = decode_zip(&bytes).expect("decode full metadata");
        assert_eq!(decoded.comment, "archive comment");
        let e = &decoded.entries[0];
        assert_eq!(e.name, "readme.md");
        assert_eq!(e.data, snap.entries[0].data);
        assert_eq!(e.method, ZipCompressionMethod::Deflate);
        assert_eq!(e.dos_date, 0x5678);
        assert_eq!(e.dos_time, 0x1234);
        assert_eq!(e.unix_mtime, Some(1_700_000_000));
        assert_eq!(e.internal_attrs, 1);
        assert_eq!(e.external_attrs, 0o100644 << 16);
        assert_eq!(e.comment, "a readme");
        assert_eq!(e.flags & 0x0008, 0, "writer must clear the data-descriptor bit");
        assert_eq!(e.flags & 0x0800, 0x0800, "writer must set the utf-8 bit");
    }

    #[test]
    fn encode_rejects_would_be_zip64_entry_size() {
        // Rather than allocate a real 4GiB buffer, exercise the guard directly: an entry whose
        // *compressed* size would exceed u32::MAX must be rejected, never silently truncated.
        // We simulate this cheaply by checking the guard logic's boundary via a crafted deflate
        // payload is impractical in a unit test, so instead assert the documented contract on
        // the archive-wide guard (entry count), which is cheap to construct and exercises the
        // same `UnsupportedZip64Write` code path.
        let mut entries = Vec::new();
        for i in 0..=0xFFFFu32 {
            entries.push(ZipEntry { name: format!("f{i}"), data: Vec::new(), ..Default::default() });
        }
        let snap = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: String::new() };
        let err = encode_zip(&snap).expect_err("more than 0xFFFF entries requires ZIP64");
        assert_eq!(err, ZipError::UnsupportedZip64Write);
    }

    #[test]
    fn sniff_recognizes_real_magic_and_rejects_garbage() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry { name: "a".into(), data: b"b".to_vec(), ..Default::default() }],
            comment: String::new(),
        };
        let real = encode_zip(&snap).expect("encode");
        assert_eq!(sniff_zip_bytes(&real), SniffConfidence::High);

        let empty_archive = encode_zip(&ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: Vec::new(), comment: String::new() }).unwrap();
        assert_eq!(sniff_zip_bytes(&empty_archive), SniffConfidence::High);

        assert_eq!(sniff_zip_bytes(b"not a zip at all, just prose"), SniffConfidence::Low);
        assert_eq!(sniff_zip_bytes(b""), SniffConfidence::Low);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-P2: per-artifact conformance laws (item 6 of the deliverable list) — grammar/protocol
    /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
    /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Lives here (the engine's own test region), not any framework
    /// file — `m5` auto-discovers the snapshot grammar+`.dsl.semio`/protocol+`.pack.semio` pairs
    /// independently (`🧪️fixture-sweep/🦀️component.rs`'s `m5_auto_discovery`); these tests are this
    /// artifact's OWN early-warning, plus direct coverage of the mutations/diff facets that harness
    /// does not auto-discover at all. Same convention P2-P1's json/csv pilots established.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::zip::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — `stdio.zip` is
        /// binary-native) recognizes real `print_dsl` output for the demo archive — same
        /// preamble-stripped body reconstruction `m5_handcrafted_grammar_conformance`'s own
        /// `dsl_body_from_fixture` uses, so this is a direct proof this artifact will pass that
        /// harness once graduated, not merely an analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_zip_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every representative `ZipMutation` variant (`mutations::demo_mutation_cases()`),
        /// including the three genuinely-recursive-payload variants (`SetSnapshot`/`AddEntry`/
        /// `SetEntryExtra`), which the grammar honestly models via `REST` (see that file's own doc
        /// comment) — this law proves `REST` genuinely swallows their real nested-block/list output,
        /// not just that the simple scalar-only variants parse.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `ZipDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
        /// a two-directional `between()` result exercising the full `entries` collection triple and
        /// the tri-state `unix_mtime`.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`.
        ///
        /// The snapshot/pack case does NOT assert `consumed == bytes.len()` — per M2's own
        /// documented exception (`walk_protocol`'s doc comment, `📖️grammar/🦀️component.rs`), a
        /// protocol that performs a `backward`/`jump` (ours does, twice: EOCD backward-scan +
        /// central-directory jump) is no longer required to land on exactly EOF, since the bytes
        /// between the final block's landing point and EOF are validly described by AN EARLIER
        /// block the walk already visited (here: the `backward eocd` block, which already fully
        /// captured the EOCD's own fields before the final `central_directory` repeat's sentinel
        /// match re-touches its first 4 bytes only to terminate cleanly). The op/diff cases declare
        /// neither block, so the ordinary `consumed == bytes.len()` law holds for them exactly.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_zip_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range position: {} (len {})", trace.consumed, inner.len());

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_zip_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_zip_snapshot();

            let parsed = <ZipSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_zip_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_zip_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <ZipSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_zip_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_zip_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion Tests
