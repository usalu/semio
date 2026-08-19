//! 🔺️ SemioCadDiff — handcrafted sparse diff over `SemioCadSnapshot`. No
//! `replacement: Option<SemioCadSnapshot>` full-replace slot — even `SetSnapshot`'s diff is the
//! sparse field-by-field `SemioCadDiff::between(base, next)`.
//!
//! `layers`/`blocks`/`entities` (name/handle-keyed) are diffed via the SHARED
//! `engine::triples::NamedTripleDiff<K,D,T>` (per the w1b type-ownership brief — do NOT redefine
//! this per-subset, unlike bcf/docx which predate the shared module). `blocks[].entities` reuses
//! the SAME `CadEntitiesDiff` alias one level deeper (a block is just a second id-keyed entity
//! collection, same shape as the top-level one). `CadEntity` itself is a WEAK value (whole-value
//! replaced, never sub-diffed — same treatment `BcfCamera`/`XlsxCellValue` get), so
//! `CadEntityRecordDiff.entity` is a plain `Option<CadEntity>`, not a nested diff type.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{dec_named_triple, enc_named_triple, split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️DiffTypes
pub type CadLayersDiff = NamedTripleDiff<String, CadLayerDiff, CadLayer>;
pub type CadBlocksDiff = NamedTripleDiff<String, CadBlockDiff, CadBlock>;
pub type CadEntitiesDiff = NamedTripleDiff<String, CadEntityRecordDiff, CadEntityRecord>;

/// 🔺️ Per-layer sparse diff — all 3 mutable fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadLayerDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

/// 🔺️ Per-block sparse diff — `base_point` scalar; `entities` a nested id-keyed triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadBlockDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_point: Option<SemioPoint2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<CadEntitiesDiff>,
}

/// 🔺️ Per-entity-record sparse diff — `layer` is a scalar patch; `entity` is whole-value replaced
/// (weak value struct, per this module's doc comment).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadEntityRecordDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<CadEntity>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioCadDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<CadLayersDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<CadBlocksDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<CadEntitiesDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/id-keyed collection algebra (apply/between/inverse/absorb), generic over key `K`, item
/// `T`, per-field diff `D`. Operates on the SHARED `engine::triples::NamedTripleDiff` type — this
/// artifact's own copy of the algorithm (cross-artifact algorithm imports would be architecturally
/// wrong, same rationale bcf's own copy documents), not a re-definition of the data shape.
async fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    for item in &diff.added {
        items.push(item.clone());
    }
}

async fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

async fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Key-identity absorb (not position) — a `d2`-removal of a `d1`-added key annihilates the add;
/// a `d2`-modify of a `d1`-added key patches into the carried payload; everything else composes on
/// the shared key space. Mirrors bcf's `absorb_named` (same canonical cases, B-R7).
async fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️WrapHelpers
/// 🧭️ Lowers a per-layer leaf diff into a full `SemioCadDiff` (mirrors bcf's `wrap_topic_diff`).
pub async fn wrap_layer_diff(name: &str, diff: CadLayerDiff) -> SemioCadDiff {
    SemioCadDiff { layers: Some(CadLayersDiff { removed: Vec::new(), modified: vec![NamedModified { key: name.to_string(), diff }], added: Vec::new() }), blocks: None, entities: None }
}

/// 🧭️ Lowers a per-block leaf diff into a full `SemioCadDiff`.
pub async fn wrap_block_diff(name: &str, diff: CadBlockDiff) -> SemioCadDiff {
    SemioCadDiff { layers: None, blocks: Some(CadBlocksDiff { removed: Vec::new(), modified: vec![NamedModified { key: name.to_string(), diff }], added: Vec::new() }), entities: None }
}

/// 🧭️ Lowers a per-top-level-entity leaf diff into a full `SemioCadDiff`.
pub async fn wrap_entity_diff(handle: &str, diff: CadEntityRecordDiff) -> SemioCadDiff {
    SemioCadDiff { layers: None, blocks: None, entities: Some(CadEntitiesDiff { removed: Vec::new(), modified: vec![NamedModified { key: handle.to_string(), diff }], added: Vec::new() }) }
}

/// 🧭️ Lowers a per-block-entity leaf diff (inside block `block_name`) into a full `SemioCadDiff`.
pub async fn wrap_block_entity_diff(block_name: &str, handle: &str, diff: CadEntityRecordDiff) -> SemioCadDiff {
    wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(CadEntitiesDiff { removed: Vec::new(), modified: vec![NamedModified { key: handle.to_string(), diff }], added: Vec::new() }) })
}
//#endregion 🔖️WrapHelpers

