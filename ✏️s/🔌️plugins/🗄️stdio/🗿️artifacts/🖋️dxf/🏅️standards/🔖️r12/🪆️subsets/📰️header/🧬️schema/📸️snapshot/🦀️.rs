//! 🧬️ DxfSnapshot schema — complete per DXF R12 ASCII spec, not per codec capability.
//!
//! Ticket `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`: replaces the
//! old flat `tags: Vec<DxfTag>` passthrough model with a typed document: `$VAR`-keyed
//! [`DxfHeaderVar`] header, name-keyed [`DxfTables`] (LAYER/STYLE/LTYPE — the three table kinds
//! this codec typed-models; every other R12 table kind (VPORT/VIEW/UCS/APPID/DIMSTYLE/
//! BLOCK_RECORD) is retained verbatim in `other_tables`, never silently dropped), index-keyed
//! `blocks`, and index-keyed top-level `entities`. [`DxfEntity`] types LINE/CIRCLE/ARC/POLYLINE
//! (via the real R12 POLYLINE/VERTEX/SEQEND record group, not the R14+ LWPOLYLINE shape)/TEXT/
//! SOLID/INSERT; every other entity kind (3DFACE, POINT, DIMENSION, …) falls back to
//! `DxfEntity::Other{kind, group_codes}` — raw-retention, per the recipe's honesty rule. Every
//! typed entity/table/header-var additionally carries `unknown_group_codes`/`extra_group_codes`
//! for any group code within its own body this codec doesn't specifically model.
//!
//! [`DxfValue`] is the typed union over DXF group-code value kinds (string/integer/double/
//! point-component — the ticket's own listed kinds); `Point` combines a 10/20/30-style code
//! triplet (or 10/20 2D pair, z=0) into one 3-vector so multi-component header vars like
//! `$INSBASE`/`$EXTMIN`/`$EXTMAX` are captured losslessly under ONE name-keyed entry instead of
//! three same-named ones (a deliberate, documented divergence from a literal single-group-code
//! reading of the spec table — see the diff/mutations files' module docs and this wave's report).
//!
//! Native snapshot DSL/Pack preserve the complete owned model. Ordinary DXF ASCII file parsing
//! and printing retain their group-code representation in the explicit document codec helpers.

use crate::STDIO_DXF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use super::text as snapshot_text;

//#region 🔖️RawTag
/// 🏷️ One raw DXF group-code/value pair — used only as the tokenizer's intermediate unit and as
/// the raw-retention payload for whole unmodeled tables (`DxfOtherTable`). The typed model above
/// it (`DxfSnapshot`'s real fields) is the source of truth everywhere else.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfTag {
    pub code: i32,
    pub value: String,
}
//#endregion 🔖️RawTag

//#region 🔖️DxfValue
/// 🧮 Typed union over DXF group-code value kinds: string (codes 0-9/100-109/300-309/…),
/// integer (60-79/90-99/160-179/…), double (40-59/110-149/…), and point-component (a combined
/// 10/20/30-style triplet — see module docs). `classify_group_code_value` never produces `Point`
/// for a single raw tag; only header-var parsing manually combines an adjacent triplet into one.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DxfValue {
    Str { value: String },
    Int { value: i64 },
    Double { value: f64 },
    Point { value: [f64; 3] },
}

impl Default for DxfValue {
    fn default() -> Self {
        DxfValue::Str { value: String::new() }
    }
}








//#endregion 🔖️DxfValue

//#region 🔖️Header
/// 🏷️ One `$VAR` header entry: `9/$NAME` followed by its primary value group code, plus (rare)
/// any additional group codes beyond a plain scalar/point that this codec still retains losslessly.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfHeaderVar {
    pub name: String,
    pub group_code: i32,
    pub value: DxfValue,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_group_codes: Vec<(i32, DxfValue)>,
}
//#endregion 🔖️Header

