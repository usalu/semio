//! 🔍️ Inspect, decode and sniff of the zip codec: the central entry header inspection, the end of central directory lookup, the archive decode and the byte sniff. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
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
pub const SIG_LOCAL: u32 = 0x0403_4b50;

pub const SIG_CENTRAL: u32 = 0x0201_4b50;

pub const SIG_EOCD: u32 = 0x0605_4b50;

const SIG_EOCD64_LOCATOR: u32 = 0x0706_4b50;

const SIG_EOCD64_RECORD: u32 = 0x0606_4b50;

pub const SIG_DATA_DESCRIPTOR: u32 = 0x0807_4b50;

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
    pub(super) count: usize,
    pub(super) cd_size: usize,
    pub(super) cd_offset: usize,
    pub(super) comment: String,
    pub(super) comment_utf8: bool,
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
//#endregion 🦑️DissolvedEngineCodec