//#region 🔖️Apply
impl MutationDiff<SemioCadSnapshot> for SemioCadDiff {
    async fn apply(&self, base: &SemioCadSnapshot) -> protocol::MutationApplyResult<SemioCadSnapshot> {
        let mut next = base.clone();
        if let Some(ld) = &self.layers {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.layers, ld, |layer| layer.name.clone(), |layer| layer.name.clone(), ["layers"])?;
            apply_named(&mut next.layers, ld, |l| l.name.clone(), apply_layer);
        }
        if let Some(bd) = &self.blocks {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.blocks, bd, |block| block.name.clone(), |block| block.name.clone(), ["blocks"])?;
            apply_named(&mut next.blocks, bd, |b| b.name.clone(), apply_block);
        }
        if let Some(ed) = &self.entities {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.entities, ed, |entity| entity.handle.clone(), |entity| entity.handle.clone(), ["entities"])?;
            apply_named(&mut next.entities, ed, |e| e.handle.clone(), apply_entity_record);
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        self.layers = match (self.layers.take(), other.layers) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |l| l.name.clone(), absorb_layer_diff, apply_layer)),
        };
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |bl| bl.name.clone(), absorb_block_diff, apply_block)),
        };
        self.entities = match (self.entities.take(), other.entities) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |e| e.handle.clone(), absorb_entity_record_diff, apply_entity_record)),
        };
    }
}

async fn apply_layer(layer: &mut CadLayer, diff: &CadLayerDiff) {
    if let Some(v) = &diff.color_index {
        layer.color_index = *v;
    }
    if let Some(v) = &diff.line_type {
        layer.line_type = v.clone();
    }
    if let Some(v) = &diff.visible {
        layer.visible = *v;
    }
}

async fn apply_block(block: &mut CadBlock, diff: &CadBlockDiff) {
    if let Some(v) = &diff.base_point {
        block.base_point = *v;
    }
    if let Some(ed) = &diff.entities {
        apply_named(&mut block.entities, ed, |e| e.handle.clone(), apply_entity_record);
    }
}

async fn apply_entity_record(rec: &mut CadEntityRecord, diff: &CadEntityRecordDiff) {
    if let Some(v) = &diff.layer {
        rec.layer = v.clone();
    }
    if let Some(v) = &diff.entity {
        rec.entity = v.clone();
    }
}

async fn absorb_layer_diff(mut a: CadLayerDiff, b: CadLayerDiff) -> CadLayerDiff {
    if b.color_index.is_some() {
        a.color_index = b.color_index;
    }
    if b.line_type.is_some() {
        a.line_type = b.line_type;
    }
    if b.visible.is_some() {
        a.visible = b.visible;
    }
    a
}

async fn absorb_block_diff(mut a: CadBlockDiff, b: CadBlockDiff) -> CadBlockDiff {
    if b.base_point.is_some() {
        a.base_point = b.base_point;
    }
    a.entities = match (a.entities.take(), b.entities) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(absorb_named(x, y, |e| e.handle.clone(), absorb_entity_record_diff, apply_entity_record)),
    };
    a
}

