//! 🧬️ DwgSnapshot schema — persistent fields + real codecs. Byte/bit-level decode logic (file
//! header decrypt, R2004+ LZ77-variant decompression, section/page directory walk) lives in
//! `⚙️engine` (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION 🖊️dwg
//! D1-D2 wave); this module owns the typed persisted model and glues `decode_dwg`/`encode_dwg`
//! to it.

use crate::artifacts::dwg::STDIO_DWG_DOCUMENT_SCHEMA;
use crate::artifacts::dwg::standards::v_ac1024::engine as dwg_engine;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️SectionModel
/// 📄️ One page's on-disk footprint plus its decoded content for a named R2004+ section.
/// `decoded` is the raw decompressed (or, for stored/uncompressed sections, verbatim-copied)
/// bytes -- kept uninterpreted (bitcode/header-variable parsing is D3-D4, out of scope for this
/// ticket's D1-D2 bar). Empty `decoded` + non-empty `error` means this specific page's content
/// wasn't recovered; the whole-file `DwgSnapshot.bytes` fallback keeps re-encoding lossless
/// regardless.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSectionPage {
    pub page_number: i32,
    pub file_address: u64,
    pub compressed_size: u32,
    #[serde(default)]
    #[dsl(base64)]
    pub decoded: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 🗂️ One named R2004+ section (`AcDb:Header`, `AcDb:Classes`, ...) as located via the file
/// header's decrypted section-map/section-info directories (D1) and, for `compressed` sections,
/// LZ-decompressed per page (D2). Never authoritative for round-trip encode -- `DwgSnapshot.bytes`
/// (the untouched original file) is always what `encode_dwg` re-emits; this is read-only
/// structural insight layered non-destructively on top.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSection {
    pub name: String,
    #[serde(default)]
    pub compressed: bool,
    #[serde(default)]
    pub declared_size: u64,
    #[serde(default)]
    pub pages: Vec<DwgSectionPage>,
}

/// 🚦️ How far real (non-sentinel) decode reached -- honest per the ticket's D1-D5 phase gates,
/// never silently claims a phase that wasn't actually reached. `bytes` on `DwgSnapshot` holds
/// the complete original file regardless of this status, so re-encode is always lossless.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgDecodeStatus {
    /// Only the 6-byte `AC10xx` version magic was recognized -- pre-R2004 file, or the R2004+
    /// pipeline failed structurally (malformed/truncated/unrecognized variant).
    #[default]
    SentinelOnly,
    /// D1: file header decrypted and every section+page located by name and byte range, but at
    /// least one page's content (D2) failed to decompress (see each page's `error`).
    SectionsLocated,
    /// D2: every located section's every page decompressed (or, for stored sections, copied)
    /// cleanly into `sections[].pages[].decoded`.
    SectionsDecompressed,
}
//#endregion 🔖️SectionModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub version: String,
    /// 🗓️ `maint_version` (RC, plain preamble byte 0x12) — cross-checked against LibreDWG's
    /// `header.spec` (`FIELD_RC (maint_version, 0);` right after `dwg_version` at 0x11) and
    /// verified on the real `architectural.dwg` fixture (byte 0x12 == 0x02).
    #[state(persistent)]
    #[serde(default)]
    pub maintenance_version: u8,
    /// 🌐 `codepage` (RS, plain preamble bytes 0x13-0x14, little-endian) — LibreDWG's
    /// `header.spec` documents this exact offset with `//@0x13: 29/30 for ANSI_1252`; the real
    /// `architectural.dwg` fixture reads `30` there, an exact match.
    #[state(persistent)]
    #[serde(default)]
    pub codepage: u16,
    /// 🔒 The complete, untouched original file bytes -- ALWAYS the lossless source of truth for
    /// `encode_dwg`, regardless of `decode_status`. Nothing this codec doesn't understand is ever
    /// dropped: undecoded content is implicitly retained here even where `sections` below is
    /// empty or partial.
    #[state(persistent)]
    #[serde(default)]
    #[dsl(base64)]
    pub bytes: Vec<u8>,
    /// 🧮 DERIVED from `sections` (never independently diffed -- see the diff module's own doc
    /// comment): kept as a field for back-compat with existing readers, always recomputed by
    /// `decode_dwg`/`derive_section_names` and by every `sections`-mutating diff/mutation.
    #[state(persistent)]
    #[serde(default)]
    pub section_names: Vec<String>,
    /// 🗂️ Real D1/D2 structural decode: every located named section, with page content
    /// decompressed where reachable. Read-only insight layered on top of `bytes` -- never
    /// consulted by `encode_dwg`.
    #[state(persistent)]
    #[serde(default)]
    pub sections: Vec<DwgSection>,
    /// 🧮 DERIVED from `sections` (never independently diffed -- see `derive_decode_status`).
    #[state(persistent)]
    #[serde(default)]
    pub decode_status: DwgDecodeStatus,
}

