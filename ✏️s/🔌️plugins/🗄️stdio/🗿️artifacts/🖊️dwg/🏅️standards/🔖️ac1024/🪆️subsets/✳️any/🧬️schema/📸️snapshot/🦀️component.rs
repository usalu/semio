//! 🧬️ DwgSnapshot schema — persistent fields + real codecs. Byte/bit-level decode logic (file
//! header decrypt, R2004+ LZ77-variant decompression, section/page directory walk) lives in
//! `⚙️engine` (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION 🖊️dwg
//! D1-D2 wave); this module owns the typed persisted model and glues `decode_dwg`/`encode_dwg`
//! to it.

use crate::artifacts::dwg::STDIO_DWG_DOCUMENT_SCHEMA;
use crate::artifacts::dwg::standards::v_ac1024::engine as dwg_engine;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgLogicalGeometryKind {
    #[default]
    Point,
    Line,
    Circle,
    Arc,
    Ellipse,
    LwPolyline,
    Spline,
    Text,
    Face3d,
    Polyline3d,
    PolyfaceMesh,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalGeometry {
    pub kind: DwgLogicalGeometryKind,
    #[serde(default)]
    pub values: Vec<f64>,
    #[serde(default)]
    pub indices: Vec<i32>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub closed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalLayer {
    pub name: String,
    pub color: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalEntity {
    pub layer: usize,
    pub color: i16,
    pub geometry: DwgLogicalGeometry,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalDrawing {
    #[serde(default)]
    pub layers: Vec<DwgLogicalLayer>,
    #[serde(default)]
    pub entities: Vec<DwgLogicalEntity>,
    #[serde(default)]
    pub extmin: Vec<f64>,
    #[serde(default)]
    pub extmax: Vec<f64>,
}

impl DwgLogicalDrawing {
    pub fn from_native(drawing: &dwg_engine::DwgDrawing) -> Self {
        Self {
            layers: drawing.layers.iter().map(|layer| DwgLogicalLayer { name: layer.name.clone(), color: layer.color }).collect(),
            entities: drawing.entities.iter().map(DwgLogicalEntity::from_native).collect(),
            extmin: drawing.extmin.to_vec(),
            extmax: drawing.extmax.to_vec(),
        }
    }

    pub fn to_native(&self) -> Result<dwg_engine::DwgDrawing, String> {
        Ok(dwg_engine::DwgDrawing {
            layers: self.layers.iter().map(|layer| dwg_engine::DwgLayer { name: layer.name.clone(), color: layer.color }).collect(),
            entities: self.entities.iter().map(DwgLogicalEntity::to_native).collect::<Result<_, _>>()?,
            extmin: vec3(&self.extmin)?,
            extmax: vec3(&self.extmax)?,
        })
    }
}

impl DwgLogicalEntity {
    fn from_native(entity: &dwg_engine::DwgEntity) -> Self {
        let color = match entity.color {
            dwg_engine::DwgColor::ByLayer => -1,
            dwg_engine::DwgColor::ByBlock => -2,
            dwg_engine::DwgColor::Index(value) => value as i16,
        };
        Self { layer: entity.layer, color, geometry: DwgLogicalGeometry::from_native(&entity.geometry) }
    }

    fn to_native(&self) -> Result<dwg_engine::DwgEntity, String> {
        let color = match self.color {
            -1 => dwg_engine::DwgColor::ByLayer,
            -2 => dwg_engine::DwgColor::ByBlock,
            value if (0..=255).contains(&value) => dwg_engine::DwgColor::Index(value as u8),
            value => return Err(format!("invalid logical DWG color {value}")),
        };
        Ok(dwg_engine::DwgEntity { layer: self.layer, color, geometry: self.geometry.to_native()? })
    }
}

fn vec2(values: &[f64]) -> Result<[f64; 2], String> {
    values.try_into().map_err(|_| format!("expected 2 values, got {}", values.len()))
}

fn vec3(values: &[f64]) -> Result<[f64; 3], String> {
    values.try_into().map_err(|_| format!("expected 3 values, got {}", values.len()))
}

impl DwgLogicalGeometry {
    fn from_native(geometry: &dwg_engine::DwgGeometry) -> Self {
        use dwg_engine::DwgGeometry::*;
        match geometry {
            Point { at } => Self { kind: DwgLogicalGeometryKind::Point, values: at.to_vec(), ..Default::default() },
            Line { start, end } => Self { kind: DwgLogicalGeometryKind::Line, values: start.iter().chain(end).copied().collect(), ..Default::default() },
            Circle { center, radius, normal } => Self { kind: DwgLogicalGeometryKind::Circle, values: center.iter().chain([radius]).chain(normal).copied().collect(), ..Default::default() },
            Arc { center, radius, start_angle, end_angle, normal } => Self { kind: DwgLogicalGeometryKind::Arc, values: center.iter().chain([radius, start_angle, end_angle]).chain(normal).copied().collect(), ..Default::default() },
            Ellipse { center, major_axis, ratio, start_param, end_param, normal } => Self { kind: DwgLogicalGeometryKind::Ellipse, values: center.iter().chain(major_axis).chain([ratio, start_param, end_param]).chain(normal).copied().collect(), ..Default::default() },
            LwPolyline { closed, elevation, vertices, bulges } => Self { kind: DwgLogicalGeometryKind::LwPolyline, values: std::iter::once(*elevation).chain(vertices.iter().flatten().copied()).chain(bulges.iter().copied()).collect(), indices: vec![vertices.len() as i32], closed: *closed, ..Default::default() },
            Spline { degree, control_points, knots, weights } => Self { kind: DwgLogicalGeometryKind::Spline, values: control_points.iter().flatten().copied().chain(knots.iter().copied()).chain(weights.iter().copied()).collect(), indices: vec![*degree as i32, control_points.len() as i32, knots.len() as i32], ..Default::default() },
            Text { at, height, rotation, content } => Self { kind: DwgLogicalGeometryKind::Text, values: at.iter().copied().chain([*height, *rotation]).collect(), text: content.clone(), ..Default::default() },
            Face3d { corners } => Self { kind: DwgLogicalGeometryKind::Face3d, values: corners.iter().flatten().copied().collect(), ..Default::default() },
            Polyline3d { closed, vertices } => Self { kind: DwgLogicalGeometryKind::Polyline3d, values: vertices.iter().flatten().copied().collect(), closed: *closed, ..Default::default() },
            PolyfaceMesh { vertices, faces } => Self { kind: DwgLogicalGeometryKind::PolyfaceMesh, values: vertices.iter().flatten().copied().collect(), indices: std::iter::once(vertices.len() as i32).chain(faces.iter().flatten().copied()).collect(), ..Default::default() },
        }
    }

    fn to_native(&self) -> Result<dwg_engine::DwgGeometry, String> {
        use DwgLogicalGeometryKind::*;
        Ok(match self.kind {
            Point => dwg_engine::DwgGeometry::Point { at: vec3(&self.values)? },
            Line => dwg_engine::DwgGeometry::Line { start: vec3(&self.values[0..3])?, end: vec3(&self.values[3..6])? },
            Circle => dwg_engine::DwgGeometry::Circle { center: vec3(&self.values[0..3])?, radius: self.values[3], normal: vec3(&self.values[4..7])? },
            Arc => dwg_engine::DwgGeometry::Arc { center: vec3(&self.values[0..3])?, radius: self.values[3], start_angle: self.values[4], end_angle: self.values[5], normal: vec3(&self.values[6..9])? },
            Ellipse => dwg_engine::DwgGeometry::Ellipse { center: vec3(&self.values[0..3])?, major_axis: vec3(&self.values[3..6])?, ratio: self.values[6], start_param: self.values[7], end_param: self.values[8], normal: vec3(&self.values[9..12])? },
            LwPolyline => {
                let count = *self.indices.first().ok_or("polyline vertex count missing")? as usize;
                let vertices = self.values[1..1 + count * 2].chunks_exact(2).map(vec2).collect::<Result<_, _>>()?;
                dwg_engine::DwgGeometry::LwPolyline { closed: self.closed, elevation: self.values[0], vertices, bulges: self.values[1 + count * 2..].to_vec() }
            }
            Spline => {
                let degree = self.indices[0] as u32;
                let point_count = self.indices[1] as usize;
                let knot_count = self.indices[2] as usize;
                let point_end = point_count * 3;
                let control_points = self.values[..point_end].chunks_exact(3).map(vec3).collect::<Result<_, _>>()?;
                dwg_engine::DwgGeometry::Spline { degree, control_points, knots: self.values[point_end..point_end + knot_count].to_vec(), weights: self.values[point_end + knot_count..].to_vec() }
            }
            Text => dwg_engine::DwgGeometry::Text { at: vec3(&self.values[0..3])?, height: self.values[3], rotation: self.values[4], content: self.text.clone() },
            Face3d => dwg_engine::DwgGeometry::Face3d { corners: [vec3(&self.values[0..3])?, vec3(&self.values[3..6])?, vec3(&self.values[6..9])?, vec3(&self.values[9..12])?] },
            Polyline3d => dwg_engine::DwgGeometry::Polyline3d { closed: self.closed, vertices: self.values.chunks_exact(3).map(vec3).collect::<Result<_, _>>()? },
            PolyfaceMesh => {
                let vertex_count = self.indices[0] as usize;
                let vertices = self.values.chunks_exact(3).take(vertex_count).map(vec3).collect::<Result<_, _>>()?;
                let faces = self.indices[1..].chunks_exact(4).map(|face| face.try_into().unwrap()).collect();
                dwg_engine::DwgGeometry::PolyfaceMesh { vertices, faces }
            }
        })
    }
}
//#endregion 🔖️DrawingModel

//#region 🔖️SectionModel
/// 📄️ One logical page of decoded named-section content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSectionPage {
    pub page_number: i32,
    pub start_offset: u64,
    pub decompressed_size: u32,
    #[serde(default)]
    #[dsl(base64)]
    pub decoded: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 🗂️ One named R2004+ section (`AcDb:Header`, `AcDb:Classes`, ...) as located via the file
/// header's decrypted section-map/section-info directories (D1) and, for `compressed` sections,
/// LZ-decompressed per page (D2) and deterministically rematerialized by the AC1024 writer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSection {
    pub name: String,
    #[serde(default)]
    pub compressed: bool,
    #[serde(default)]
    pub declared_size: u64,
    #[serde(default)]
    pub max_decompressed_page_size: u32,
    #[serde(default)]
    pub section_id: u32,
    #[serde(default)]
    pub encrypted: u32,
    #[serde(default)]
    pub pages: Vec<DwgSectionPage>,
}

/// 🚦️ How far real (non-sentinel) decode reached -- honest per the ticket's D1-D5 phase gates,
/// never silently claims a phase that wasn't actually reached.
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
//#region 🧱️PhysicalLayout
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPreamble {
    #[dsl(base64)] pub reserved_06_0a: Vec<u8>,
    pub zero_one_or_three: u8, pub reserved_0c: u8, pub thumbnail_address: u32, pub drawing_version: u8,
    #[dsl(base64)] pub reserved_15_27: Vec<u8>,
    pub encrypted_header_address: u32,
    #[dsl(base64)] pub reserved_2c_7f: Vec<u8>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgFileHeader {
    #[dsl(base64)] pub file_id: Vec<u8>, #[dsl(base64)] pub reserved_0c_2b: Vec<u8>,
    pub last_section_address: u64, pub second_header_address: u64, pub num_gaps: u32, pub num_sections: u32,
    #[dsl(base64)] pub reserved_44_4f: Vec<u8>,
    pub section_map_id: i32, pub section_map_address: u64, pub section_info_id: i32,
    pub section_array_size: u32, pub reserved_64_67: u32, pub crc32: u32,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgGapTree { pub parent: i32, pub left: i32, pub right: i32, pub zero: i32 }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPageDirectoryEntry { pub number: i32, pub allocation_size: u32, pub gap_tree: Option<DwgGapTree> }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgPhysicalPageKind { Data, SectionInfo, PageMap, Gap, #[default] Unknown }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDataPageHeader { pub page_type: u32, pub section_id: u32, pub data_size: u32, pub page_size: u32, pub start_offset: u32, pub header_checksum: u32, pub data_checksum: u32, pub unknown: u32 }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSystemPageHeader { pub page_type: u32, pub decompressed_size: u32, pub compressed_size: u32, pub compression_type: u32, pub checksum: u32 }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPhysicalPage {
    pub number: i32, pub address: u64, pub allocation_size: u32, pub kind: DwgPhysicalPageKind,
    pub data_header: Option<DwgDataPageHeader>, pub system_header: Option<DwgSystemPageHeader>,
    #[dsl(base64)] pub opaque_payload: Vec<u8>, #[dsl(base64)] pub trailing_bytes: Vec<u8>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgTrailer { pub address: u64, pub second_header: Option<DwgFileHeader>, #[dsl(base64)] pub unknown_suffix: Vec<u8> }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPhysicalLayout {
    pub preamble: DwgPreamble, pub encrypted_header: DwgFileHeader,
    #[dsl(base64)] pub encrypted_header_padding: Vec<u8>, pub page_directory: Vec<DwgPageDirectoryEntry>,
    #[dsl(base64)] pub page_directory_trailing: Vec<u8>, pub pages: Vec<DwgPhysicalPage>, pub trailer: DwgTrailer,
}
//#endregion 🧱️PhysicalLayout

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub version: String,
    /// 🗓️ `maint_version` (RC, plain preamble byte 0x12) — cross-checked against LibreDWG's
    /// `header.spec` (`FIELD_RC (maint_version, 0);` right after `dwg_version` at 0x11) and
    /// verified on the real `architectural.dwg` fixture (byte 0x12 == 0x02).
    #[state(artifact)]
    #[serde(default)]
    pub maintenance_version: u8,
    /// 🌐 `codepage` (RS, plain preamble bytes 0x13-0x14, little-endian) — LibreDWG's
    /// `header.spec` documents this exact offset with `//@0x13: 29/30 for ANSI_1252`; the real
    /// `architectural.dwg` fixture reads `30` there, an exact match.
    #[state(artifact)]
    #[serde(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[serde(default)]
    pub drawing: DwgLogicalDrawing,
    /// 🧮 DERIVED from `sections` (never independently diffed -- see the diff module's own doc
    /// comment): kept as a field for back-compat with existing readers, always recomputed by
    /// `decode_dwg`/`derive_section_names` and by every `sections`-mutating diff/mutation.
    #[state(artifact)]
    #[serde(default)]
    pub section_names: Vec<String>,
    /// 🗂️ Real D1/D2 structural decode: every located named section, with logical page content
    /// decompressed where reachable.
    #[state(artifact)]
    #[serde(default)]
    pub sections: Vec<DwgSection>,
    /// 🧮 DERIVED from `sections` (never independently diffed -- see `derive_decode_status`).
    #[state(artifact)]
    #[serde(default)]
    pub decode_status: DwgDecodeStatus,
    #[state(artifact)]
    #[serde(default)]
    pub physical: DwgPhysicalLayout,
}

impl Default for DwgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: String::new(),
            maintenance_version: 0,
            codepage: 0,
            drawing: DwgLogicalDrawing::default(),
            section_names: Vec::new(),
            sections: Vec::new(),
            decode_status: DwgDecodeStatus::SentinelOnly,
            physical: DwgPhysicalLayout::default(),
        }
    }
}