async fn absorb_entity_record_diff(mut a: CadEntityRecordDiff, b: CadEntityRecordDiff) -> CadEntityRecordDiff {
    if b.layer.is_some() {
        a.layer = b.layer;
    }
    if b.entity.is_some() {
        a.entity = b.entity;
    }
    a
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioCadSnapshot> for SemioCadDiff {
    async fn inverse(&self, base: &SemioCadSnapshot) -> Self {
        SemioCadDiff {
            layers: self.layers.as_ref().map(|d| inverse_named(&base.layers, d, |l| l.name.clone(), inverse_layer)),
            blocks: self.blocks.as_ref().map(|d| inverse_named(&base.blocks, d, |b| b.name.clone(), inverse_block)),
            entities: self.entities.as_ref().map(|d| inverse_named(&base.entities, d, |e| e.handle.clone(), inverse_entity_record)),
        }
    }

    async fn between(base: &SemioCadSnapshot, other: &SemioCadSnapshot) -> Self {
        SemioCadDiff {
            layers: between_named(&base.layers, &other.layers, |l| l.name.clone(), between_layer),
            blocks: between_named(&base.blocks, &other.blocks, |b| b.name.clone(), between_block),
            entities: between_named(&base.entities, &other.entities, |e| e.handle.clone(), between_entity_record),
        }
    }

    async fn is_empty(&self) -> bool {
        self.layers.is_none() && self.blocks.is_none() && self.entities.is_none()
    }
}

async fn inverse_layer(base: &CadLayer, diff: &CadLayerDiff) -> CadLayerDiff {
    CadLayerDiff { color_index: diff.color_index.as_ref().map(|_| base.color_index), line_type: diff.line_type.as_ref().map(|_| base.line_type.clone()), visible: diff.visible.as_ref().map(|_| base.visible) }
}

async fn inverse_block(base: &CadBlock, diff: &CadBlockDiff) -> CadBlockDiff {
    CadBlockDiff { base_point: diff.base_point.as_ref().map(|_| base.base_point), entities: diff.entities.as_ref().map(|d| inverse_named(&base.entities, d, |e| e.handle.clone(), inverse_entity_record)) }
}

async fn inverse_entity_record(base: &CadEntityRecord, diff: &CadEntityRecordDiff) -> CadEntityRecordDiff {
    CadEntityRecordDiff { layer: diff.layer.as_ref().map(|_| base.layer.clone()), entity: diff.entity.as_ref().map(|_| base.entity.clone()) }
}

async fn between_layer(base: &CadLayer, other: &CadLayer) -> Option<CadLayerDiff> {
    let color_index = if base.color_index != other.color_index { Some(other.color_index) } else { None };
    let line_type = if base.line_type != other.line_type { Some(other.line_type.clone()) } else { None };
    let visible = if base.visible != other.visible { Some(other.visible) } else { None };
    if color_index.is_none() && line_type.is_none() && visible.is_none() {
        None
    } else {
        Some(CadLayerDiff { color_index, line_type, visible })
    }
}

async fn between_block(base: &CadBlock, other: &CadBlock) -> Option<CadBlockDiff> {
    let base_point = if base.base_point != other.base_point { Some(other.base_point) } else { None };
    let entities = between_named(&base.entities, &other.entities, |e| e.handle.clone(), between_entity_record);
    if base_point.is_none() && entities.is_none() {
        None
    } else {
        Some(CadBlockDiff { base_point, entities })
    }
}

async fn between_entity_record(base: &CadEntityRecord, other: &CadEntityRecord) -> Option<CadEntityRecordDiff> {
    let layer = if base.layer != other.layer { Some(other.layer.clone()) } else { None };
    let entity = if base.entity != other.entity { Some(other.entity.clone()) } else { None };
    if layer.is_none() && entity.is_none() {
        None
    } else {
        Some(CadEntityRecordDiff { layer, entity })
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No
/// `snapshot: Option<SemioCadSnapshot>` full-replace slot -- this IS `SemioCadDiff::between`.
pub async fn diff_set_snapshot(base: &SemioCadSnapshot, next: &SemioCadSnapshot) -> SemioCadDiff {
    SemioCadDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Hand-rolled `protocol::DiffCodec` — no `dsl::DslDiff` derive attempted: `CadEntity` is a
/// data-carrying enum reached through `entities`/`blocks[].entities` (§3a family), and
/// `CadLayersDiff`/`CadBlocksDiff`/`CadEntitiesDiff` are all instances of the generic
/// `NamedTripleDiff<K,D,T>` (§4.4 family, `dsl` has no `DslField` bridge for generic collection
/// wrappers — f6-final-summary.md §4.4). Grammar: bracket-depth-aware split, hex for strings,
/// `[0]`/`[1,x]` for `Option<T>`, single-letter tag prefix for `CadEntity`'s 9-variant `xs:choice`
/// — same primitive set gif/svg/bcf established.
//#region 🔖️Primitives
pub(crate) async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
async fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

pub(crate) async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
pub(crate) async fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) async fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", p.x, p.y)
}
pub(crate) async fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}

/// 📐️ `L`ine/`A`rc/`C`ircle/`E`llipse/`P`olyline/`T`ext/`I`nsert/`S`olid/`D`imension — the `dxf`
/// r12 `xs:choice`-equivalent made concrete (single-letter tag, same convention bcf's `enc_camera`
/// established).
pub(crate) async fn enc_entity(e: &CadEntity) -> String {
    match e {
        CadEntity::Line { a, b } => format!("L[{},{}]", enc_point2(a), enc_point2(b)),
        CadEntity::Arc { center, radius, start_angle, end_angle } => format!("A[{},{},{},{}]", enc_point2(center), radius, start_angle, end_angle),
        CadEntity::Circle { center, radius } => format!("C[{},{}]", enc_point2(center), radius),
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => {
            format!("E[{},{},{},{},{}]", enc_point2(center), enc_point2(major_axis_end), ratio, start_param, end_param)
        }
        CadEntity::Polyline { vertices, closed } => format!("P[{},{}]", enc_list(vertices, enc_point2), if *closed { "1" } else { "0" }),
        CadEntity::Text { position, height, rotation, content } => format!("T[{},{},{},{}]", enc_point2(position), height, rotation, enc_str(content)),
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => format!("I[{},{},{},{}]", enc_str(block_name), enc_point2(insertion_point), enc_point2(scale), rotation),
        CadEntity::Solid { p1, p2, p3, p4 } => format!("S[{},{},{},{}]", enc_point2(p1), enc_point2(p2), enc_point2(p3), enc_point2(p4)),
        CadEntity::Dimension { def_point, text_position, measurement, text } => format!("D[{},{},{},{}]", enc_point2(def_point), enc_point2(text_position), measurement, enc_str(text)),
    }
}
pub(crate) async fn dec_entity(s: &str) -> Result<CadEntity, String> {
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
            Ok(CadEntity::Polyline { vertices: dec_list(vertices, dec_point2)?, closed: *closed == "1" })
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

pub(crate) async fn enc_layer(l: &CadLayer) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.color_index, enc_str(&l.line_type), if l.visible { "1" } else { "0" })
}
pub(crate) async fn dec_layer(s: &str) -> Result<CadLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color_index, line_type, visible] = parts.as_slice() else { return Err(format!("layer: expected 4 fields, got {}", parts.len())) };
    Ok(CadLayer { name: dec_str(name)?, color_index: parse_i32(color_index)?, line_type: dec_str(line_type)?, visible: *visible == "1" })
}

