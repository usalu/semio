//! 🧬️ SemioCadSnapshot — layers/blocks/entities, informed by dxf r12's typed entity list, dwg's
//! `DwgDrawing`/`DwgEntity`/`DwgGeometry`, and the 📐️cad plugin's domain artifact (master plan
//! "Subset snapshot cores" table, `cad` row). `CadEntity` carries the full 9-variant vocabulary
//! (Line/Arc/Circle/Ellipse/Polyline/Text/Insert/Solid/Dimension).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOCAD_DOCUMENT_SCHEMA: &str = "stdio.semio.cad";
//#endregion 🔖️Ids

//#region 🔖️Entity
/// 📐️ Owned by the `cad` subset — a WEAK value struct (see `🔺️diff`'s module doc comment): whole-
/// value replaced in diffs, never sub-diffed, same treatment as `BcfCamera`/`XlsxCellValue`.
///
/// 🧪️ `Default` (with `Line` as the zero-length degenerate default) is required here, not for any
/// domain reason, but to satisfy a spurious `T: Default` bound the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s derived `Deserialize` impl infers from its own
/// `#[serde(default)]`-annotated `added: Vec<T>` field (same known `serde_derive` quirk bcf's
/// local `NamedTripleDiff` copy already worked around via an explicit `#[serde(bound(...))]—`
/// the SHARED copy under `⚙️engine/🧰️triples` is missing that override; noted as a shared-infra
/// gap for the closer, not fixed here per this ticket's write-scope rules).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CadEntity {
    Line { a: SemioPoint2, b: SemioPoint2 },
    Arc { center: SemioPoint2, radius: f64, start_angle: f64, end_angle: f64 },
    Circle { center: SemioPoint2, radius: f64 },
    Ellipse { center: SemioPoint2, major_axis_end: SemioPoint2, ratio: f64, start_param: f64, end_param: f64 },
    Polyline { vertices: Vec<SemioPoint2>, closed: bool },
    Text { position: SemioPoint2, height: f64, rotation: f64, content: String },
    Insert { block_name: String, insertion_point: SemioPoint2, scale: SemioPoint2, rotation: f64 },
    Solid { p1: SemioPoint2, p2: SemioPoint2, p3: SemioPoint2, p4: SemioPoint2 },
    Dimension { def_point: SemioPoint2, text_position: SemioPoint2, measurement: f64, text: String },
}

/// 🧭️ Manual impl (not `#[derive(Default)]`) -- `Default` on an enum requires a UNIT default
/// variant, but every `CadEntity` variant carries fields, so the derive attribute is structurally
/// rejected here; hand-written zero-length `Line` matches what a derive-with-unit-variant would
/// have produced field-by-field if it were allowed. See this type's own doc comment above for WHY
/// `Default` is needed at all (the shared `engine::triples` spurious-bound workaround).
impl Default for CadEntity {
    async fn default() -> Self {
        CadEntity::Line { a: SemioPoint2::default(), b: SemioPoint2::default() }
    }
}
//#endregion 🔖️Entity

//#region 🔖️Layer
/// 🗂️ Name-keyed (dxf `TABLES/LAYER`-style) — strong entity, own per-field diff. `Default` is the
/// same spurious-bound workaround `CadEntity` documents above.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadLayer {
    pub name: String,
    pub color_index: i32,
    pub line_type: String,
    pub visible: bool,
}
//#endregion 🔖️Layer

//#region 🔖️EntityRecord
/// 🏷️ One placed entity — `handle` is the id key (dxf group code 5); `layer` names the owning
/// `CadLayer` by reference. Referential invariants (dangling `layer`/`Insert.block_name`) are
/// checked by the composer's `SemioCadValidator`, not enforced structurally here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadEntityRecord {
    pub handle: String,
    pub layer: String,
    pub entity: CadEntity,
}
//#endregion 🔖️EntityRecord

