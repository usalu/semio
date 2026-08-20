//! 🧬️ LasSnapshot schema — full LAS 1.0 public header block + VLRs + point records. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the earlier
//! "Weak" tier (`{schema, points}` only, no header/VLRs) with the recipe's full-completeness
//! model: a typed `LasHeader` (every real §LAS 1.0 public header block field the ticket lists),
//! an index-keyed `vlrs: Vec<LasVlr>` (payload verbatim — VLR content is proprietary/unmodeled
//! per-registered-id, the recipe's typed raw-retention exception), and the existing index-keyed
//! `points: Vec<LasPoint>`.

use crate::artifacts::las::STDIO_LAS_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Header
/// 📋 The LAS 1.0 public header block, minus the fixed 4-byte "LASF" signature (checked, never
/// stored — an identity constant, not diffable content) and the file-source-id/global-encoding/
/// project-id-GUID fields (spec-real but out of this wave's contracted field list; deviation
/// noted in the closing report). `header_size`/`offset_to_point_data`/`number_of_vlrs`/
/// `point_data_format_id`/`point_data_record_length`/`number_of_point_records` are STRUCTURAL —
/// `engine::encode_las` always recomputes them from the real `vlrs`/`points` content (matching
/// the pre-existing `header_size` precedent) so a stale value here can never corrupt a re-encode;
/// they stay typed + diffable because they're real bytes on disk that `decode_las` retains
/// verbatim from whatever was actually read.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasHeader {
    pub version_major: u8,
    pub version_minor: u8,
    /// 🏢 §2.3 System Identifier — 32-byte ASCII, null/space-padded on the wire.
    pub system_identifier: String,
    /// 🛠️ §2.3 Generating Software — 32-byte ASCII, null/space-padded on the wire.
    pub generating_software: String,
    pub creation_day_of_year: u16,
    pub creation_year: u16,
    /// 📏 STRUCTURAL — see struct docs.
    pub header_size: u16,
    /// 📍 STRUCTURAL — see struct docs.
    pub offset_to_point_data: u32,
    /// 🔢 STRUCTURAL (== `vlrs.len()` after any successful encode) — see struct docs.
    pub number_of_vlrs: u32,
    /// 🆔 STRUCTURAL (chosen from which optional point fields are populated) — see struct docs.
    pub point_data_format_id: u8,
    /// 📐 STRUCTURAL (derived from `point_data_format_id`) — see struct docs.
    pub point_data_record_length: u16,
    /// 🔢 STRUCTURAL (== `points.len()` after any successful encode) — see struct docs.
    pub number_of_point_records: u32,
    /// 🔁 §2.3 Number of Points by Return — counts for return channels 1..=5. NOT structural:
    /// retained/settable independently of `points`' real return-number histogram (real-world LAS
    /// files frequently carry an inaccurate one; honest retention beats silent "correction").
    pub points_by_return: [u32; 5],
    pub x_scale: f64,
    pub y_scale: f64,
    pub z_scale: f64,
    pub x_offset: f64,
    pub y_offset: f64,
    pub z_offset: f64,
    pub max_x: f64,
    pub min_x: f64,
    pub max_y: f64,
    pub min_y: f64,
    pub max_z: f64,
    pub min_z: f64,
}

impl Default for LasHeader {
    fn default() -> Self {
        Self {
            version_major: 1,
            version_minor: 2,
            system_identifier: String::new(),
            generating_software: String::new(),
            creation_day_of_year: 0,
            creation_year: 0,
            header_size: 227,
            offset_to_point_data: 227,
            number_of_vlrs: 0,
            point_data_format_id: 0,
            point_data_record_length: 20,
            number_of_point_records: 0,
            points_by_return: [0; 5],
            x_scale: 0.01,
            y_scale: 0.01,
            z_scale: 0.01,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
            max_x: 0.0,
            min_x: 0.0,
            max_y: 0.0,
            min_y: 0.0,
            max_z: 0.0,
            min_z: 0.0,
        }
    }
}
//#endregion 🔖️Header

//#region 🔖️Vlr
/// 📦 One Variable Length Record — `data` is retained byte-verbatim (VLR content is registered
/// per `(user_id, record_id)` by third parties and is proprietary/unmodeled by spec, the
/// recipe's typed raw-retention exception, same shape as `PngChunk`/`GifAppExtension`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasVlr {
    pub user_id: String,
    pub record_id: u16,
    pub description: String,
    pub data: Vec<u8>,
}
//#endregion 🔖️Vlr

//#region 🔖️PointModel
/// 📍 One LAS point record, decomposed per LAS 1.2 §point data record formats 0-3 (this
/// artifact's contracted scope is formats 0/1; 2/3's `rgb` field is kept — already-working,
/// already-tested content the recipe's "nothing real on disk silently dropped" rule forbids
/// regressing). `gps_time` / `rgb` are `None` for point data formats that don't carry them (0/2
/// and 0/1 respectively).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub intensity: u16,
    pub return_number: u8,
    pub number_of_returns: u8,
    pub scan_direction_flag: bool,
    pub edge_of_flight_line: bool,
    pub classification: u8,
    pub scan_angle_rank: i8,
    pub user_data: u8,
    pub point_source_id: u16,
    #[serde(default)]
    pub gps_time: Option<f64>,
    #[serde(default)]
    pub rgb: Option<(u16, u16, u16)>,
}
//#endregion 🔖️PointModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las")]
pub struct LasSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub header: LasHeader,
    #[state(artifact)]
    #[serde(default)]
    pub vlrs: Vec<LasVlr>,
    #[state(artifact)]
    #[serde(default)]
    pub points: Vec<LasPoint>,
}

impl Default for LasSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), header: LasHeader::default(), vlrs: Vec::new(), points: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 📌 The real byte-level las codec (header field reads, VLR walk, point-data-format 0-3 record
// layouts) lives in `engine::{encode_las, decode_las}` per the png/jpg precedent; this
// impl block only wraps the hex-dump DSL envelope and the binary pack envelope around it.
impl store::ArtifactDsl for LasSnapshot {
    const EXTENSION: &'static str = "las";
    async fn envelope_id() -> &'static str {
        "stdio.las"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::las::engine::decode_las(&bytes).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::las::engine::encode_las(self).await.unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LasSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::las::engine::encode_las(self).await.map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::las::engine::decode_las(&inner).await.map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