pub(crate) async fn enc_entity_record(r: &CadEntityRecord) -> String {
    format!("[{},{},{}]", enc_str(&r.handle), enc_str(&r.layer), enc_entity(&r.entity))
}
pub(crate) async fn dec_entity_record(s: &str) -> Result<CadEntityRecord, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [handle, layer, entity] = parts.as_slice() else { return Err(format!("entity record: expected 3 fields, got {}", parts.len())) };
    Ok(CadEntityRecord { handle: dec_str(handle)?, layer: dec_str(layer)?, entity: dec_entity(entity)? })
}

pub(crate) async fn enc_block(b: &CadBlock) -> String {
    format!("[{},{},{}]", enc_str(&b.name), enc_point2(&b.base_point), enc_list(&b.entities, enc_entity_record))
}
pub(crate) async fn dec_block(s: &str) -> Result<CadBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities] = parts.as_slice() else { return Err(format!("block: expected 3 fields, got {}", parts.len())) };
    Ok(CadBlock { name: dec_str(name)?, base_point: dec_point2(base_point)?, entities: dec_list(entities, dec_entity_record)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
pub(crate) async fn enc_layer_diff(d: &CadLayerDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.color_index, |v: &i32| v.to_string()), encode_option(&d.line_type, |v: &String| enc_str(v)), encode_option(&d.visible, |v: &bool| if *v { "1".to_string() } else { "0".to_string() }),)
}
pub(crate) async fn dec_layer_diff(s: &str) -> Result<CadLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color_index, line_type, visible] = parts.as_slice() else { return Err(format!("layer diff: expected 3 fields, got {}", parts.len())) };
    Ok(CadLayerDiff { color_index: decode_option(color_index, parse_i32)?, line_type: decode_option(line_type, dec_str)?, visible: decode_option(visible, |v| Ok(v == "1"))? })
}

pub(crate) async fn enc_entity_record_diff(d: &CadEntityRecordDiff) -> String {
    format!("[{},{}]", encode_option(&d.layer, |v: &String| enc_str(v)), encode_option(&d.entity, enc_entity))
}
pub(crate) async fn dec_entity_record_diff(s: &str) -> Result<CadEntityRecordDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layer, entity] = parts.as_slice() else { return Err(format!("entity record diff: expected 2 fields, got {}", parts.len())) };
    Ok(CadEntityRecordDiff { layer: decode_option(layer, dec_str)?, entity: decode_option(entity, dec_entity)? })
}