//#region 🔖️Block
/// 📦️ Name-keyed (dxf `BLOCKS` section) — strong entity; `entities` is its own nested id-keyed
/// collection (same shape as the top-level `entities`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadBlock {
    pub name: String,
    pub base_point: SemioPoint2,
    #[serde(default)]
    pub entities: Vec<CadEntityRecord>,
}
//#endregion 🔖️Block

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.cad")]
pub struct SemioCadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub layers: Vec<CadLayer>,
    #[state(artifact)]
    #[serde(default)]
    pub blocks: Vec<CadBlock>,
    #[state(artifact)]
    #[serde(default)]
    pub entities: Vec<CadEntityRecord>,
}

impl Default for SemioCadSnapshot {
    async fn default() -> Self {
        Self { schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(), layers: Vec::new(), blocks: Vec::new(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION cad wave (following the
/// flow/brep pilots' proven template, `ws-codec-workflow-report.md`/`ws-codec-brep-report.md`):
/// real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets, duplicated here (not imported from
/// `schema::diff`) to keep `snapshot` — the base type `diff`/`mutations` both depend ON — free of a
/// reverse dependency on either sibling facet.
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was reconsidered now that `SemioPoint2` derives
/// `dsl::DslRecord`. Still blocked: `CadEntity` is a data-carrying TAGGED ENUM (9 variants, each
/// with a DIFFERENT field set) — the derive machinery's `DslVariants`/`DslEnum` support targets
/// one-spec-per-variant BINARY layouts, not a single alternated TEXT grammar production set (the
/// `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` gap brep's wave first hit).
/// Hand-rolled instead, matching the established hex/bracket convention this subset's own `🔺️diff`
/// facet already uses for exactly this enum.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
async fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
async fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
async fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
async fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

async fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", p.x, p.y)
}
async fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}

/// 📐️ `L`ine/`A`rc/`C`ircle/`E`llipse/`P`olyline/`T`ext/`I`nsert/`S`olid/`D`imension — single-letter
/// tag prefix, same convention this subset's own `🔺️diff/🦀️component.rs`'s `enc_entity` uses
/// (duplicated here, field-for-field).
async fn enc_entity(e: &CadEntity) -> String {
    match e {
        CadEntity::Line { a, b } => format!("L[{},{}]", enc_point2(a), enc_point2(b)),
        CadEntity::Arc { center, radius, start_angle, end_angle } => format!("A[{},{},{},{}]", enc_point2(center), radius, start_angle, end_angle),
        CadEntity::Circle { center, radius } => format!("C[{},{}]", enc_point2(center), radius),
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => {
            format!("E[{},{},{},{},{}]", enc_point2(center), enc_point2(major_axis_end), ratio, start_param, end_param)
        }
        CadEntity::Polyline { vertices, closed } => format!("P[{},{}]", enc_list(vertices, enc_point2), enc_bool(*closed)),
        CadEntity::Text { position, height, rotation, content } => format!("T[{},{},{},{}]", enc_point2(position), height, rotation, enc_str(content)),
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => format!("I[{},{},{},{}]", enc_str(block_name), enc_point2(insertion_point), enc_point2(scale), rotation),
        CadEntity::Solid { p1, p2, p3, p4 } => format!("S[{},{},{},{}]", enc_point2(p1), enc_point2(p2), enc_point2(p3), enc_point2(p4)),
        CadEntity::Dimension { def_point, text_position, measurement, text } => format!("D[{},{},{},{}]", enc_point2(def_point), enc_point2(text_position), measurement, enc_str(text)),
    }
}
async fn dec_entity(s: &str) -> Result<CadEntity, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "L" => {
            let [a, b] = parts.as_slice() else { return Err(format!("line: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Line { a: dec_point2(a)?, b: dec_point2(b)? })
        }
        "A" => {
            let [center, radius, start_angle, end_angle] = parts.as_slice() else { return Err(format!("arc: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Arc { center: dec_point2(center)?, radius: parse_f64(radius)?, start_angle: parse_f64(start_angle)?, end_angle: parse_f64(end_angle)? })
        }
        "C" => {
            let [center, radius] = parts.as_slice() else { return Err(format!("circle: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Circle { center: dec_point2(center)?, radius: parse_f64(radius)? })
        }
        "E" => {
            let [center, major_axis_end, ratio, start_param, end_param] = parts.as_slice() else { return Err(format!("ellipse: expected 5 fields, got {}", parts.len())) };
            Ok(CadEntity::Ellipse { center: dec_point2(center)?, major_axis_end: dec_point2(major_axis_end)?, ratio: parse_f64(ratio)?, start_param: parse_f64(start_param)?, end_param: parse_f64(end_param)? })
        }
        "P" => {
            let [vertices, closed] = parts.as_slice() else { return Err(format!("polyline: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Polyline { vertices: dec_list(vertices, dec_point2)?, closed: parse_bool(closed)? })
        }
        "T" => {
            let [position, height, rotation, content] = parts.as_slice() else { return Err(format!("text: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Text { position: dec_point2(position)?, height: parse_f64(height)?, rotation: parse_f64(rotation)?, content: dec_str(content)? })
        }
        "I" => {
            let [block_name, insertion_point, scale, rotation] = parts.as_slice() else { return Err(format!("insert: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Insert { block_name: dec_str(block_name)?, insertion_point: dec_point2(insertion_point)?, scale: dec_point2(scale)?, rotation: parse_f64(rotation)? })
        }
        "S" => {
            let [p1, p2, p3, p4] = parts.as_slice() else { return Err(format!("solid: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Solid { p1: dec_point2(p1)?, p2: dec_point2(p2)?, p3: dec_point2(p3)?, p4: dec_point2(p4)? })
        }
        "D" => {
            let [def_point, text_position, measurement, text] = parts.as_slice() else { return Err(format!("dimension: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Dimension { def_point: dec_point2(def_point)?, text_position: dec_point2(text_position)?, measurement: parse_f64(measurement)?, text: dec_str(text)? })
        }
        other => Err(format!("entity: unknown tag {other:?}")),
    }
}

async fn enc_layer(l: &CadLayer) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.color_index, enc_str(&l.line_type), enc_bool(l.visible))
}
async fn dec_layer(s: &str) -> Result<CadLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color_index, line_type, visible] = parts.as_slice() else { return Err(format!("layer: expected 4 fields, got {}", parts.len())) };
    Ok(CadLayer { name: dec_str(name)?, color_index: parse_i32(color_index)?, line_type: dec_str(line_type)?, visible: parse_bool(visible)? })
}

async fn enc_entity_record(r: &CadEntityRecord) -> String {
    format!("[{},{},{}]", enc_str(&r.handle), enc_str(&r.layer), enc_entity(&r.entity))
}
async fn dec_entity_record(s: &str) -> Result<CadEntityRecord, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [handle, layer, entity] = parts.as_slice() else { return Err(format!("entity record: expected 3 fields, got {}", parts.len())) };
    Ok(CadEntityRecord { handle: dec_str(handle)?, layer: dec_str(layer)?, entity: dec_entity(entity)? })
}

async fn enc_block(b: &CadBlock) -> String {
    format!("[{},{},{}]", enc_str(&b.name), enc_point2(&b.base_point), enc_list(&b.entities, enc_entity_record))
}
async fn dec_block(s: &str) -> Result<CadBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities] = parts.as_slice() else { return Err(format!("block: expected 3 fields, got {}", parts.len())) };
    Ok(CadBlock { name: dec_str(name)?, base_point: dec_point2(base_point)?, entities: dec_list(entities, dec_entity_record)? })
}

/// 📄️ The real structured text body: four lines — `schema=<hex>`, `layers=[...]`, `blocks=[...]`,
/// `entities=[...]` — matching the grammar's `document = artifact-mark schema-line layers-line
/// blocks-line entities-line`. Newlines are pure lexer trivia in the shared dialect, so this is
/// genuinely recognizable by `dsl::Recognizer`, not merely readable.
async fn print_cad_snapshot_body(s: &SemioCadSnapshot) -> String {
    format!(
        "schema={}\nlayers=[{}]\nblocks=[{}]\nentities=[{}]",
        enc_str(&s.schema),
        s.layers.iter().map(enc_layer).collect::<Vec<_>>().join(","),
        s.blocks.iter().map(enc_block).collect::<Vec<_>>().join(","),
        s.entities.iter().map(enc_entity_record).collect::<Vec<_>>().join(","),
    )
}
async fn parse_cad_snapshot_body(body: &str) -> Result<SemioCadSnapshot, String> {
    let mut schema = None;
    let mut layers = Vec::new();
    let mut blocks = Vec::new();
    let mut entities = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("layers=") {
            layers = dec_list(rest, dec_layer)?;
        } else if let Some(rest) = line.strip_prefix("blocks=") {
            blocks = dec_list(rest, dec_block)?;
        } else if let Some(rest) = line.strip_prefix("entities=") {
            entities = dec_list(rest, dec_entity_record)?;
        } else {
            return Err(format!("cad snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "cad snapshot: missing schema line".to_string())?;
    Ok(SemioCadSnapshot { schema, layers, blocks, entities })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`/`stdio.semio.brep`'s upgraded
/// `OpBinary`/`DiffCodec` reuse) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut.
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
async fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}
async fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(SemioPoint2 { x, y })
}
async fn write_point2_vec(out: &mut Vec<u8>, v: &[SemioPoint2]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for p in v {
        write_point2(out, p);
    }
}
async fn read_point2_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioPoint2>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(read_point2(reader)?);
    }
    Ok(v)
}
async fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}
async fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

/// 🏷️ `CadEntity` variant tags — 0=Line, 1=Arc, 2=Circle, 3=Ellipse, 4=Polyline, 5=Text, 6=Insert,
/// 7=Solid, 8=Dimension (declaration order).
async fn write_entity(out: &mut Vec<u8>, e: &CadEntity) {
    match e {
        CadEntity::Line { a, b } => {
            out.push(0);
            write_point2(out, a);
            write_point2(out, b);
        }
        CadEntity::Arc { center, radius, start_angle, end_angle } => {
            out.push(1);
            write_point2(out, center);
            out.extend_from_slice(&radius.to_le_bytes());
            out.extend_from_slice(&start_angle.to_le_bytes());
            out.extend_from_slice(&end_angle.to_le_bytes());
        }
        CadEntity::Circle { center, radius } => {
            out.push(2);
            write_point2(out, center);
            out.extend_from_slice(&radius.to_le_bytes());
        }
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => {
            out.push(3);
            write_point2(out, center);
            write_point2(out, major_axis_end);
            out.extend_from_slice(&ratio.to_le_bytes());
            out.extend_from_slice(&start_param.to_le_bytes());
            out.extend_from_slice(&end_param.to_le_bytes());
        }
        CadEntity::Polyline { vertices, closed } => {
            out.push(4);
            write_point2_vec(out, vertices);
            write_bool(out, *closed);
        }
        CadEntity::Text { position, height, rotation, content } => {
            out.push(5);
            write_point2(out, position);
            out.extend_from_slice(&height.to_le_bytes());
            out.extend_from_slice(&rotation.to_le_bytes());
            write_str_lp(out, content);
        }
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => {
            out.push(6);
            write_str_lp(out, block_name);
            write_point2(out, insertion_point);
            write_point2(out, scale);
            out.extend_from_slice(&rotation.to_le_bytes());
        }
        CadEntity::Solid { p1, p2, p3, p4 } => {
            out.push(7);
            write_point2(out, p1);
            write_point2(out, p2);
            write_point2(out, p3);
            write_point2(out, p4);
        }
        CadEntity::Dimension { def_point, text_position, measurement, text } => {
            out.push(8);
            write_point2(out, def_point);
            write_point2(out, text_position);
            out.extend_from_slice(&measurement.to_le_bytes());
            write_str_lp(out, text);
        }
    }
}
async fn read_entity(reader: &mut store::ByteReader<'_>) -> Result<CadEntity, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(CadEntity::Line { a: read_point2(reader)?, b: read_point2(reader)? }),
        1 => Ok(CadEntity::Arc { center: read_point2(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())?, start_angle: reader.read_f64_le().map_err(|e| e.to_string())?, end_angle: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(CadEntity::Circle { center: read_point2(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        3 => Ok(CadEntity::Ellipse {
            center: read_point2(reader)?,
            major_axis_end: read_point2(reader)?,
            ratio: reader.read_f64_le().map_err(|e| e.to_string())?,
            start_param: reader.read_f64_le().map_err(|e| e.to_string())?,
            end_param: reader.read_f64_le().map_err(|e| e.to_string())?,
        }),
        4 => Ok(CadEntity::Polyline { vertices: read_point2_vec(reader)?, closed: read_bool(reader)? }),
        5 => Ok(CadEntity::Text { position: read_point2(reader)?, height: reader.read_f64_le().map_err(|e| e.to_string())?, rotation: reader.read_f64_le().map_err(|e| e.to_string())?, content: read_str_lp(reader)? }),
        6 => Ok(CadEntity::Insert { block_name: read_str_lp(reader)?, insertion_point: read_point2(reader)?, scale: read_point2(reader)?, rotation: reader.read_f64_le().map_err(|e| e.to_string())? }),
        7 => Ok(CadEntity::Solid { p1: read_point2(reader)?, p2: read_point2(reader)?, p3: read_point2(reader)?, p4: read_point2(reader)? }),
        8 => Ok(CadEntity::Dimension { def_point: read_point2(reader)?, text_position: read_point2(reader)?, measurement: reader.read_f64_le().map_err(|e| e.to_string())?, text: read_str_lp(reader)? }),
        other => Err(format!("entity: unknown binary tag {other}")),
    }
}

async fn write_layer(out: &mut Vec<u8>, l: &CadLayer) {
    write_str_lp(out, &l.name);
    store::pack_rt::write_varint_u64(out, l.color_index as u64);
    write_str_lp(out, &l.line_type);
    write_bool(out, l.visible);
}
async fn read_layer(reader: &mut store::ByteReader<'_>) -> Result<CadLayer, String> {
    let name = read_str_lp(reader)?;
    let color_index = reader.read_varint_u64().map_err(|e| e.to_string())? as i32;
    let line_type = read_str_lp(reader)?;
    let visible = read_bool(reader)?;
    Ok(CadLayer { name, color_index, line_type, visible })
}

async fn write_entity_record(out: &mut Vec<u8>, r: &CadEntityRecord) {
    write_str_lp(out, &r.handle);
    write_str_lp(out, &r.layer);
    write_entity(out, &r.entity);
}
async fn read_entity_record(reader: &mut store::ByteReader<'_>) -> Result<CadEntityRecord, String> {
    Ok(CadEntityRecord { handle: read_str_lp(reader)?, layer: read_str_lp(reader)?, entity: read_entity(reader)? })
}

async fn write_block(out: &mut Vec<u8>, b: &CadBlock) {
    write_str_lp(out, &b.name);
    write_point2(out, &b.base_point);
    store::pack_rt::write_varint_u64(out, b.entities.len() as u64);
    for r in &b.entities {
        write_entity_record(out, r);
    }
}
async fn read_block(reader: &mut store::ByteReader<'_>) -> Result<CadBlock, String> {
    let name = read_str_lp(reader)?;
    let base_point = read_point2(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entities = Vec::with_capacity(n as usize);
    for _ in 0..n {
        entities.push(read_entity_record(reader)?);
    }
    Ok(CadBlock { name, base_point, entities })
}

async fn encode_cad_snapshot_binary(s: &SemioCadSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.layers.len() as u64);
    for l in &s.layers {
        write_layer(&mut out, l);
    }
    store::pack_rt::write_varint_u64(&mut out, s.blocks.len() as u64);
    for b in &s.blocks {
        write_block(&mut out, b);
    }
    store::pack_rt::write_varint_u64(&mut out, s.entities.len() as u64);
    for r in &s.entities {
        write_entity_record(&mut out, r);
    }
    out
}
async fn decode_cad_snapshot_binary(bytes: &[u8]) -> Result<SemioCadSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let layer_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layers = Vec::with_capacity(layer_count as usize);
    for _ in 0..layer_count {
        layers.push(read_layer(&mut reader)?);
    }
    let block_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for _ in 0..block_count {
        blocks.push(read_block(&mut reader)?);
    }
    let entity_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entities = Vec::with_capacity(entity_count as usize);
    for _ in 0..entity_count {
        entities.push(read_entity_record(&mut reader)?);
    }
    Ok(SemioCadSnapshot { schema, layers, blocks, entities })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (cad wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow/brep pilots' proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioCadSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIOCAD_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_cad_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_cad_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioCadSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_cad_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_cad_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.cad` document — a small floor-plan-shaped drawing exercising every
/// collection AND every `CadEntity` variant at least once (a `door` block with a nested `Line`,
/// plus a top-level `Arc`/`Circle`/`Ellipse`/`Polyline`/`Text`/`Insert`/`Solid`/`Dimension`). Single
/// source of truth for `📚️examples/📐️drawing/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) async fn demo_cad_snapshot() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "dim".into(), color_index: 1, line_type: "DASHED".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "door".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "be1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 0.0 } } }],
        }],
        entities: vec![
            CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.0, start_angle: 0.0, end_angle: 180.0 } },
            CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 5.0, y: 5.0 }, radius: 2.0 } },
            CadEntityRecord { handle: "h3".into(), layer: "0".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 3.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.283 } },
            CadEntityRecord { handle: "h4".into(), layer: "0".into(), entity: CadEntity::Polyline { vertices: vec![SemioPoint2 { x: 0.0, y: 0.0 }, SemioPoint2 { x: 1.0, y: 1.0 }, SemioPoint2 { x: 2.0, y: 0.0 }], closed: true } },
            CadEntityRecord { handle: "h5".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 0.0, y: 0.0 }, height: 2.5, rotation: 0.0, content: "Room 101".into() } },
            CadEntityRecord { handle: "h6".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2 { x: 10.0, y: 10.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 90.0 } },
            CadEntityRecord { handle: "h7".into(), layer: "0".into(), entity: CadEntity::Solid { p1: SemioPoint2 { x: 0.0, y: 0.0 }, p2: SemioPoint2 { x: 1.0, y: 0.0 }, p3: SemioPoint2 { x: 1.0, y: 1.0 }, p4: SemioPoint2 { x: 0.0, y: 1.0 } } },
            CadEntityRecord { handle: "h8".into(), layer: "dim".into(), entity: CadEntity::Dimension { def_point: SemioPoint2 { x: 0.0, y: 0.0 }, text_position: SemioPoint2 { x: 1.0, y: 1.0 }, measurement: 4.2, text: "4.20m".into() } },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn populated_snapshot() -> SemioCadSnapshot {
        SemioCadSnapshot {
            schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
            blocks: vec![CadBlock {
                name: "door".into(),
                base_point: SemioPoint2 { x: 0.0, y: 0.0 },
                entities: vec![CadEntityRecord { handle: "b1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 0.0 } } }],
            }],
            entities: vec![
                CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.5 } },
                CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2 { x: 5.0, y: 5.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 90.0 } },
            ],
        }
    }

    #[test]
    async fn json_pack_round_trips() {
        let snap = SemioCadSnapshot::default();
        let bytes = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioCadSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    async fn dsl_text_round_trips() {
        let snap = SemioCadSnapshot::default();
        let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ Law 5 — `codec_retention_law`: decode(encode(x)) == x on a fully populated snapshot
    /// (layers/blocks/nested-block-entities/top-level entities incl. `Insert`), both facets.
    #[test]
    async fn codec_retention_law() {
        let snap = populated_snapshot();
        let bytes = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let via_pack = <SemioCadSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(via_pack, snap);

        let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let via_dsl = <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse dsl");
        assert_eq!(via_dsl, snap);
    }

    /// 🧪️ Every `CadEntity` variant (all 9) round-trips through both the pack binary and the dsl
    /// text codec — the demo fixture used by the fixture-honesty conformance law.
    #[test]
    async fn demo_snapshot_round_trips_pack_and_dsl() {
        let demo = demo_cad_snapshot();
        let packed = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&demo);
        assert_eq!(<SemioCadSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), demo);
        let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&demo);
        assert_eq!(<SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), demo);
    }
}
//#endregion 🔖️Tests
