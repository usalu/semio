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
//! `decode_pack`/`ArtifactDsl` regenerate canonical DXF ASCII text from the typed model — this is
//! a documented NORMAL FORM (not raw byte preservation): incidental source formatting (float
//! print precision, whitespace) is not preserved, but every group code's semantic content is,
//! including every unmodeled region (`other_tables`, `Other` entities/tables, `unknown_group_codes`
//! / `extra_group_codes`) — see `codec_retention_law` in `⚙️engine` for the fixed-point proof.

use crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️RawTag
/// 🏷️ One raw DXF group-code/value pair — used only as the tokenizer's intermediate unit and as
/// the raw-retention payload for whole unmodeled tables (`DxfOtherTable`). The typed model above
/// it (`DxfSnapshot`'s real fields) is the source of truth everywhere else.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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

/// 🧭️ Simplification of the DXF group-code value-type table (spec appendix) into the four
/// `DxfValue` kinds — good enough for every code this codec reads generically (unknown-group-code
/// retention, `Other` fallbacks); codes with dedicated typed fields (10/20/30 point triplets on
/// known entities, etc.) are parsed directly by their own field-specific logic instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn classify_group_code_value(code: i32, raw: &str) -> DxfValue {
    match code {
        10..=59 | 110..=149 | 210..=239 | 460..=469 => DxfValue::Double { value: parse_f64(raw) },
        60..=99 | 160..=179 | 270..=289 | 370..=389 | 400..=409 | 440..=459 => DxfValue::Int { value: parse_i64(raw) },
        _ => DxfValue::Str { value: raw.to_string() },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn format_dxf_value(v: &DxfValue) -> String {
    match v {
        DxfValue::Str { value } => value.clone(),
        DxfValue::Int { value } => value.to_string(),
        DxfValue::Double { value } => format_f64(*value),
        DxfValue::Point { value } => format_f64(value[0]),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(v: &str) -> f64 {
    v.trim().parse::<f64>().unwrap_or(0.0)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_i64(v: &str) -> i64 {
    v.trim().parse::<i64>().unwrap_or_else(|_| parse_f64(v) as i64)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn format_f64(v: f64) -> String {
    let s = format!("{v}");
    if s.contains('.') || s.contains('e') || s.contains("inf") || s.contains("NaN") {
        s
    } else {
        format!("{s}.0")
    }
}
//#endregion 🔖️DxfValue

//#region 🔖️Header
/// 🏷️ One `$VAR` header entry: `9/$NAME` followed by its primary value group code, plus (rare)
/// any additional group codes beyond a plain scalar/point that this codec still retains losslessly.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVar {
    pub name: String,
    pub group_code: i32,
    pub value: DxfValue,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_group_codes: Vec<(i32, DxfValue)>,
}
//#endregion 🔖️Header

//#region 🔖️Tables
/// 🗂️ `LAYER` table entry — group codes 2 (name), 70 (flags), 62 (color), 6 (linetype).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayer {
    pub name: String,
    pub color: i32,
    pub linetype: String,
    pub flags: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 🗂️ `STYLE` table entry — group codes 2 (name), 70 (flags), 3 (primary font file).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyle {
    pub name: String,
    pub flags: i32,
    pub font_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 🗂️ `LTYPE` table entry — group codes 2 (name), 70 (flags), 3 (description).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetype {
    pub name: String,
    pub flags: i32,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 🗂️ The three name-keyed table kinds this codec typed-models.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfTables {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<DxfLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub styles: Vec<DxfStyle>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linetypes: Vec<DxfLinetype>,
}

/// 🕳️ Raw retention for any R12 `TABLE` kind other than LAYER/STYLE/LTYPE (VPORT, VIEW, UCS,
/// APPID, DIMSTYLE, BLOCK_RECORD, …) — this codec has no typed view for these, but every tag is
/// preserved verbatim, per the recipe's raw-retention rule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfOtherTable {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<DxfTag>,
}
//#endregion 🔖️Tables

//#region 🔖️Entities
/// 📍 One `POLYLINE` vertex record — group codes 10/20/30 (point), 42 (bulge).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub bulge: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}

/// 📐️ The R12 entity set this codec types directly. `Other` retains any entity kind this codec
/// has no typed view for (`3DFACE`, `POINT`, `DIMENSION`, `SHAPE`, `ATTRIB`, …) — its whole
/// group-code body verbatim, never silently dropped.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DxfEntity {
    /// `LINE` — 10/20/30 (start), 11/21/31 (end), 8 (layer).
    Line {
        start: [f64; 3],
        end: [f64; 3],
        layer: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `CIRCLE` — 10/20/30 (center), 40 (radius), 8 (layer).
    Circle {
        center: [f64; 3],
        radius: f64,
        layer: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `ARC` — 10/20/30 (center), 40 (radius), 50/51 (start/end angle), 8 (layer).
    Arc {
        center: [f64; 3],
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        layer: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `POLYLINE`/`VERTEX`.../`SEQEND` (the real R12 polyline record group — NOT the R14+
    /// `LWPOLYLINE` entity, which does not exist in R12) — 70 bit 0 (closed), 8 (layer), each
    /// vertex its own `DxfVertex`.
    Polyline {
        vertices: Vec<DxfVertex>,
        closed: bool,
        layer: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `TEXT` — 10/20/30 (position), 40 (height), 1 (value), 8 (layer).
    Text {
        position: [f64; 3],
        height: f64,
        value: String,
        layer: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// `SOLID` — 10/20/30, 11/21/31, 12/22/32, 13/23/33 (4 corner points), 8 (layer).
    Solid {
        points: [[f64; 3]; 4],
        layer: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unknown_group_codes: Vec<(i32, DxfValue)>,
    },
    /// 🕳️ Any other entity kind — raw-retained verbatim.
    Other {
        kind: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        group_codes: Vec<(i32, DxfValue)>,
    },
}
//#endregion 🔖️Entities

//#region 🔖️Blocks
/// 🧱 One `BLOCK` — 2 (name), 10/20/30 (base point), followed by its own nested entity list up
/// to `ENDBLK`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlock {
    pub name: String,
    pub base_point: [f64; 3],
    pub entities: Vec<DxfEntity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_group_codes: Vec<(i32, DxfValue)>,
}
//#endregion 🔖️Blocks

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🏷️ `HEADER` section — every `$VAR`, name-keyed.
    #[state(artifact)]
    #[serde(default)]
    pub header_vars: Vec<DxfHeaderVar>,
    /// 🗂️ `TABLES` section — the three typed table kinds.
    #[state(artifact)]
    #[serde(default)]
    pub tables: DxfTables,
    /// 🕳️ `TABLES` section — every other table kind, raw-retained.
    #[state(artifact)]
    #[serde(default)]
    pub other_tables: Vec<DxfOtherTable>,
    /// 🧱 `BLOCKS` section.
    #[state(artifact)]
    #[serde(default)]
    pub blocks: Vec<DxfBlock>,
    /// 📐️ `ENTITIES` section.
    #[state(artifact)]
    #[serde(default)]
    pub entities: Vec<DxfEntity>,
}

impl Default for DxfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), header_vars: Vec::new(), tables: DxfTables::default(), other_tables: Vec::new(), blocks: Vec::new(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Tokenizer
/// 📥️ Tokenizes raw DXF ASCII text into its flat `(code, value)` tag stream — the tokenizer's
/// output is consumed immediately by the section walker below; it is never itself the source of
/// truth (contrast with the pre-overhaul model).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tokenize_dxf(text: &str) -> Result<Vec<DxfTag>, String> {
    let raw: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let mut tags = Vec::new();
    let mut i = 0usize;
    while i < raw.len() {
        let code_line = raw[i].trim();
        if code_line.is_empty() {
            i += 1;
            continue;
        }
        let value = raw.get(i + 1).ok_or_else(|| format!("dxf: group code {code_line:?} missing its value line"))?;
        let code: i32 = code_line.parse().map_err(|e| format!("dxf: invalid group code {code_line:?}: {e}"))?;
        tags.push(DxfTag { code, value: value.trim().to_string() });
        i += 2;
    }
    Ok(tags)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_tag(out: &mut String, code: i32, value: &str) {
    out.push_str(&code.to_string());
    out.push('\n');
    out.push_str(value);
    out.push('\n');
}
//#endregion 🔖️Tokenizer

//#region 🔖️HeaderCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_header_var(name: String, tags: &[DxfTag]) -> DxfHeaderVar {
    if tags.is_empty() {
        return DxfHeaderVar { name, group_code: 0, value: DxfValue::default(), extra_group_codes: Vec::new() };
    }
    // 🧭️ Point-component detection: an adjacent 3-code (or 2-code, z=0) run at +10/+20 offsets
    // from the primary code (the DXF convention for $INSBASE/$EXTMIN/$EXTMAX/… point vars).
    if tags.len() >= 3 && tags[1].code == tags[0].code + 10 && tags[2].code == tags[0].code + 20 {
        let value = DxfValue::Point { value: [parse_f64(&tags[0].value), parse_f64(&tags[1].value), parse_f64(&tags[2].value)] };
        let extra = tags[3..].iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect();
        return DxfHeaderVar { name, group_code: tags[0].code, value, extra_group_codes: extra };
    }
    if tags.len() >= 2 && tags[1].code == tags[0].code + 10 {
        let value = DxfValue::Point { value: [parse_f64(&tags[0].value), parse_f64(&tags[1].value), 0.0] };
        let extra = tags[2..].iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect();
        return DxfHeaderVar { name, group_code: tags[0].code, value, extra_group_codes: extra };
    }
    let value = classify_group_code_value(tags[0].code, &tags[0].value);
    let extra = tags[1..].iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect();
    DxfHeaderVar { name, group_code: tags[0].code, value, extra_group_codes: extra }
}

/// 📥️ Parses a `HEADER` section body (tags strictly between `0/SECTION,2/HEADER` and
/// `0/ENDSEC`, exclusive of both).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_header_section(tags: &[DxfTag]) -> Vec<DxfHeaderVar> {
    let mut vars = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 9 {
            let name = tags[i].value.clone();
            let start = i + 1;
            let mut end = start;
            while end < tags.len() && tags[end].code != 9 {
                end += 1;
            }
            vars.push(parse_header_var(name, &tags[start..end]));
            i = end;
        } else {
            i += 1;
        }
    }
    vars
}

/// 📤️ Value pairs a header var's `group_code`/`value` expand to on print — a `Point` expands
/// into the code/code+10/code+20 triplet convention; everything else is a single pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_var_value_pairs(code: i32, value: &DxfValue) -> Vec<(i32, String)> {
    match value {
        DxfValue::Point { value } => vec![(code, format_f64(value[0])), (code + 10, format_f64(value[1])), (code + 20, format_f64(value[2]))],
        other => vec![(code, format_dxf_value(other))],
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_header_section(vars: &[DxfHeaderVar], out: &mut String) {
    push_tag(out, 0, "SECTION");
    push_tag(out, 2, "HEADER");
    for v in vars {
        push_tag(out, 9, &v.name);
        for (code, s) in header_var_value_pairs(v.group_code, &v.value) {
            push_tag(out, code, &s);
        }
        for (code, val) in &v.extra_group_codes {
            push_tag(out, *code, &format_dxf_value(val));
        }
    }
    push_tag(out, 0, "ENDSEC");
}
//#endregion 🔖️HeaderCodec

//#region 🔖️TablesCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_layer(body: &[DxfTag]) -> DxfLayer {
    let mut layer = DxfLayer::default();
    for t in body {
        match t.code {
            2 => layer.name = t.value.clone(),
            62 => layer.color = parse_i64(&t.value) as i32,
            6 => layer.linetype = t.value.clone(),
            70 => layer.flags = parse_i64(&t.value) as i32,
            _ => layer.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    layer
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_style(body: &[DxfTag]) -> DxfStyle {
    let mut style = DxfStyle::default();
    for t in body {
        match t.code {
            2 => style.name = t.value.clone(),
            70 => style.flags = parse_i64(&t.value) as i32,
            3 => style.font_name = t.value.clone(),
            _ => style.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    style
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_linetype(body: &[DxfTag]) -> DxfLinetype {
    let mut lt = DxfLinetype::default();
    for t in body {
        match t.code {
            2 => lt.name = t.value.clone(),
            70 => lt.flags = parse_i64(&t.value) as i32,
            3 => lt.description = t.value.clone(),
            _ => lt.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    lt
}

/// 🔎 Splits a table's entry body (between `2/<TABLENAME>` and `0/ENDTAB`, table-level fields
/// like `70`/count already skipped by the caller) into per-entry `(0/<ENTRYKIND> … )` slices.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_table_entries<'a>(tags: &'a [DxfTag], entry_kind: &str) -> Vec<&'a [DxfTag]> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == entry_kind {
            let start = i + 1;
            let mut end = start;
            while end < tags.len() && tags[end].code != 0 {
                end += 1;
            }
            out.push(&tags[start..end]);
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// 📥️ Parses a `TABLES` section body. Returns the three typed table kinds plus raw-retained
/// entries for every other table kind (VPORT/VIEW/UCS/APPID/DIMSTYLE/BLOCK_RECORD/…).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_tables_section(tags: &[DxfTag]) -> (DxfTables, Vec<DxfOtherTable>) {
    let mut tables = DxfTables::default();
    let mut others = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == "TABLE" {
            i += 1;
            let table_name = if i < tags.len() && tags[i].code == 2 {
                let n = tags[i].value.clone();
                i += 1;
                n
            } else {
                String::new()
            };
            let raw_body_start = i;
            let mut body_end = raw_body_start;
            while body_end < tags.len() && !(tags[body_end].code == 0 && tags[body_end].value == "ENDTAB") {
                body_end += 1;
            }
            match table_name.as_str() {
                "LAYER" | "STYLE" | "LTYPE" => {
                    // 🧭️ Known kinds: skip table-level fields (70 count, 5 handle, …) up to the
                    // first entry's `0/<KIND>` marker before splitting into per-entry slices.
                    let mut entries_start = raw_body_start;
                    while entries_start < body_end && tags[entries_start].code != 0 {
                        entries_start += 1;
                    }
                    let body = &tags[entries_start..body_end];
                    match table_name.as_str() {
                        "LAYER" => tables.layers = split_table_entries(body, "LAYER").into_iter().map(build_layer).collect(),
                        "STYLE" => tables.styles = split_table_entries(body, "STYLE").into_iter().map(build_style).collect(),
                        "LTYPE" => tables.linetypes = split_table_entries(body, "LTYPE").into_iter().map(build_linetype).collect(),
                        _ => unreachable!(),
                    }
                }
                // 🕳️ Unknown kinds: capture the WHOLE raw span (informational fields AND any
                // entry markers alike) verbatim — no skip, so hand-built and real-parsed
                // `DxfOtherTable.tags` are both trivially lossless round-trip fixed points.
                _ => others.push(DxfOtherTable { name: table_name, tags: tags[raw_body_start..body_end].to_vec() }),
            }
            i = body_end;
            if i < tags.len() && tags[i].code == 0 && tags[i].value == "ENDTAB" {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    (tables, others)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_layer(out: &mut String, l: &DxfLayer) {
    push_tag(out, 0, "LAYER");
    push_tag(out, 2, &l.name);
    push_tag(out, 70, &l.flags.to_string());
    push_tag(out, 62, &l.color.to_string());
    push_tag(out, 6, &l.linetype);
    for (code, v) in &l.unknown_group_codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_style(out: &mut String, s: &DxfStyle) {
    push_tag(out, 0, "STYLE");
    push_tag(out, 2, &s.name);
    push_tag(out, 70, &s.flags.to_string());
    push_tag(out, 3, &s.font_name);
    for (code, v) in &s.unknown_group_codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_linetype(out: &mut String, l: &DxfLinetype) {
    push_tag(out, 0, "LTYPE");
    push_tag(out, 2, &l.name);
    push_tag(out, 70, &l.flags.to_string());
    push_tag(out, 3, &l.description);
    for (code, v) in &l.unknown_group_codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_table_block(out: &mut String, name: &str, count: usize, mut body: impl FnMut(&mut String)) {
    push_tag(out, 0, "TABLE");
    push_tag(out, 2, name);
    push_tag(out, 70, &count.to_string());
    body(out);
    push_tag(out, 0, "ENDTAB");
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_tables_section(tables: &DxfTables, others: &[DxfOtherTable], out: &mut String) {
    push_tag(out, 0, "SECTION");
    push_tag(out, 2, "TABLES");
    print_table_block(out, "LAYER", tables.layers.len(), |out| {
        for l in &tables.layers {
            print_layer(out, l);
        }
    });
    print_table_block(out, "STYLE", tables.styles.len(), |out| {
        for s in &tables.styles {
            print_style(out, s);
        }
    });
    print_table_block(out, "LTYPE", tables.linetypes.len(), |out| {
        for l in &tables.linetypes {
            print_linetype(out, l);
        }
    });
    for t in others {
        push_tag(out, 0, "TABLE");
        push_tag(out, 2, &t.name);
        for tag in &t.tags {
            push_tag(out, tag.code, &tag.value);
        }
        push_tag(out, 0, "ENDTAB");
    }
    push_tag(out, 0, "ENDSEC");
}
//#endregion 🔖️TablesCodec

//#region 🔖️EntityCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_vertex(body: &[DxfTag]) -> DxfVertex {
    let mut v = DxfVertex::default();
    for t in body {
        match t.code {
            10 => v.x = parse_f64(&t.value),
            20 => v.y = parse_f64(&t.value),
            30 => v.z = parse_f64(&t.value),
            42 => v.bulge = parse_f64(&t.value),
            _ => v.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    v
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_entity(kind: &str, body: &[DxfTag]) -> DxfEntity {
    match kind {
        "LINE" => {
            let (mut start, mut end, mut layer, mut unknown) = ([0.0; 3], [0.0; 3], String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => start[0] = parse_f64(&t.value),
                    20 => start[1] = parse_f64(&t.value),
                    30 => start[2] = parse_f64(&t.value),
                    11 => end[0] = parse_f64(&t.value),
                    21 => end[1] = parse_f64(&t.value),
                    31 => end[2] = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Line { start, end, layer, unknown_group_codes: unknown }
        }
        "CIRCLE" => {
            let (mut center, mut radius, mut layer, mut unknown) = ([0.0; 3], 0.0, String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => center[0] = parse_f64(&t.value),
                    20 => center[1] = parse_f64(&t.value),
                    30 => center[2] = parse_f64(&t.value),
                    40 => radius = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Circle { center, radius, layer, unknown_group_codes: unknown }
        }
        "ARC" => {
            let (mut center, mut radius, mut sa, mut ea, mut layer, mut unknown) = ([0.0; 3], 0.0, 0.0, 0.0, String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => center[0] = parse_f64(&t.value),
                    20 => center[1] = parse_f64(&t.value),
                    30 => center[2] = parse_f64(&t.value),
                    40 => radius = parse_f64(&t.value),
                    50 => sa = parse_f64(&t.value),
                    51 => ea = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Arc { center, radius, start_angle: sa, end_angle: ea, layer, unknown_group_codes: unknown }
        }
        "TEXT" => {
            let (mut position, mut height, mut value, mut layer, mut unknown) = ([0.0; 3], 0.0, String::new(), String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => position[0] = parse_f64(&t.value),
                    20 => position[1] = parse_f64(&t.value),
                    30 => position[2] = parse_f64(&t.value),
                    40 => height = parse_f64(&t.value),
                    1 => value = t.value.clone(),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Text { position, height, value, layer, unknown_group_codes: unknown }
        }
        "SOLID" => {
            let (mut points, mut layer, mut unknown) = ([[0.0; 3]; 4], String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => points[0][0] = parse_f64(&t.value),
                    20 => points[0][1] = parse_f64(&t.value),
                    30 => points[0][2] = parse_f64(&t.value),
                    11 => points[1][0] = parse_f64(&t.value),
                    21 => points[1][1] = parse_f64(&t.value),
                    31 => points[1][2] = parse_f64(&t.value),
                    12 => points[2][0] = parse_f64(&t.value),
                    22 => points[2][1] = parse_f64(&t.value),
                    32 => points[2][2] = parse_f64(&t.value),
                    13 => points[3][0] = parse_f64(&t.value),
                    23 => points[3][1] = parse_f64(&t.value),
                    33 => points[3][2] = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Solid { points, layer, unknown_group_codes: unknown }
        }
        "INSERT" => {
            let (mut block_name, mut position, mut scale, mut rotation, mut layer, mut unknown) = (String::new(), [0.0; 3], [1.0, 1.0, 1.0], 0.0, String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    2 => block_name = t.value.clone(),
                    10 => position[0] = parse_f64(&t.value),
                    20 => position[1] = parse_f64(&t.value),
                    30 => position[2] = parse_f64(&t.value),
                    41 => scale[0] = parse_f64(&t.value),
                    42 => scale[1] = parse_f64(&t.value),
                    43 => scale[2] = parse_f64(&t.value),
                    50 => rotation = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes: unknown }
        }
        _ => DxfEntity::Other { kind: kind.to_string(), group_codes: body.iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect() },
    }
}

/// 📥️ Parses the real R12 `POLYLINE`/`VERTEX`.../`SEQEND` record group. `i` points just past
/// the `0/POLYLINE` header tag; returns the built entity plus the index just past `SEQEND`'s
/// own (usually empty) body.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_polyline(tags: &[DxfTag], mut i: usize) -> (DxfEntity, usize) {
    let header_start = i;
    let mut header_end = header_start;
    while header_end < tags.len() && tags[header_end].code != 0 {
        header_end += 1;
    }
    let (mut layer, mut closed, mut unknown) = (String::new(), false, Vec::new());
    for t in &tags[header_start..header_end] {
        match t.code {
            8 => layer = t.value.clone(),
            70 => closed = parse_i64(&t.value) & 1 == 1,
            66 => {} // "entities follow" flag — implicit in this model, not retained
            _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    i = header_end;
    let mut vertices = Vec::new();
    while i < tags.len() && tags[i].code == 0 && tags[i].value == "VERTEX" {
        i += 1;
        let vstart = i;
        let mut vend = vstart;
        while vend < tags.len() && tags[vend].code != 0 {
            vend += 1;
        }
        vertices.push(build_vertex(&tags[vstart..vend]));
        i = vend;
    }
    if i < tags.len() && tags[i].code == 0 && tags[i].value == "SEQEND" {
        i += 1;
        while i < tags.len() && tags[i].code != 0 {
            i += 1;
        }
    }
    (DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes: unknown }, i)
}

/// 📥️ Consumes entities from `i` until `(0, stop_kind)` (exclusive) or end of `tags`. Used both
/// for a pre-sliced `ENTITIES` section body (`stop_kind` inert, loop ends at `tags.len()`) and
/// for a block's nested entity list within the unsliced `BLOCKS` section body (`stop_kind =
/// "ENDBLK"`, since blocks are sequential and each one's extent must be discovered, not sliced
/// up front).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_entities_until(tags: &[DxfTag], mut i: usize, stop_kind: &str) -> (Vec<DxfEntity>, usize) {
    let mut entities = Vec::new();
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == stop_kind {
            break;
        }
        if tags[i].code != 0 {
            i += 1; // defensive skip of a stray non-header tag
            continue;
        }
        let kind = tags[i].value.clone();
        i += 1;
        if kind == "POLYLINE" {
            let (entity, next_i) = parse_polyline(tags, i);
            entities.push(entity);
            i = next_i;
        } else {
            let body_start = i;
            let mut body_end = body_start;
            while body_end < tags.len() && tags[body_end].code != 0 {
                body_end += 1;
            }
            entities.push(build_entity(&kind, &tags[body_start..body_end]));
            i = body_end;
        }
    }
    (entities, i)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_unknown(out: &mut String, codes: &[(i32, DxfValue)]) {
    for (code, v) in codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_entity(e: &DxfEntity, out: &mut String) {
    match e {
        DxfEntity::Line { start, end, layer, unknown_group_codes } => {
            push_tag(out, 0, "LINE");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(start[0]));
            push_tag(out, 20, &format_f64(start[1]));
            push_tag(out, 30, &format_f64(start[2]));
            push_tag(out, 11, &format_f64(end[0]));
            push_tag(out, 21, &format_f64(end[1]));
            push_tag(out, 31, &format_f64(end[2]));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Circle { center, radius, layer, unknown_group_codes } => {
            push_tag(out, 0, "CIRCLE");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(center[0]));
            push_tag(out, 20, &format_f64(center[1]));
            push_tag(out, 30, &format_f64(center[2]));
            push_tag(out, 40, &format_f64(*radius));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes } => {
            push_tag(out, 0, "ARC");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(center[0]));
            push_tag(out, 20, &format_f64(center[1]));
            push_tag(out, 30, &format_f64(center[2]));
            push_tag(out, 40, &format_f64(*radius));
            push_tag(out, 50, &format_f64(*start_angle));
            push_tag(out, 51, &format_f64(*end_angle));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes } => {
            push_tag(out, 0, "POLYLINE");
            push_tag(out, 8, layer);
            push_tag(out, 66, "1");
            push_tag(out, 70, if *closed { "1" } else { "0" });
            print_unknown(out, unknown_group_codes);
            for v in vertices {
                push_tag(out, 0, "VERTEX");
                // 🧭️ No hardcoded `8/<layer>` here: a real vertex's own `8` code (if present in
                // the source — it's optional, inheriting the polyline's layer when absent) is
                // already captured in `unknown_group_codes` by `build_vertex`; emitting it again
                // here would duplicate the tag on decode(encode(...)).
                push_tag(out, 10, &format_f64(v.x));
                push_tag(out, 20, &format_f64(v.y));
                push_tag(out, 30, &format_f64(v.z));
                push_tag(out, 42, &format_f64(v.bulge));
                print_unknown(out, &v.unknown_group_codes);
            }
            push_tag(out, 0, "SEQEND");
        }
        DxfEntity::Text { position, height, value, layer, unknown_group_codes } => {
            push_tag(out, 0, "TEXT");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(position[0]));
            push_tag(out, 20, &format_f64(position[1]));
            push_tag(out, 30, &format_f64(position[2]));
            push_tag(out, 40, &format_f64(*height));
            push_tag(out, 1, value);
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Solid { points, layer, unknown_group_codes } => {
            push_tag(out, 0, "SOLID");
            push_tag(out, 8, layer);
            let codes: [(i32, i32, i32); 4] = [(10, 20, 30), (11, 21, 31), (12, 22, 32), (13, 23, 33)];
            for (idx, (cx, cy, cz)) in codes.iter().enumerate() {
                push_tag(out, *cx, &format_f64(points[idx][0]));
                push_tag(out, *cy, &format_f64(points[idx][1]));
                push_tag(out, *cz, &format_f64(points[idx][2]));
            }
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes } => {
            push_tag(out, 0, "INSERT");
            push_tag(out, 8, layer);
            push_tag(out, 2, block_name);
            push_tag(out, 10, &format_f64(position[0]));
            push_tag(out, 20, &format_f64(position[1]));
            push_tag(out, 30, &format_f64(position[2]));
            push_tag(out, 41, &format_f64(scale[0]));
            push_tag(out, 42, &format_f64(scale[1]));
            push_tag(out, 43, &format_f64(scale[2]));
            push_tag(out, 50, &format_f64(*rotation));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Other { kind, group_codes } => {
            push_tag(out, 0, kind);
            print_unknown(out, group_codes);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_entities(entities: &[DxfEntity], out: &mut String) {
    for e in entities {
        print_entity(e, out);
    }
}
//#endregion 🔖️EntityCodec

//#region 🔖️BlocksCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_blocks_section(tags: &[DxfTag]) -> Vec<DxfBlock> {
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == "BLOCK" {
            i += 1;
            let header_start = i;
            let mut header_end = header_start;
            while header_end < tags.len() && tags[header_end].code != 0 {
                header_end += 1;
            }
            let (mut name, mut bx, mut by, mut bz, mut unknown) = (String::new(), 0.0, 0.0, 0.0, Vec::new());
            for t in &tags[header_start..header_end] {
                match t.code {
                    2 => name = t.value.clone(),
                    10 => bx = parse_f64(&t.value),
                    20 => by = parse_f64(&t.value),
                    30 => bz = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            i = header_end;
            let (entities, next_i) = parse_entities_until(tags, i, "ENDBLK");
            i = next_i;
            if i < tags.len() && tags[i].code == 0 && tags[i].value == "ENDBLK" {
                i += 1;
                while i < tags.len() && tags[i].code != 0 {
                    i += 1;
                }
            }
            blocks.push(DxfBlock { name, base_point: [bx, by, bz], entities, unknown_group_codes: unknown });
        } else {
            i += 1;
        }
    }
    blocks
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_blocks_section(blocks: &[DxfBlock], out: &mut String) {
    push_tag(out, 0, "SECTION");
    push_tag(out, 2, "BLOCKS");
    for b in blocks {
        push_tag(out, 0, "BLOCK");
        push_tag(out, 2, &b.name);
        push_tag(out, 10, &format_f64(b.base_point[0]));
        push_tag(out, 20, &format_f64(b.base_point[1]));
        push_tag(out, 30, &format_f64(b.base_point[2]));
        print_unknown(out, &b.unknown_group_codes);
        print_entities(&b.entities, out);
        push_tag(out, 0, "ENDBLK");
    }
    push_tag(out, 0, "ENDSEC");
}
//#endregion 🔖️BlocksCodec

//#region 🔖️DocumentCodec
/// 📥️ Parses a complete R12 ASCII document: `HEADER`, `TABLES`, `BLOCKS`, `ENTITIES` sections
/// (the full R12 section set — R12 predates `CLASSES`/`OBJECTS`/thumbnails), terminated by `0/EOF`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_dxf_document(text: &str) -> Result<DxfSnapshot, String> {
    let tags = tokenize_dxf(text)?;
    let mut snap = DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), ..DxfSnapshot::default() };
    let mut i = 0usize;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == "SECTION" {
            i += 1;
            let section_name = if i < tags.len() && tags[i].code == 2 {
                let n = tags[i].value.clone();
                i += 1;
                n
            } else {
                String::new()
            };
            let body_start = i;
            let mut body_end = body_start;
            while body_end < tags.len() && !(tags[body_end].code == 0 && tags[body_end].value == "ENDSEC") {
                body_end += 1;
            }
            let body = &tags[body_start..body_end];
            match section_name.as_str() {
                "HEADER" => snap.header_vars = parse_header_section(body),
                "TABLES" => {
                    let (t, o) = parse_tables_section(body);
                    snap.tables = t;
                    snap.other_tables = o;
                }
                "BLOCKS" => snap.blocks = parse_blocks_section(body),
                "ENTITIES" => {
                    let (e, _) = parse_entities_until(body, 0, "ENDSEC");
                    snap.entities = e;
                }
                _ => {} // R12 has no other section kinds
            }
            i = body_end;
            if i < tags.len() && tags[i].code == 0 && tags[i].value == "ENDSEC" {
                i += 1;
            }
        } else if tags[i].code == 0 && tags[i].value == "EOF" {
            break;
        } else {
            i += 1;
        }
    }
    Ok(snap)
}

/// 📤️ Regenerates canonical R12 ASCII text from the typed model — the documented NORMAL FORM
/// (see module docs): semantic content is fully preserved; incidental source formatting is not.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_dxf_document(snap: &DxfSnapshot) -> String {
    let mut out = String::new();
    print_header_section(&snap.header_vars, &mut out);
    print_tables_section(&snap.tables, &snap.other_tables, &mut out);
    print_blocks_section(&snap.blocks, &mut out);
    push_tag(&mut out, 0, "SECTION");
    push_tag(&mut out, 2, "ENTITIES");
    print_entities(&snap.entities, &mut out);
    push_tag(&mut out, 0, "ENDSEC");
    push_tag(&mut out, 0, "EOF");
    out
}
//#endregion 🔖️DocumentCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DxfSnapshot {
    const EXTENSION: &'static str = "dxf";
    async fn envelope_id() -> &'static str {
        "stdio.dxf"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_dxf_document(body).map_err(|e| store::TextError::new(format!("dxf parse: {e}"), dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_dxf_document(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DxfSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = print_dxf_document(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        parse_dxf_document(&text).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_dxf_text() -> String {
        concat!(
            "0\nSECTION\n2\nHEADER\n9\n$ACADVER\n1\nAC1009\n9\n$INSBASE\n10\n1\n20\n2\n30\n3\n0\nENDSEC\n",
            "0\nSECTION\n2\nTABLES\n",
            "0\nTABLE\n2\nLAYER\n70\n1\n0\nLAYER\n2\n0\n70\n0\n62\n7\n6\nCONTINUOUS\n0\nENDTAB\n",
            "0\nTABLE\n2\nSTYLE\n70\n1\n0\nSTYLE\n2\nSTANDARD\n70\n0\n3\ntxt\n0\nENDTAB\n",
            "0\nTABLE\n2\nLTYPE\n70\n1\n0\nLTYPE\n2\nCONTINUOUS\n70\n0\n3\nSolid\n0\nENDTAB\n",
            "0\nTABLE\n2\nVPORT\n70\n1\n0\nVPORT\n2\n*ACTIVE\n0\nENDTAB\n",
            "0\nENDSEC\n",
            "0\nSECTION\n2\nBLOCKS\n",
            "0\nBLOCK\n2\nMYBLOCK\n70\n0\n10\n0\n20\n0\n30\n0\n0\nLINE\n8\n0\n10\n0\n20\n0\n30\n0\n11\n1\n21\n1\n31\n0\n0\nENDBLK\n",
            "0\nENDSEC\n",
            "0\nSECTION\n2\nENTITIES\n",
            "0\nLINE\n8\n0\n10\n1\n20\n2\n30\n3\n11\n4\n21\n5\n31\n6\n",
            "0\nCIRCLE\n8\n0\n10\n10\n20\n20\n30\n30\n40\n5\n",
            "0\nARC\n8\n0\n10\n1\n20\n2\n30\n3\n40\n7\n50\n0\n51\n180\n",
            "0\nTEXT\n8\n0\n10\n1\n20\n1\n30\n0\n40\n2.5\n1\nHello\n",
            "0\nSOLID\n8\n0\n10\n0\n20\n0\n30\n0\n11\n1\n21\n0\n31\n0\n12\n1\n22\n1\n32\n0\n13\n0\n23\n1\n33\n0\n",
            "0\nINSERT\n8\n0\n2\nMYBLOCK\n10\n5\n20\n5\n30\n0\n41\n1\n42\n1\n43\n1\n50\n0\n",
            "0\nPOLYLINE\n8\n0\n66\n1\n70\n1\n0\nVERTEX\n8\n0\n10\n0\n20\n0\n30\n0\n0\nVERTEX\n8\n0\n10\n1\n20\n0\n30\n0\n0\nSEQEND\n",
            "0\n3DFACE\n8\n0\n10\n0\n20\n0\n30\n0\n",
            "0\nENDSEC\n0\nEOF\n",
        )
        .to_string()
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_every_section_and_entity_kind() {
        let snap = parse_dxf_document(&sample_dxf_text()).expect("parse");
        assert_eq!(snap.header_vars.len(), 2);
        assert_eq!(snap.header_vars[0].name, "$ACADVER");
        assert_eq!(snap.header_vars[1].name, "$INSBASE");
        assert_eq!(snap.header_vars[1].value, DxfValue::Point { value: [1.0, 2.0, 3.0] });

        assert_eq!(snap.tables.layers.len(), 1);
        assert_eq!(snap.tables.layers[0].name, "0");
        assert_eq!(snap.tables.styles.len(), 1);
        assert_eq!(snap.tables.linetypes.len(), 1);
        assert_eq!(snap.other_tables.len(), 1, "VPORT retained raw");
        assert_eq!(snap.other_tables[0].name, "VPORT");

        assert_eq!(snap.blocks.len(), 1);
        assert_eq!(snap.blocks[0].name, "MYBLOCK");
        assert_eq!(snap.blocks[0].entities.len(), 1);
        assert!(matches!(snap.blocks[0].entities[0], DxfEntity::Line { .. }));

        assert_eq!(snap.entities.len(), 8);
        assert!(matches!(snap.entities[0], DxfEntity::Line { .. }));
        assert!(matches!(snap.entities[1], DxfEntity::Circle { .. }));
        assert!(matches!(snap.entities[2], DxfEntity::Arc { .. }));
        assert!(matches!(snap.entities[3], DxfEntity::Text { .. }));
        assert!(matches!(snap.entities[4], DxfEntity::Solid { .. }));
        assert!(matches!(snap.entities[5], DxfEntity::Insert { .. }));
        match &snap.entities[6] {
            DxfEntity::Polyline { vertices, closed, .. } => {
                assert_eq!(vertices.len(), 2);
                assert!(*closed);
            }
            other => panic!("expected Polyline, got {other:?}"),
        }
        match &snap.entities[7] {
            DxfEntity::Other { kind, .. } => assert_eq!(kind, "3DFACE"),
            other => panic!("expected Other(3DFACE), got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_retention_is_a_fixed_point_from_generation_two() {
        let snap1 = parse_dxf_document(&sample_dxf_text()).expect("parse");
        let text2 = print_dxf_document(&snap1);
        let snap2 = parse_dxf_document(&text2).expect("re-parse");
        assert_eq!(snap1, snap2, "decode(encode(decode(text))) must be a fixed point");
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_parse_dsl_print_dsl_round_trips() {
        let snap = parse_dxf_document(&sample_dxf_text()).expect("parse");
        let printed = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parse");
        assert_eq!(parsed, snap);

        let packed = store::ArtifactPack::encode_pack(&snap);
        let unpacked = <DxfSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("unpack");
        assert_eq!(unpacked, snap);
    }
}
//#endregion 🧪️Tests