//#region 🔖️Tables
/// 🗂️ `LAYER` table entry — group codes 2 (name), 70 (flags), 62 (color), 6 (linetype).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfLayer {
    pub name: String,
    pub color: i32,
    pub linetype: String,
    pub flags: i32,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 🗂️ `STYLE` table entry — group codes 2 (name), 70 (flags), 3 (primary font file).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfStyle {
    pub name: String,
    pub flags: i32,
    pub font_name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 🗂️ `LTYPE` table entry — group codes 2 (name), 70 (flags), 3 (description).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfLinetype {
    pub name: String,
    pub flags: i32,
    pub description: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 🗂️ The three name-keyed table kinds this codec typed-models.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfTables {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<DxfLayer>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub styles: Vec<DxfStyle>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub linetypes: Vec<DxfLinetype>,
}

/// 🕳️ Raw retention for any R12 `TABLE` kind other than LAYER/STYLE/LTYPE (VPORT, VIEW, UCS,
/// APPID, DIMSTYLE, BLOCK_RECORD, …) — this codec has no typed view for these, but every tag is
/// preserved verbatim, per the recipe's raw-retention rule.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfOtherTable {
    pub name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<DxfTag>,
}
//#endregion 🔖️Tables

//#region 🔖️Entities
/// 📍 One `POLYLINE` vertex record — group codes 10/20/30 (point), 42 (bulge).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub bulge: f64,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 📐️ The R12 entity set this codec types directly. `Other` retains any entity kind this codec
/// has no typed view for (`3DFACE`, `POINT`, `DIMENSION`, `SHAPE`, `ATTRIB`, …) — its whole
/// group-code body verbatim, never silently dropped.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum DxfEntity {
    /// `LINE` — 10/20/30 (start), 11/21/31 (end), 8 (layer).
    Line {
        start: [f64; 3],
        end: [f64; 3],
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `CIRCLE` — 10/20/30 (center), 40 (radius), 8 (layer).
    Circle {
        center: [f64; 3],
        radius: f64,
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `ARC` — 10/20/30 (center), 40 (radius), 50/51 (start/end angle), 8 (layer).
    Arc {
        center: [f64; 3],
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `POLYLINE`/`VERTEX`.../`SEQEND` (the real R12 polyline record group — NOT the R14+
    /// `LWPOLYLINE` entity, which does not exist in R12) — 70 bit 0 (closed), 8 (layer), each
    /// vertex its own `DxfVertex`.
    Polyline {
        vertices: Vec<DxfVertex>,
        closed: bool,
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `TEXT` — 10/20/30 (position), 40 (height), 1 (value), 8 (layer).
    Text {
        position: [f64; 3],
        height: f64,
        value: String,
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `SOLID` — 10/20/30, 11/21/31, 12/22/32, 13/23/33 (4 corner points), 8 (layer).
    Solid {
        points: [[f64; 3]; 4],
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `INSERT` — 2 (block name), 10/20/30 (position), 41/42/43 (scale, default 1/1/1), 50
    /// (rotation), 8 (layer).
    Insert {
        block_name: String,
        position: [f64; 3],
        scale: [f64; 3],
        rotation: f64,
        layer: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// 🕳️ Any other entity kind — raw-retained verbatim.
    Other {
        kind: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        group_codes: Vec<(i32, DxfValue)>,
    },
}
//#endregion 🔖️Entities

//#region 🔖️Blocks
/// 🧱 One `BLOCK` — 2 (name), 10/20/30 (base point), followed by its own nested entity list up
/// to `ENDBLK`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfBlock {
    pub name: String,
    pub base_point: [f64; 3],
    pub entities: Vec<DxfEntity>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}
//#endregion 🔖️Blocks

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🏷️ `HEADER` section — every `$VAR`, name-keyed.
    #[state(artifact)]
    #[value(default)]
    pub header_vars: Vec<DxfHeaderVar>,
    /// 🗂️ `TABLES` section — the three typed table kinds.
    #[state(artifact)]
    #[value(default)]
    pub tables: DxfTables,
    /// 🕳️ `TABLES` section — every other table kind, raw-retained.
    #[state(artifact)]
    #[value(default)]
    pub other_tables: Vec<DxfOtherTable>,
    /// 🧱 `BLOCKS` section.
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<DxfBlock>,
    /// 📐️ `ENTITIES` section.
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<DxfEntity>,
}

impl Default for DxfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), header_vars: Vec::new(), tables: DxfTables::default(), other_tables: Vec::new(), blocks: Vec::new(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Tokenizer



//#endregion 🔖️Tokenizer

//#region 🔖️HeaderCodec







//#endregion 🔖️HeaderCodec

//#region 🔖️TablesCodec















//#endregion 🔖️TablesCodec

//#region 🔖️EntityCodec













//#endregion 🔖️EntityCodec

//#region 🔖️BlocksCodec



//#endregion 🔖️BlocksCodec

//#region 🔖️DocumentCodec



//#endregion 🔖️DocumentCodec

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