pub(crate) async fn enc_block_diff(d: &CadBlockDiff) -> String {
    format!("[{},{}]", encode_option(&d.base_point, |p: &SemioPoint2| enc_point2(p)), encode_option(&d.entities, |v: &CadEntitiesDiff| enc_named_triple(v, |k: &String| enc_str(k), enc_entity_record_diff, enc_entity_record)),)
}
pub(crate) async fn dec_block_diff(s: &str) -> Result<CadBlockDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [base_point, entities] = parts.as_slice() else { return Err(format!("block diff: expected 2 fields, got {}", parts.len())) };
    Ok(CadBlockDiff { base_point: decode_option(base_point, dec_point2)?, entities: decode_option(entities, |v| dec_named_triple(v, dec_str, dec_entity_record_diff, dec_entity_record))? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
async fn print_cad_diff(d: &SemioCadDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(l) = &d.layers {
        tokens.push(format!("layers={}", enc_named_triple(l, |k: &String| enc_str(k), enc_layer_diff, enc_layer)));
    }
    if let Some(b) = &d.blocks {
        tokens.push(format!("blocks={}", enc_named_triple(b, |k: &String| enc_str(k), enc_block_diff, enc_block)));
    }
    if let Some(e) = &d.entities {
        tokens.push(format!("entities={}", enc_named_triple(e, |k: &String| enc_str(k), enc_entity_record_diff, enc_entity_record)));
    }
    tokens.join(" ")
}
async fn parse_cad_diff(line: &str) -> Result<SemioCadDiff, String> {
    let mut d = SemioCadDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("layers=") {
            d.layers = Some(dec_named_triple(rest, dec_str, dec_layer_diff, dec_layer)?);
        } else if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_named_triple(rest, dec_str, dec_block_diff, dec_block)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_named_triple(rest, dec_str, dec_entity_record_diff, dec_entity_record)?);
        } else {
            return Err(format!("cad diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`) backing the real `DiffCodec::encode_diff`/`decode_diff` below — replaces
/// the old `print_diff().into_bytes()` text-as-binary shortcut.
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

impl protocol::DiffCodec for SemioCadDiff {
    async fn print_diff(&self) -> String {
        print_cad_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_cad_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
    /// shortcut. `format u8` + `presence u8` (bit0=`layers`, bit1=`blocks`, bit2=`entities`) are
    /// two REAL fixed fields; each present collection then follows as its own varint-length-
    /// prefixed opaque blob (the same `enc_named_triple` bracket/hex text this type's `print_diff`
    /// already produces) — independently-delimited segments rather than one bare trailing `bytes`
    /// because there can be 0-3 of them (chaining a `Cond` per-segment hits the
    /// `protocol-cond-cannot-chain` gap: a second `if`-guard on a field that was itself only
    /// conditionally decoded hard-errors `eval_cond`).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.layers.is_some() {
            presence |= 0b0000_0001;
        }
        if self.blocks.is_some() {
            presence |= 0b0000_0010;
        }
        if self.entities.is_some() {
            presence |= 0b0000_0100;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = &self.layers {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_layer_diff, enc_layer));
        }
        if let Some(v) = &self.blocks {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_block_diff, enc_block));
        }
        if let Some(v) = &self.entities {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_entity_record_diff, enc_entity_record));
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let mut next_blob = |what: &'static str| -> Result<String, protocol::ProtocolError> { read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what, offset: 2, detail: e }) };
        let layers = if presence & 0b0000_0001 != 0 {
            Some(dec_named_triple(&next_blob("diff layers blob")?, dec_str, dec_layer_diff, dec_layer).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layers text", offset: 2, detail: e })?)
        } else {
            None
        };
        let blocks = if presence & 0b0000_0010 != 0 {
            Some(dec_named_triple(&next_blob("diff blocks blob")?, dec_str, dec_block_diff, dec_block).map_err(|e| protocol::ProtocolError::Malformed { what: "diff blocks text", offset: 2, detail: e })?)
        } else {
            None
        };
        let entities = if presence & 0b0000_0100 != 0 {
            Some(dec_named_triple(&next_blob("diff entities blob")?, dec_str, dec_entity_record_diff, dec_entity_record).map_err(|e| protocol::ProtocolError::Malformed { what: "diff entities text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioCadDiff { layers, blocks, entities })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioCadDiff` cases (empty/no-op, a full removed/modified/added sweep both
/// directions across every collection incl. the nested `blocks[].entities`, exercising 7 of the 9
/// `CadEntity` variants) — single source of truth for `diff_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️component.rs`. Self-contained (does not reach into
/// `#[cfg(test)] mod tests`'s own private `sweep_a`/`sweep_b`, since a private item of a child
/// module is not visible to its parent).
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<SemioCadDiff> {
    let a = SemioCadSnapshot {
        schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "keep".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "layer-removed".into(), color_index: 2, line_type: "DASHED".into(), visible: false }],
        blocks: vec![CadBlock {
            name: "keep-block".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "be1".into(), layer: "keep".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 1.0 } } }],
        }],
        entities: vec![
            CadEntityRecord { handle: "e1".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0 } },
            CadEntityRecord { handle: "e-removed".into(), layer: "keep".into(), entity: CadEntity::Polyline { vertices: vec![SemioPoint2 { x: 0.0, y: 0.0 }], closed: false } },
        ],
    };
    let b = SemioCadSnapshot {
        schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "keep".into(), color_index: 9, line_type: "DASHDOT".into(), visible: false }, CadLayer { name: "layer-added".into(), color_index: 4, line_type: "HIDDEN".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "keep-block".into(),
            base_point: SemioPoint2 { x: 5.0, y: 5.0 },
            entities: vec![
                CadEntityRecord { handle: "be1".into(), layer: "layer-added".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 } },
                CadEntityRecord { handle: "be-added".into(), layer: "keep".into(), entity: CadEntity::Dimension { def_point: SemioPoint2 { x: 0.0, y: 0.0 }, text_position: SemioPoint2 { x: 1.0, y: 1.0 }, measurement: 3.3, text: "3.3m".into() } },
            ],
        }],
        entities: vec![
            CadEntityRecord { handle: "e1".into(), layer: "layer-added".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 } },
            CadEntityRecord { handle: "e-added".into(), layer: "keep".into(), entity: CadEntity::Insert { block_name: "keep-block".into(), insertion_point: SemioPoint2 { x: 0.0, y: 0.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } },
        ],
    };

    vec![SemioCadDiff::default(), <SemioCadDiff as DiffAlgebra<SemioCadSnapshot>>::between(&a, &b), <SemioCadDiff as DiffAlgebra<SemioCadSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;

    //#region Fixtures
    /// 🧪️ Every field/collection mutable, incl. a nested `blocks[].entities` add/remove/modify and
    /// an `entities` entry whose `entity` variant itself changes kind (Circle -> Ellipse).
    async fn sweep_a() -> SemioCadSnapshot {
        SemioCadSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: vec![CadLayer { name: "keep".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "layer-remove".into(), color_index: 2, line_type: "DASHED".into(), visible: false }],
            blocks: vec![
                CadBlock {
                    name: "keep-block".into(),
                    base_point: SemioPoint2 { x: 0.0, y: 0.0 },
                    entities: vec![
                        CadEntityRecord { handle: "be-keep".into(), layer: "keep".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 1.0 } } },
                        CadEntityRecord { handle: "be-remove".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 3.0 } },
                    ],
                },
                CadBlock { name: "block-remove".into(), base_point: SemioPoint2 { x: 5.0, y: 5.0 }, entities: Vec::new() },
            ],
            entities: vec![
                CadEntityRecord { handle: "e-keep".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0 } },
                CadEntityRecord { handle: "e-remove".into(), layer: "layer-remove".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 9.0, y: 9.0 } } },
            ],
        }
    }

    async fn sweep_b() -> SemioCadSnapshot {
        SemioCadSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: vec![CadLayer { name: "keep".into(), color_index: 9, line_type: "DASHDOT".into(), visible: false }, CadLayer { name: "layer-add".into(), color_index: 4, line_type: "HIDDEN".into(), visible: true }],
            blocks: vec![
                CadBlock {
                    name: "keep-block".into(),
                    base_point: SemioPoint2 { x: 10.0, y: 10.0 },
                    entities: vec![
                        CadEntityRecord { handle: "be-keep".into(), layer: "layer-add".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 } },
                        CadEntityRecord { handle: "be-add".into(), layer: "keep".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 1.0, y: 1.0 }, height: 2.0, rotation: 0.0, content: "hi".into() } },
                    ],
                },
                CadBlock { name: "block-add".into(), base_point: SemioPoint2 { x: 7.0, y: 7.0 }, entities: Vec::new() },
            ],
            entities: vec![
                CadEntityRecord {
                    handle: "e-keep".into(),
                    layer: "layer-add".into(),
                    entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 },
                },
                CadEntityRecord { handle: "e-add".into(), layer: "keep".into(), entity: CadEntity::Insert { block_name: "keep-block".into(), insertion_point: SemioPoint2 { x: 0.0, y: 0.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } },
            ],
        }
    }
    //#endregion Fixtures

    //#region 🧪️Law6_FieldSweep
    /// ⚖️ Law 6 — `field_sweep`: `sweep_a`/`sweep_b` differ in every mutable field, incl. per
    /// collection one removed/one modified-in-every-field/one added, at BOTH the top level and the
    /// nested `blocks[].entities` level.
    #[test]
    async fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = SemioCadDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("apply must succeed for a well-formed fixture"), b, "between(a,b).apply(a) must equal b");
        let backward = SemioCadDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("apply must succeed for a well-formed fixture"), a, "between(b,a).apply(b) must equal a");
        assert!(SemioCadDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        let layers_diff = forward.layers.as_ref().expect("layers diff present");
        assert!(!layers_diff.removed.is_empty(), "layers.removed not swept");
        assert!(!layers_diff.added.is_empty(), "layers.added not swept");
        let keep_layer_diff = &layers_diff.modified.iter().find(|m| m.key == "keep").expect("keep layer modified").diff;
        assert!(keep_layer_diff.color_index.is_some(), "layer.color_index not swept");
        assert!(keep_layer_diff.line_type.is_some(), "layer.line_type not swept");
        assert!(keep_layer_diff.visible.is_some(), "layer.visible not swept");

        let blocks_diff = forward.blocks.as_ref().expect("blocks diff present");
        assert!(!blocks_diff.removed.is_empty(), "blocks.removed not swept");
        assert!(!blocks_diff.added.is_empty(), "blocks.added not swept");
        let keep_block_diff = &blocks_diff.modified.iter().find(|m| m.key == "keep-block").expect("keep-block modified").diff;
        assert!(keep_block_diff.base_point.is_some(), "block.base_point not swept");
        let nested_entities_diff = keep_block_diff.entities.as_ref().expect("nested block entities diff present");
        assert!(!nested_entities_diff.removed.is_empty(), "block.entities.removed not swept");
        assert!(!nested_entities_diff.added.is_empty(), "block.entities.added not swept");
        let be_keep_diff = &nested_entities_diff.modified.iter().find(|m| m.key == "be-keep").expect("be-keep modified").diff;
        assert!(be_keep_diff.layer.is_some(), "block entity.layer not swept");
        assert!(be_keep_diff.entity.is_some(), "block entity.entity not swept");

        let entities_diff = forward.entities.as_ref().expect("entities diff present");
        assert!(!entities_diff.removed.is_empty(), "entities.removed not swept");
        assert!(!entities_diff.added.is_empty(), "entities.added not swept");
        let e_keep_diff = &entities_diff.modified.iter().find(|m| m.key == "e-keep").expect("e-keep modified").diff;
        assert!(e_keep_diff.layer.is_some(), "entity.layer not swept");
        assert!(e_keep_diff.entity.is_some(), "entity.entity not swept");
    }
    //#endregion

    //#region 🧪️Law3_AbsorbLaw
    /// ⚖️ Law 3 — `absorb_law`: curated op list (Insert+Remove-before, Add+SetField-patches-into-
    /// added, Modify+Remove-annihilates) plus associativity — same canonical cases as bcf's.
    #[test]
    async fn absorb_law() {
        let base = sweep_a();

        // Insert+Remove-before: add a layer, then remove an unrelated layer — independent, net
        // effect must match sequential application.
        let d1 = SemioCadDiff { layers: Some(CadLayersDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CadLayer { name: "fresh".into(), color_index: 3, line_type: "CONTINUOUS".into(), visible: true }] }), blocks: None, entities: None };
        let d2 = SemioCadDiff { layers: Some(CadLayersDiff { removed: vec!["layer-remove".into()], modified: Vec::new(), added: Vec::new() }), blocks: None, entities: None };
        assert_absorb_matches_sequential(&base, d1, d2);

        // Add+SetField: insert an entity, then immediately edit that SAME entity's layer -- must
        // patch into the carried `added` payload, not become a dangling `modified` entry.
        let new_entity = CadEntityRecord { handle: "e-fresh".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0 } };
        let d1 = SemioCadDiff { layers: None, blocks: None, entities: Some(CadEntitiesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![new_entity] }) };
        let d2 = wrap_entity_diff("e-fresh", CadEntityRecordDiff { layer: Some("layer-remove".into()), entity: None });
        let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
        let entities_diff = absorbed.entities.as_ref().expect("entities diff");
        assert!(entities_diff.modified.is_empty(), "edit-after-insert must patch into added, not appear as modified");
        let added_entity = entities_diff.added.iter().find(|e| e.handle == "e-fresh").expect("e-fresh still in added");
        assert_eq!(added_entity.layer, "layer-remove");

        // Modify+Remove: edit a block's base_point, then remove that same block -- must annihilate
        // to a plain removal, not a dangling modify+remove pair.
        let d1 = wrap_block_diff("keep-block", CadBlockDiff { base_point: Some(SemioPoint2 { x: 99.0, y: 99.0 }), entities: None });
        let d2 = SemioCadDiff { layers: None, blocks: Some(CadBlocksDiff { removed: vec!["keep-block".into()], modified: Vec::new(), added: Vec::new() }), entities: None };
        let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
        let blocks_diff = absorbed.blocks.as_ref().expect("blocks diff");
        assert_eq!(blocks_diff.removed, vec!["keep-block".to_string()]);
        assert!(blocks_diff.modified.is_empty());

        // Associativity: absorb(absorb(d1,d2),d3) == absorb(d1,absorb(d2,d3)).
        let d1 = wrap_layer_diff("keep", CadLayerDiff { color_index: Some(42), line_type: None, visible: None });
        let mid1 = d1.apply(&base).expect("apply must succeed for a well-formed fixture");
        let d2 = SemioCadDiff { layers: Some(CadLayersDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CadLayer { name: "assoc".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }] }), blocks: None, entities: None };
        let _mid2 = d2.apply(&mid1).expect("apply must succeed for a well-formed fixture");
        let d3 = wrap_layer_diff("assoc", CadLayerDiff { color_index: None, line_type: Some("DASHED".into()), visible: None });

        let mut left = d1.clone();
        MutationDiff::absorb(&mut left, d2.clone());
        MutationDiff::absorb(&mut left, d3.clone());

        let mut d2_d3 = d2;
        MutationDiff::absorb(&mut d2_d3, d3);
        let mut right = d1;
        MutationDiff::absorb(&mut right, d2_d3);

        assert_eq!(left.apply(&base).expect("apply must succeed for a well-formed fixture"), right.apply(&base).expect("apply must succeed for a well-formed fixture"), "absorb must be associative");
    }

    async fn assert_absorb_matches_sequential(base: &SemioCadSnapshot, d1: SemioCadDiff, d2: SemioCadDiff) -> SemioCadDiff {
        let sequential = d2.apply(&d1.apply(base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        let mut absorbed = d1;
        MutationDiff::absorb(&mut absorbed, d2);
        assert_eq!(absorbed.apply(base).expect("apply must succeed for a well-formed fixture"), sequential, "absorb(d1,d2).apply(base) must equal sequential application");
        absorbed
    }
    //#endregion

    //#region 🧪️Law4_BetweenRoundtripLaw
    /// ⚖️ Law 4 — `between_roundtrip_law`: `between(a,b).apply(a) == b` on fixtures.
    #[test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let d = SemioCadDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        let d_back = SemioCadDiff::between(&b, &a);
        assert_eq!(d_back.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(SemioCadDiff::between(&a, &a).is_empty());
    }
    //#endregion

    //#region 🧪️Law8_DiffCodecTextBinaryRoundtripLaw
    /// ⚖️ Law 8 — `diff_codec_text_binary_roundtrip_law`: hand-rolled `DiffCodec` text/binary
    /// round-trip, exercising every collection triple (top-level AND the nested
    /// `blocks[].entities`) plus all 9 `CadEntity` variants across `between()` results.
    #[test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let mut cases = vec![SemioCadDiff::default(), SemioCadDiff::between(&a, &b), SemioCadDiff::between(&b, &a), SemioCadDiff::between(&a, &a)];
        // Exercise every remaining CadEntity variant not already covered by sweep_a/sweep_b.
        cases.push(wrap_entity_diff("h", CadEntityRecordDiff { layer: None, entity: Some(CadEntity::Polyline { vertices: vec![SemioPoint2 { x: 0.0, y: 0.0 }, SemioPoint2 { x: 1.0, y: 1.0 }], closed: true }) }));
        cases.push(wrap_entity_diff(
            "h",
            CadEntityRecordDiff { layer: None, entity: Some(CadEntity::Solid { p1: SemioPoint2 { x: 0.0, y: 0.0 }, p2: SemioPoint2 { x: 1.0, y: 0.0 }, p3: SemioPoint2 { x: 1.0, y: 1.0 }, p4: SemioPoint2 { x: 0.0, y: 1.0 } }) },
        ));
        cases.push(wrap_entity_diff("h", CadEntityRecordDiff { layer: None, entity: Some(CadEntity::Dimension { def_point: SemioPoint2 { x: 0.0, y: 0.0 }, text_position: SemioPoint2 { x: 1.0, y: 1.0 }, measurement: 4.2, text: "4.2m".into() }) }));

        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioCadDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioCadDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion
}
//#endregion 🔖️Tests