impl DwgSnapshot {
    /// 🪞️ Clones the deterministic logical projection.
    pub fn projection(&self) -> Self {
        self.clone()
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
/// fabricated/garbage partial result.
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
                    start_offset: p.start_offset,
                    decompressed_size: p.decompressed_size,
                    decoded: p.decoded,
                    error: p.error,
                })
                .collect(),
            max_decompressed_page_size: r.max_decomp_size,
            section_id: r.section_id,
            encrypted: r.encrypted,
        })
        .collect()
}

pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes);
    let sections = decode_sections(bytes);
    let section_names = derive_section_names(&sections);
    let decode_status = derive_decode_status(&sections);
    let drawing = dwg_engine::dwg_from_bytes(bytes).ok().map(|value| DwgLogicalDrawing::from_native(&value)).unwrap_or_default();
    let physical = dwg_engine::decode_r2004_physical(bytes)?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        maintenance_version,
        codepage,
        drawing,
        section_names,
        sections,
        decode_status,
        physical,
    })
}

/// 🚫 Typed DWG export failures distinguish invalid logical state from writer failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DwgExportError {
    InvalidLogical(String),
    InvalidVersion(String),
    HeaderMismatch(String),
    Writer(String),
}

impl fmt::Display for DwgExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLogical(message) => write!(formatter, "invalid logical DWG: {message}"),
            Self::InvalidVersion(message) => write!(formatter, "invalid DWG version: {message}"),
            Self::HeaderMismatch(message) => write!(formatter, "DWG header mismatch: {message}"),
            Self::Writer(message) => write!(formatter, "DWG writer failed: {message}"),
        }
    }
}

