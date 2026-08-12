//! 🧬️ SemioCadMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's
//! `inverse()` looks up prior state from `base` and constructs the exact undoing mutation
//! (name/handle-aware, matching bcf/docx precedent).

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::diff::{
    dec_block, dec_entity, dec_entity_record, dec_layer, dec_list, dec_point2, dec_str, diff_set_snapshot,
    enc_block, enc_entity, enc_entity_record, enc_layer, enc_list, enc_point2, enc_str, decode_option, encode_option,
    wrap_block_diff, wrap_block_entity_diff, wrap_entity_diff, wrap_layer_diff,
    CadBlockDiff, CadEntityRecordDiff, CadLayerDiff, SemioCadDiff,
};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::{Mutation, OpText};
#[cfg(test)]
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioCadMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: SemioCadSnapshot },
    AddLayer { layer: CadLayer },
    RemoveLayer { name: String },
    SetLayer {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color_index: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        line_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        visible: Option<bool>,
    },
    AddBlock { block: CadBlock },
    RemoveBlock { name: String },
    SetBlockBasePoint { name: String, base_point: SemioPoint2 },
    AddEntity { entity: CadEntityRecord },
    RemoveEntity { handle: String },
    SetEntityLayer { handle: String, layer: String },
    SetEntityGeometry { handle: String, entity: CadEntity },
    AddBlockEntity { block_name: String, entity: CadEntityRecord },
    RemoveBlockEntity { block_name: String, handle: String },
    SetBlockEntityLayer { block_name: String, handle: String, layer: String },
    SetBlockEntityGeometry { block_name: String, handle: String, entity: CadEntity },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Single semantics source: the returned diff IS what gets
