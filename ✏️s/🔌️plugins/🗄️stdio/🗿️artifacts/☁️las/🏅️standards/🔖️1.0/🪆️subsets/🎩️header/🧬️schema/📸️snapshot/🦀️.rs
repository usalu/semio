//! 🧬️ LasSnapshot schema — full LAS 1.0 public header block + VLRs + point records. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the earlier
//! "Weak" tier (`{schema, points}` only, no header/VLRs) with the recipe's full-completeness
//! model: a typed `LasHeader` (every real §LAS 1.0 public header block field the ticket lists),
//! an index-keyed `vlrs: Vec<LasVlr>` (payload verbatim — VLR content is proprietary/unmodeled
//! per-registered-id, the recipe's typed raw-retention exception), and the existing index-keyed
//! `points: Vec<LasPoint>`.

use crate::STDIO_LAS_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
    #[value(default)]
    pub gps_time: Option<f64>,
    #[value(default)]
    pub rgb: Option<(u16, u16, u16)>,
}
//#endregion 🔖️PointModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las")]
pub struct LasSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: LasHeader,
    #[state(artifact)]
    #[value(default)]
    pub vlrs: Vec<LasVlr>,
    #[state(artifact)]
    #[value(default)]
    pub points: Vec<LasPoint>,
}

impl Default for LasSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), header: LasHeader::default(), vlrs: Vec::new(), points: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

#[path="📦️pack/🦀️.rs"]
mod pack;