impl std::error::Error for DwgExportError {}

fn validate_export_header(bytes: &[u8], snapshot: &DwgSnapshot) -> Result<(), DwgExportError> {
    let version = dwg_version_sentinel(bytes).map_err(DwgExportError::Writer)?;
    if snapshot.version.len() != 6 {
        return Err(DwgExportError::InvalidVersion("AC10xx sentinel must contain six ASCII bytes".into()));
    }
    if version != snapshot.version {
        return Err(DwgExportError::HeaderMismatch(format!("version {} != {}", version, snapshot.version)));
    }
    let (maintenance_version, codepage) = parse_version_header_fields(bytes);
    if maintenance_version != snapshot.maintenance_version {
        return Err(DwgExportError::HeaderMismatch(format!(
            "maintenance version {maintenance_version} != {}",
            snapshot.maintenance_version
        )));
    }
    if codepage != snapshot.codepage {
        return Err(DwgExportError::HeaderMismatch(format!("codepage {codepage} != {}", snapshot.codepage)));
    }
    Ok(())
}

/// 🔄 Updates supported typed header fields.
pub fn synchronize_version_info(
    snapshot: &mut DwgSnapshot,
    version: &str,
    maintenance_version: u8,
    codepage: u16,
) -> Result<(), DwgExportError> {
    dwg_version_sentinel(version.as_bytes()).map_err(DwgExportError::InvalidVersion)?;
    snapshot.version = version.to_string();
    snapshot.maintenance_version = maintenance_version;
    snapshot.codepage = codepage;
    Ok(())
}

pub fn encode_dwg(snapshot: &DwgSnapshot) -> Result<Vec<u8>, DwgExportError> {
    if snapshot.schema != STDIO_DWG_DOCUMENT_SCHEMA {
        return Err(DwgExportError::InvalidLogical("schema identity changed".into()));
    }
    let bytes = if snapshot.sections.is_empty() && (!snapshot.drawing.layers.is_empty() || !snapshot.drawing.entities.is_empty()) {
        dwg_engine::dwg_to_bytes(&snapshot.drawing.to_native().map_err(DwgExportError::InvalidLogical)?)
            .map_err(DwgExportError::Writer)?
    } else {
        dwg_engine::encode_r2004_snapshot(snapshot).map_err(DwgExportError::Writer)?
    };
    validate_export_header(&bytes, snapshot)?;
    Ok(bytes)
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
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
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
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
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
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
