//! 📮️ Encode of the zip codec: the member header encoding, the snapshot validation, the ordered archive encoding and the document archive helpers. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
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

pub fn encode_zip_with_entry_names(snapshot: &ZipSnapshot, names: &[String]) -> Result<Vec<u8>, ZipError> {
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
        cen.extend_from_slice(&u16_le(0));
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
    eocd.extend_from_slice(&u16_le(0));
    eocd.extend_from_slice(&u16_le(0));
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
pub fn document_archive_member<S: pack::snapshot::ArtifactDsl>() -> String {
    format!("snapshot.{}.semio", S::EXTENSION)
}

/// 🎒️ A zip 2.0 archive of one artifact document: its canonical DSL text as
/// [`document_archive_member`] (authoritative, read back by [`decode_document_archive`]) and its
/// rfc8259 rendition as `snapshot.json` for readers without a semio parser. Both are lossless, so
/// the hop is `IoFidelity::Exact` — the shared zip carrier every artifact whose own shape is its
/// archive content uses, instead of each owner restating it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_document_archive<S: pack::snapshot::ArtifactDsl + semio_framework_value::ToValue>(document: &S) -> Result<Vec<u8>, ZipError> {
    let entries = vec![
        ZipEntry { name: document_archive_member::<S>(), data: document.print_dsl().into_bytes(), ..Default::default() },
        ZipEntry { name: "snapshot.json".into(), data: semio_framework_pack_json::to_json_string(document).into_bytes(), ..Default::default() },
    ];
    encode_zip(&ZipSnapshot { entries, ..ZipSnapshot::default() })
}

/// 🎒️ The document inside an archive written by [`encode_document_archive`]: its DSL member parsed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_document_archive<S: pack::snapshot::ArtifactDsl>(bytes: &[u8]) -> Result<S, String> {
    let archive = decode_zip(bytes).map_err(|error| error.to_string())?;
    let member = document_archive_member::<S>();
    let entry = archive.entries.iter().find(|entry| entry.name == member).ok_or_else(|| format!("the archive has no {member} member"))?;
    S::parse_dsl(std::str::from_utf8(&entry.data).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}
//#endregion Encode
//#endregion 🦑️DissolvedEngineCodec
