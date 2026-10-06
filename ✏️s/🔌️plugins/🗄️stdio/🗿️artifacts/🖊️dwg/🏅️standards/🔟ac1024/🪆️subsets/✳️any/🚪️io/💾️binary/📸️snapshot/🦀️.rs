//! dwg rep for stdio.dwg 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use crate::standards::v_ac1024::engine as dwg_engine;
use crate::STDIO_DWG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use std::fmt;
use semio_framework_value::{ValueError, ValueRefusalKind};

impl store::ArtifactPack for DwgSnapshot {
    /// 🪶️ Publishes this owner's complete authored relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use crate::standards::v_ac1024::engine as dwg_engine;
use crate::STDIO_DWG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use std::fmt;
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::io::text::snapshot::*;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dwg_version_sentinel(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() < 6 {
        return Err("DWG too short for AC10xx header".into());
    }
    let head = &bytes[0..6];
    if head[0] != b'A' || head[1] != b'C' || !head[2].is_ascii_digit() || !head[3].is_ascii_digit() {
        return Err("missing AC10xx DWG version sentinel".into());
    }
    if !head[4].is_ascii_digit() || !head[5].is_ascii_digit() {
        return Err("invalid AC10xx version digits".into());
    }
    Ok(String::from_utf8_lossy(head).into_owned())
}
/// 🗓️🌐 Reads `maint_version` (offset 0x12) and `codepage` (offset 0x13-0x14 LE) from the plain
/// file-header preamble shared by every AC1015+ DWG file, per LibreDWG's own
/// `header.spec` field order (`zero_one_or_three@0x0B`, `thumbnail_address@0x0D`,
/// `dwg_version@0x11`, `maint_version@0x12`, `codepage@0x13`). Truncated headers are rejected.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_version_header_fields(bytes: &[u8]) -> Result<(u8, u16), String> {
    let maintenance_version = *bytes.get(0x12).ok_or("DWG header is too short for maintenance version")?;
    let codepage = bytes.get(0x13..0x15).ok_or("DWG header is too short for codepage")?;
    Ok((maintenance_version, u16::from_le_bytes([codepage[0], codepage[1]])))
}
/// 🫙️ Whether `snapshot` carries the empty document — the preamble triple and nothing else. Compared
/// against a freshly defaulted snapshot wearing the same triple rather than by inspecting fields one
/// at a time, so a field added to `DwgSnapshot` later cannot quietly fall out of the question.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn is_preamble_only_document(snapshot: &DwgSnapshot) -> bool {
    let bare = DwgSnapshot { schema: snapshot.schema.clone(), version: snapshot.version.clone(), maintenance_version: snapshot.maintenance_version, codepage: snapshot.codepage, ..DwgSnapshot::default() };
    *snapshot == bare
}
/// 🖨️ The empty document's bytes: the preamble region, version stamp and preamble triple written in,
/// every other byte zero — byte for byte what the committed demo example already is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_preamble_only_document(snapshot: &DwgSnapshot) -> Vec<u8> {
    let mut bytes = vec![0u8; DWG_PREAMBLE_LEN];
    bytes[0..6].copy_from_slice(snapshot.version.as_bytes());
    bytes[0x12] = snapshot.maintenance_version;
    bytes[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    bytes
}
/// 🗺️ Materializes section pages only while deserializing and projects their standard objects.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_drawing(bytes: &[u8]) -> Result<DwgLogicalDrawing, String> {
    DwgLogicalDrawing::from_native(&dwg_engine::dwg_from_bytes(bytes)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes)?;
    // 🫙️ The empty document. A stream that stops at the end of the preamble carries no section map,
    // no page map and no encrypted file header, so there is nothing behind it to decode — and the
    // R2004 reader's "file too short for encrypted header" is the wrong answer to give about a
    // document this artifact itself writes and commits as an example.
    if bytes.len() == DWG_PREAMBLE_LEN {
        return Ok(DwgSnapshot { schema: STDIO_DWG_DOCUMENT_SCHEMA.into(), version, maintenance_version, codepage, ..Default::default() });
    }
    if version != "AC1024" {
        return DwgSnapshot::from_drawing(&dwg_engine::dwg_from_bytes(bytes)?);
    }
    let mut drawing = decode_drawing(bytes)?;
    let classes = dwg_engine::decode_r2004_classes(bytes)?;
    drawing.objects = dwg_engine::decode_r2004_object_identities(bytes, &classes)?;
    let document = dwg_engine::decode_r2004_document_sections(bytes)?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        maintenance_version,
        codepage,
        drawing,
        header: document.header,
        classes,
        dependencies: document.dependencies,
        summary: document.summary,
        application: document.application,
        template: document.template,
        auxiliary_header: document.auxiliary_header,
        revision_history: document.revision_history,
        preview: document.preview,
        application_history: document.application_history,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn validate_export_header(bytes: &[u8], snapshot: &DwgSnapshot) -> Result<(), DwgExportError> {
    let version = dwg_version_sentinel(bytes).map_err(DwgExportError::Writer)?;
    if snapshot.version.len() != 6 {
        return Err(DwgExportError::InvalidVersion("AC10xx sentinel must contain six ASCII bytes".into()));
    }
    if version != snapshot.version {
        return Err(DwgExportError::HeaderMismatch(format!("version {} != {}", version, snapshot.version)));
    }
    let (maintenance_version, codepage) = parse_version_header_fields(bytes).map_err(DwgExportError::Writer)?;
    if maintenance_version != snapshot.maintenance_version {
        return Err(DwgExportError::HeaderMismatch(format!("maintenance version {maintenance_version} != {}", snapshot.maintenance_version)));
    }
    if codepage != snapshot.codepage {
        return Err(DwgExportError::HeaderMismatch(format!("codepage {codepage} != {}", snapshot.codepage)));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_dwg(snapshot: &DwgSnapshot) -> Result<Vec<u8>, DwgExportError> {
    if snapshot.schema != STDIO_DWG_DOCUMENT_SCHEMA {
        return Err(DwgExportError::InvalidLogical("schema identity changed".into()));
    }
    if let Some((_, message)) = unwritable_version(snapshot) {
        return Err(DwgExportError::InvalidVersion(message));
    }
    // 🫙️ The empty document is written as the preamble and nothing else — the inverse of the read
    // above, and the only representation an R2004 container HAS for a snapshot that carries no
    // Header, no Classes and no object sections to build one from.
    let bytes = if is_preamble_only_document(snapshot) { encode_preamble_only_document(snapshot) } else { dwg_engine::encode_r2004_snapshot(snapshot).map_err(DwgExportError::Writer)? };
    validate_export_header(&bytes, snapshot)?;
    Ok(bytes)
}
}
pub use native_snapshot_codec::*;