/// applied.
pub fn apply_semio_cad_mutation(snapshot: &mut SemioCadSnapshot, mutation: &SemioCadMutation) -> SemioCadDiff {
    let diff = <SemioCadMutation as Mutation<SemioCadSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioCadDiff as protocol::MutationDiff<SemioCadSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SemioCadSnapshot> for SemioCadMutation {
    type Diff = SemioCadDiff;

    fn diff(&self, base: &SemioCadSnapshot) -> Self::Diff {
        match self {
            SemioCadMutation::NoMutation => SemioCadDiff::default(),
            SemioCadMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioCadMutation::AddLayer { layer } => SemioCadDiff {
                layers: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![layer.clone()] }),
                blocks: None,
                entities: None,
            },
            SemioCadMutation::RemoveLayer { name } => SemioCadDiff {
                layers: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }),
                blocks: None,
                entities: None,
            },
            SemioCadMutation::SetLayer { name, color_index, line_type, visible } => {
                wrap_layer_diff(name, CadLayerDiff { color_index: *color_index, line_type: line_type.clone(), visible: *visible })
            }
            SemioCadMutation::AddBlock { block } => SemioCadDiff {
                layers: None,
                blocks: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![block.clone()] }),
                entities: None,
            },
            SemioCadMutation::RemoveBlock { name } => SemioCadDiff {
                layers: None,
                blocks: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }),
                entities: None,
            },
            SemioCadMutation::SetBlockBasePoint { name, base_point } => wrap_block_diff(name, CadBlockDiff { base_point: Some(*base_point), entities: None }),
            SemioCadMutation::AddEntity { entity } => SemioCadDiff {
                layers: None,
                blocks: None,
                entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }),
            },
            SemioCadMutation::RemoveEntity { handle } => SemioCadDiff {
                layers: None,
                blocks: None,
                entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }),
            },
            SemioCadMutation::SetEntityLayer { handle, layer } => wrap_entity_diff(handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }),
            SemioCadMutation::SetEntityGeometry { handle, entity } => wrap_entity_diff(handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }),
            SemioCadMutation::AddBlockEntity { block_name, entity } => wrap_block_diff(block_name, CadBlockDiff {
                base_point: None,
                entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }),
            }),
            SemioCadMutation::RemoveBlockEntity { block_name, handle } => wrap_block_diff(block_name, CadBlockDiff {
                base_point: None,
                entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }),
            }),
            SemioCadMutation::SetBlockEntityLayer { block_name, handle, layer } => wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }),
            SemioCadMutation::SetBlockEntityGeometry { block_name, handle, entity } => wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }),
        }
    }

    fn inverse(&self, base: &SemioCadSnapshot) -> Vec<Self> {
        match self {
            SemioCadMutation::NoMutation => vec![SemioCadMutation::NoMutation],
            SemioCadMutation::SetSnapshot { .. } => vec![SemioCadMutation::SetSnapshot { snapshot: base.clone() }],
            SemioCadMutation::AddLayer { layer } => vec![SemioCadMutation::RemoveLayer { name: layer.name.clone() }],
            SemioCadMutation::RemoveLayer { name } => match find_layer(base, name) {
                Some(l) => vec![SemioCadMutation::AddLayer { layer: l.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::SetLayer { name, color_index, line_type, visible } => match find_layer(base, name) {
                Some(l) => vec![SemioCadMutation::SetLayer {
                    name: name.clone(),
                    color_index: color_index.as_ref().map(|_| l.color_index),
                    line_type: line_type.as_ref().map(|_| l.line_type.clone()),
                    visible: visible.as_ref().map(|_| l.visible),
                }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::AddBlock { block } => vec![SemioCadMutation::RemoveBlock { name: block.name.clone() }],
            SemioCadMutation::RemoveBlock { name } => match find_block(base, name) {
                Some(b) => vec![SemioCadMutation::AddBlock { block: b.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::SetBlockBasePoint { name, .. } => match find_block(base, name) {
                Some(b) => vec![SemioCadMutation::SetBlockBasePoint { name: name.clone(), base_point: b.base_point }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::AddEntity { entity } => vec![SemioCadMutation::RemoveEntity { handle: entity.handle.clone() }],
            SemioCadMutation::RemoveEntity { handle } => match find_entity(base, handle) {
                Some(e) => vec![SemioCadMutation::AddEntity { entity: e.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::SetEntityLayer { handle, .. } => match find_entity(base, handle) {
                Some(e) => vec![SemioCadMutation::SetEntityLayer { handle: handle.clone(), layer: e.layer.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::SetEntityGeometry { handle, .. } => match find_entity(base, handle) {
                Some(e) => vec![SemioCadMutation::SetEntityGeometry { handle: handle.clone(), entity: e.entity.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::AddBlockEntity { block_name, entity } => vec![SemioCadMutation::RemoveBlockEntity { block_name: block_name.clone(), handle: entity.handle.clone() }],
            SemioCadMutation::RemoveBlockEntity { block_name, handle } => match find_block_entity(base, block_name, handle) {
                Some(e) => vec![SemioCadMutation::AddBlockEntity { block_name: block_name.clone(), entity: e.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::SetBlockEntityLayer { block_name, handle, .. } => match find_block_entity(base, block_name, handle) {
                Some(e) => vec![SemioCadMutation::SetBlockEntityLayer { block_name: block_name.clone(), handle: handle.clone(), layer: e.layer.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
            SemioCadMutation::SetBlockEntityGeometry { block_name, handle, .. } => match find_block_entity(base, block_name, handle) {
                Some(e) => vec![SemioCadMutation::SetBlockEntityGeometry { block_name: block_name.clone(), handle: handle.clone(), entity: e.entity.clone() }],
                None => vec![SemioCadMutation::NoMutation],
            },
        }
    }
}

fn find_layer<'a>(base: &'a SemioCadSnapshot, name: &str) -> Option<&'a CadLayer> {
    base.layers.iter().find(|l| l.name == name)
}
fn find_block<'a>(base: &'a SemioCadSnapshot, name: &str) -> Option<&'a CadBlock> {
    base.blocks.iter().find(|b| b.name == name)
}
fn find_entity<'a>(base: &'a SemioCadSnapshot, handle: &str) -> Option<&'a CadEntityRecord> {
    base.entities.iter().find(|e| e.handle == handle)
}
fn find_block_entity<'a>(base: &'a SemioCadSnapshot, block_name: &str, handle: &str) -> Option<&'a CadEntityRecord> {
    find_block(base, block_name)?.entities.iter().find(|e| e.handle == handle)
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary` — reuses the diff module's `pub(crate)` grammar primitives
/// (`enc_str`/`enc_layer`/`enc_block`/`enc_entity`/`encode_option`/...) rather than duplicating
/// them, same pattern `BcfMutation` established. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant.
fn enc_cad_snapshot(s: &SemioCadSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_list(&s.layers, enc_layer), enc_list(&s.blocks, enc_block), enc_list(&s.entities, enc_entity_record))
}
fn dec_cad_snapshot(s: &str) -> Result<SemioCadSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, layers, blocks, entities] = parts.as_slice() else { return Err(format!("cad snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(SemioCadSnapshot { schema: dec_str(schema)?, layers: dec_list(layers, dec_layer)?, blocks: dec_list(blocks, dec_block)?, entities: dec_list(entities, dec_entity_record)? })
}

fn print_cad_mutation(m: &SemioCadMutation) -> String {
    match m {
        SemioCadMutation::NoMutation => "no-mutation".to_string(),
        SemioCadMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_cad_snapshot(snapshot)),
        SemioCadMutation::AddLayer { layer } => format!("add-layer layer={}", enc_layer(layer)),
        SemioCadMutation::RemoveLayer { name } => format!("remove-layer name={}", enc_str(name)),
        SemioCadMutation::SetLayer { name, color_index, line_type, visible } => format!(
            "set-layer name={} color-index={} line-type={} visible={}",
            enc_str(name),
            encode_option(color_index, |v: &i32| v.to_string()),
            encode_option(line_type, |v: &String| enc_str(v)),
            encode_option(visible, |v: &bool| if *v { "1".to_string() } else { "0".to_string() }),
        ),
        SemioCadMutation::AddBlock { block } => format!("add-block block={}", enc_block(block)),
        SemioCadMutation::RemoveBlock { name } => format!("remove-block name={}", enc_str(name)),
        SemioCadMutation::SetBlockBasePoint { name, base_point } => format!("set-block-base-point name={} base-point={}", enc_str(name), enc_point2(base_point)),
        SemioCadMutation::AddEntity { entity } => format!("add-entity entity={}", enc_entity_record(entity)),
        SemioCadMutation::RemoveEntity { handle } => format!("remove-entity handle={}", enc_str(handle)),
        SemioCadMutation::SetEntityLayer { handle, layer } => format!("set-entity-layer handle={} layer={}", enc_str(handle), enc_str(layer)),
        SemioCadMutation::SetEntityGeometry { handle, entity } => format!("set-entity-geometry handle={} entity={}", enc_str(handle), enc_entity(entity)),
        SemioCadMutation::AddBlockEntity { block_name, entity } => format!("add-block-entity block-name={} entity={}", enc_str(block_name), enc_entity_record(entity)),
        SemioCadMutation::RemoveBlockEntity { block_name, handle } => format!("remove-block-entity block-name={} handle={}", enc_str(block_name), enc_str(handle)),
        SemioCadMutation::SetBlockEntityLayer { block_name, handle, layer } => format!("set-block-entity-layer block-name={} handle={} layer={}", enc_str(block_name), enc_str(handle), enc_str(layer)),
        SemioCadMutation::SetBlockEntityGeometry { block_name, handle, entity } => format!("set-block-entity-geometry block-name={} handle={} entity={}", enc_str(block_name), enc_str(handle), enc_entity(entity)),
    }
}

fn parse_cad_mutation(line: &str) -> Result<SemioCadMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioCadMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("cad mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("cad mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioCadMutation::SetSnapshot { snapshot: dec_cad_snapshot(arg("snapshot")?)? }),
        "add-layer" => Ok(SemioCadMutation::AddLayer { layer: dec_layer(arg("layer")?)? }),
        "remove-layer" => Ok(SemioCadMutation::RemoveLayer { name: dec_str(arg("name")?)? }),
        "set-layer" => Ok(SemioCadMutation::SetLayer {
            name: dec_str(arg("name")?)?,
            color_index: decode_option(arg("color-index")?, |v| v.parse::<i32>().map_err(|e: std::num::ParseIntError| e.to_string()))?,
            line_type: decode_option(arg("line-type")?, dec_str)?,
            visible: decode_option(arg("visible")?, |v| Ok(v == "1"))?,
        }),
        "add-block" => Ok(SemioCadMutation::AddBlock { block: dec_block(arg("block")?)? }),
        "remove-block" => Ok(SemioCadMutation::RemoveBlock { name: dec_str(arg("name")?)? }),
        "set-block-base-point" => Ok(SemioCadMutation::SetBlockBasePoint { name: dec_str(arg("name")?)?, base_point: dec_point2(arg("base-point")?)? }),
        "add-entity" => Ok(SemioCadMutation::AddEntity { entity: dec_entity_record(arg("entity")?)? }),
        "remove-entity" => Ok(SemioCadMutation::RemoveEntity { handle: dec_str(arg("handle")?)? }),
        "set-entity-layer" => Ok(SemioCadMutation::SetEntityLayer { handle: dec_str(arg("handle")?)?, layer: dec_str(arg("layer")?)? }),
        "set-entity-geometry" => Ok(SemioCadMutation::SetEntityGeometry { handle: dec_str(arg("handle")?)?, entity: dec_entity(arg("entity")?)? }),
        "add-block-entity" => Ok(SemioCadMutation::AddBlockEntity { block_name: dec_str(arg("block-name")?)?, entity: dec_entity_record(arg("entity")?)? }),
        "remove-block-entity" => Ok(SemioCadMutation::RemoveBlockEntity { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)? }),
        "set-block-entity-layer" => Ok(SemioCadMutation::SetBlockEntityLayer { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)?, layer: dec_str(arg("layer")?)? }),
        "set-block-entity-geometry" => Ok(SemioCadMutation::SetBlockEntityGeometry { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)?, entity: dec_entity(arg("entity")?)? }),
        other => Err(format!("cad mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioCadMutation {
    fn print_op(&self) -> String { print_cad_mutation(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_cad_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioCadMutation`'s own enum variants and
/// `parse_cad_mutation`'s keyword match — the real binary `tag` field's source of truth.
const OP_KEYWORDS: [&str; 16] = [
    "no-mutation",
    "set-snapshot",
    "add-layer",
    "remove-layer",
    "set-layer",
    "add-block",
    "remove-block",
    "set-block-base-point",
    "add-entity",
    "remove-entity",
    "set-entity-layer",
    "set-entity-geometry",
    "add-block-entity",
    "remove-block-entity",
    "set-block-entity-layer",
    "set-block-entity-geometry",
];
fn variant_ordinal(m: &SemioCadMutation) -> u8 {
    match m {
        SemioCadMutation::NoMutation => 0,
        SemioCadMutation::SetSnapshot { .. } => 1,
        SemioCadMutation::AddLayer { .. } => 2,
        SemioCadMutation::RemoveLayer { .. } => 3,
        SemioCadMutation::SetLayer { .. } => 4,
        SemioCadMutation::AddBlock { .. } => 5,
        SemioCadMutation::RemoveBlock { .. } => 6,
        SemioCadMutation::SetBlockBasePoint { .. } => 7,
        SemioCadMutation::AddEntity { .. } => 8,
        SemioCadMutation::RemoveEntity { .. } => 9,
        SemioCadMutation::SetEntityLayer { .. } => 10,
        SemioCadMutation::SetEntityGeometry { .. } => 11,
        SemioCadMutation::AddBlockEntity { .. } => 12,
        SemioCadMutation::RemoveBlockEntity { .. } => 13,
        SemioCadMutation::SetBlockEntityLayer { .. } => 14,
        SemioCadMutation::SetBlockEntityGeometry { .. } => 15,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_cad_mutation` (empty for `no-mutation`) —
/// the binary frame's `tag` byte already carries the keyword, so the text keyword itself is
/// redundant in the binary payload.
fn print_cad_mutation_args(m: &SemioCadMutation) -> String {
    match print_cad_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`OP_KEYWORDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_cad_mutation`/`parse_cad_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl protocol::OpBinary for SemioCadMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_cad_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture + representative `SemioCadMutation` cases (one per variant, incl.
/// `NoMutation`, plus extra `SetEntityGeometry`/`SetBlockEntityGeometry` cases exercising several
/// of the 9 `CadEntity` kinds) — single source of truth for this facet's own tests AND
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
fn fixture() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![
            CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true },
            CadLayer { name: "dim".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true },
        ],
        blocks: vec![CadBlock {
            name: "door".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "be1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } }],
        }],
        entities: vec![CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } }],
    }
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioCadMutation> {
    let base = fixture();
    vec![
        SemioCadMutation::NoMutation,
        SemioCadMutation::SetSnapshot { snapshot: base.clone() },
        SemioCadMutation::AddLayer { layer: CadLayer { name: "fresh".into(), color_index: 3, line_type: "CONTINUOUS".into(), visible: true } },
        SemioCadMutation::RemoveLayer { name: "dim".into() },
        SemioCadMutation::SetLayer { name: "0".into(), color_index: Some(3), line_type: None, visible: Some(false) },
        SemioCadMutation::AddBlock { block: CadBlock { name: "window".into(), base_point: SemioPoint2 { x: 2.0, y: 2.0 }, entities: Vec::new() } },
        SemioCadMutation::RemoveBlock { name: "door".into() },
        SemioCadMutation::SetBlockBasePoint { name: "door".into(), base_point: SemioPoint2 { x: 5.0, y: 5.0 } },
        SemioCadMutation::AddEntity { entity: CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } } },
        SemioCadMutation::RemoveEntity { handle: "h1".into() },
        SemioCadMutation::SetEntityLayer { handle: "h1".into(), layer: "dim".into() },
        SemioCadMutation::SetEntityGeometry { handle: "h1".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 } },
        SemioCadMutation::AddBlockEntity { block_name: "door".into(), entity: CadEntityRecord { handle: "be2".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 0.0, y: 0.0 }, height: 2.5, rotation: 0.0, content: "label".into() } } },
        SemioCadMutation::RemoveBlockEntity { block_name: "door".into(), handle: "be1".into() },
        SemioCadMutation::SetBlockEntityLayer { block_name: "door".into(), handle: "be1".into(), layer: "dim".into() },
        SemioCadMutation::SetBlockEntityGeometry { block_name: "door".into(), handle: "be1".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 } },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️Law1_MutationDiffLaw
    /// ⚖️ Law 1 — `mutation_diff_law`: for every variant, `apply_semio_cad_mutation`'s returned
    /// diff equals `m.diff(base)`, and applying it matches `diff.apply(base)`.
    #[test]
    fn mutation_diff_law() {
        let base = fixture();
        for m in demo_mutation_cases() {
            let mut snap = base.clone();
            let returned = apply_semio_cad_mutation(&mut snap, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
            assert_eq!(snap, protocol::MutationDiff::apply(&expected_diff, &base), "apply mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law2_InverseLaw
    /// ⚖️ Law 2 — `inverse_law`: every mutation round-trips (mutation-level) and every diff
    /// round-trips (diff-level `d.inverse(base).apply(&d.apply(base)) == base`).
    #[test]
    fn inverse_law() {
        use protocol::command::DiffAlgebra;
        let base = fixture();
        for m in demo_mutation_cases() {
            let mut snap = base.clone();
            apply_semio_cad_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                let mut undone = snap.clone();
                apply_semio_cad_mutation(&mut undone, &inv);
                assert_eq!(undone, base, "mutation-level inverse mismatch for {m:?}");
            }

            let d = m.diff(&base);
            let after = protocol::MutationDiff::apply(&d, &base);
            let d_inv = d.inverse(&base);
            assert_eq!(protocol::MutationDiff::apply(&d_inv, &after), base, "diff-level inverse mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law7_OpTextBinaryRoundtripLaw
    /// ⚖️ Law 7 — `op_text_binary_roundtrip_law`: `OpText`/`OpBinary` round-trip for the
    /// hand-rolled `SemioCadMutation` grammar, covering every variant (incl. `NoMutation`) via
    /// [`demo_mutation_cases`].
    #[test]
    fn op_text_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioCadMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioCadMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion
}
//#endregion 🔖️Tests