impl Default for DwgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: String::new(),
            maintenance_version: 0,
            codepage: 0,
            bytes: Vec::new(),
            section_names: Vec::new(),
            sections: Vec::new(),
            decode_status: DwgDecodeStatus::SentinelOnly,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DwgCodec
fn dwg_version_sentinel(bytes: &[u8]) -> Result<String, String> {
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
/// (unencrypted) file-header preamble shared by every AC1015+ DWG file, per LibreDWG's own
/// `header.spec` field order (`zero_one_or_three@0x0B`, `thumbnail_address@0x0D`,
/// `dwg_version@0x11`, `maint_version@0x12`, `codepage@0x13`). Graceful zero-defaults when
/// `bytes` is too short to reach these offsets.
fn parse_version_header_fields(bytes: &[u8]) -> (u8, u16) {
    let maintenance_version = bytes.get(0x12).copied().unwrap_or(0);
    let codepage = bytes
        .get(0x13..0x15)
        .map(|s| u16::from_le_bytes([s[0], s[1]]))
        .unwrap_or(0);
    (maintenance_version, codepage)
}

/// 🧮 `section_names` is fully DERIVED from `sections` -- the single place this projection is
/// computed, reused by `decode_dwg`, `DwgDiff::apply`, and every section-mutating
/// `apply_dwg_mutation` arm so the two fields can never drift out of sync.
pub fn derive_section_names(sections: &[DwgSection]) -> Vec<String> {
    sections.iter().map(|s| s.name.clone()).filter(|n| !n.is_empty()).collect()
}

/// 🧮 `decode_status` is fully DERIVED from `sections` -- honest per the D1-D5 phase gates
/// (`DwgDecodeStatus` docs): empty -> `SentinelOnly`; every page error-free ->
/// `SectionsDecompressed`; otherwise -> `SectionsLocated`.
pub fn derive_decode_status(sections: &[DwgSection]) -> DwgDecodeStatus {
    if sections.is_empty() {
        DwgDecodeStatus::SentinelOnly
    } else if sections.iter().all(|s| s.pages.iter().all(|p| p.error.is_none())) {
        DwgDecodeStatus::SectionsDecompressed
    } else {
        DwgDecodeStatus::SectionsLocated
    }
}

/// 🗺️ Runs the real R2004+ engine pipeline (D1 location + D2 decompression) and converts its raw
/// output into the typed schema model. Any structural failure (wrong magic, truncated header,
/// checksum-verified-wrong decrypt) falls back to an empty `sections` list -- never a
/// fabricated/garbage partial result. `bytes` is always retained by the caller regardless, so
/// this fallback is always lossless.
fn decode_sections(bytes: &[u8]) -> Vec<DwgSection> {
    let Ok(raw_sections) = dwg_engine::decode_r2004_sections(bytes) else {
        return Vec::new();
    };
    raw_sections
        .into_iter()
        .map(|r| DwgSection {
            name: r.name,
            compressed: r.compressed,
            declared_size: r.declared_size,
            pages: r
                .pages
                .into_iter()
                .map(|p| DwgSectionPage {
                    page_number: p.page_number,
                    file_address: p.file_address,
                    compressed_size: p.compressed_size,
                    decoded: p.decoded,
                    error: p.error,
                })
                .collect(),
        })
        .collect()
}

pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes);
    let sections = decode_sections(bytes);
    let section_names = derive_section_names(&sections);
    let decode_status = derive_decode_status(&sections);
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        maintenance_version,
        codepage,
        bytes: bytes.to_vec(),
        section_names,
        sections,
        decode_status,
    })
}

pub fn encode_dwg(snap: &DwgSnapshot) -> Result<Vec<u8>, String> {
    if snap.bytes.is_empty() {
        return Err("empty DWG payload".into());
    }
    dwg_version_sentinel(&snap.bytes)?;
    if snap.version.is_empty() {
        return Err("missing DWG version".into());
    }
    if snap.bytes.get(0..6) != Some(snap.version.as_bytes()) {
        return Err("version field disagrees with AC10xx header".into());
    }
    Ok(snap.bytes.clone())
}
//#endregion 🔖️DwgCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DwgSnapshot {
    const EXTENSION: &'static str = "dwg";
    fn envelope_id() -> &'static str { "stdio.dwg" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i + 1 < hex.len() {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
            i += 2;
        }
        decode_dwg(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let raw = encode_dwg(self).unwrap_or_default();
        let body: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DwgSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_dwg(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_dwg(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
