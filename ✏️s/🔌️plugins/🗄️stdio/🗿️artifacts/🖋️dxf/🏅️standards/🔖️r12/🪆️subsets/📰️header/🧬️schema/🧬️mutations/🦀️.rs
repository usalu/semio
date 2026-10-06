//! 🧬️ DxfMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `DxfDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, name/index-aware, reading the pre-state it needs from `base`.
//! `SetLayer`/`SetStyle`/`SetLinetype`/`SetEntity`/`SetBlock` each set a WHOLE item (not a
//! single sub-field) — their `diff()` still constructs a sparse per-field patch by comparing
//! against `base`'s current value, never a full-item replace.
//!
//! 🧪️ F6: `OpText`/`OpBinary` for `DxfMutation` are **hand-rolled** — `#[derive(dsl::DslOps)]`
//! confirmed rejected by a real `cargo check` (independent of `DxfDiff`'s own rejection above):
//! `SetSnapshot{snapshot:DxfSnapshot}` recursively contains `DxfEntity` (no `DslField`), and
//! `InsertEntity`/`SetEntity`'s `entity: DxfEntity` / `InsertBlock`/`SetBlock`'s `block: DxfBlock`
//! (which itself contains `Vec<DxfEntity>`) carry the same data-carrying-enum payload DIRECTLY as
//! a variant field — `error[E0277]: the trait bound 'DxfEntity: DslField' is not satisfied` at
//! `InsertEntity{entity:DxfEntity}`/`SetEntity{entity:DxfEntity}` (recon report §3a, mutation-side
//! twin of the diff-side blocker: the derive requires `DslField` on every reachable type whether
//! it arrives via a Diff struct field or a Mutation variant field). Grammar: `keyword arg=value
//! ...` (space-separated, same shape the derive's own handcrafted-wrapper convention uses),
//! reusing `🔺️diff`'s `pub(crate)` grammar primitives rather than duplicating them a second time.

use crate::schema::diff::{block_diff_between, // 🧪️ P2-FG1: real recursive binary twins backing the upgraded `OpBinary` impl below (see
    // `🔺️diff/🦀️.rs`'s `#region 🔖️ItemBinaryCodecs`/`#region 🔖️BinaryPrimitives`).
    dec_block_bin, diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_block, diff_remove_entity, diff_remove_header_var, diff_remove_layer, diff_remove_linetype, diff_remove_style, diff_set_block, diff_set_entity, diff_set_header_var, diff_set_layer, diff_set_linetype, diff_set_snapshot, diff_set_style, entity_diff_between_pub, layer_diff_between, linetype_diff_between, style_diff_between, DxfDiff};































use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle};
use crate::DxfSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "📦insert-block/🦀️.rs"]
pub mod insert_block;
#[path = "🧩insert-entity/🦀️.rs"]
pub mod insert_entity;
#[path = "🧱insert-layer/🦀️.rs"]
pub mod insert_layer;
#[path = "🧵insert-linetype/🦀️.rs"]
pub mod insert_linetype;
#[path = "🎨insert-style/🦀️.rs"]
pub mod insert_style;
#[path = "🪓remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "🗑️remove-entity/🦀️.rs"]
pub mod remove_entity;
#[path = "🧹remove-header-var/🦀️.rs"]
pub mod remove_header_var;
#[path = "🪨remove-layer/🦀️.rs"]
pub mod remove_layer;
#[path = "🪚remove-linetype/🦀️.rs"]
pub mod remove_linetype;
#[path = "🧽remove-style/🦀️.rs"]
pub mod remove_style;
#[path = "🔲set-block/🦀️.rs"]
pub mod set_block;
#[path = "🔧set-entity/🦀️.rs"]
pub mod set_entity;
#[path = "🏷️set-header-var/🦀️.rs"]
pub mod set_header_var;
#[path = "🎚️set-layer/🦀️.rs"]
pub mod set_layer;
#[path = "🪡set-linetype/🦀️.rs"]
pub mod set_linetype;
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🖌️set-style/🦀️.rs"]
pub mod set_style;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.dxf`. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none, and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = DxfSnapshot, diff = DxfDiff, schema = "DxfMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum DxfMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),

    /// 🏷️ Creates or replaces a `$VAR` header entry.
    SetHeaderVar(set_header_var::SetHeaderVar),
    /// ➖️ Removes a `$VAR` header entry.
    RemoveHeaderVar(remove_header_var::RemoveHeaderVar),

    /// ➕️ Inserts a whole `LAYER` table entry at `index`.
    InsertLayer(insert_layer::InsertLayer),
    /// ➖️ Removes a `LAYER` table entry by name.
    RemoveLayer(remove_layer::RemoveLayer),
    /// ✏️ Replaces the WHOLE `LAYER` entry named `name` (diff is still a sparse per-field patch).
    SetLayer(set_layer::SetLayer),

    /// ➕️ Inserts a whole `STYLE` table entry at `index`.
    InsertStyle(insert_style::InsertStyle),
    /// ➖️ Removes a `STYLE` table entry by name.
    RemoveStyle(remove_style::RemoveStyle),
    /// ✏️ Replaces the WHOLE `STYLE` entry named `name`.
    SetStyle(set_style::SetStyle),

    /// ➕️ Inserts a whole `LTYPE` table entry at `index`.
    InsertLinetype(insert_linetype::InsertLinetype),
    /// ➖️ Removes an `LTYPE` table entry by name.
    RemoveLinetype(remove_linetype::RemoveLinetype),
    /// ✏️ Replaces the WHOLE `LTYPE` entry named `name`.
    SetLinetype(set_linetype::SetLinetype),

    /// ➕️ Inserts a whole entity at `index` in the top-level `ENTITIES` list.
    InsertEntity(insert_entity::InsertEntity),
    /// ➖️ Removes the entity at `index`.
    RemoveEntity(remove_entity::RemoveEntity),
    /// ✏️ Replaces the WHOLE entity at `index` (diff is `Replace` on kind change, else a
    /// sparse kind-specific patch).
    SetEntity(set_entity::SetEntity),

    /// ➕️ Inserts a whole `BLOCK` at `index`.
    InsertBlock(insert_block::InsertBlock),
    /// ➖️ Removes the `BLOCK` at `index`.
    RemoveBlock(remove_block::RemoveBlock),
    /// ✏️ Replaces the WHOLE `BLOCK` at `index`.
    SetBlock(set_block::SetBlock),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `DxfMutation` variant, declaration order — the single source of
/// truth `../../🔮️oracles/🔣️.json`'s `mutationCatalogs[].kinds` and every test-case adapter
/// duplicate against (per ticket 26/08/23/END-TO-END-TESTING-REFACTOR wave 7's registration rule:
/// the framework never parses Rust, so this constant plus `kinds_const_matches_enum_variants` below
/// is what keeps the manifest honest).
pub const KINDS: &[&str] = &[
    "set-snapshot", "patch-snapshot",
    "set-header-var",
    "remove-header-var",
    "insert-layer",
    "remove-layer",
    "set-layer",
    "insert-style",
    "remove-style",
    "set-style",
    "insert-linetype",
    "remove-linetype",
    "set-linetype",
    "insert-entity",
    "remove-entity",
    "set-entity",
    "insert-block",
    "remove-block",
    "set-block",
];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_dxf_mutation(snapshot: &mut DxfSnapshot, mutation: &DxfMutation) -> protocol::MutationOutcome<DxfDiff> {
    let outcome = <DxfMutation as Mutation<DxfSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply


//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &DxfMutation, base: &DxfSnapshot) -> protocol::MutationOutcome<DxfDiff> {
    protocol::MutationOutcome::new(match this {
        DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        DxfMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<DxfSnapshot, DxfMutation>>::diff(patch, base),

        DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name, header_var }) => {
            let existed = base.header_vars.iter().any(|v| &v.name == name);
            diff_set_header_var(base.header_vars.len(), name, header_var.clone(), existed)
        }
        DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name }) => diff_remove_header_var(name),

        DxfMutation::InsertLayer(insert_layer::InsertLayer { index, layer }) => diff_insert_layer(*index, layer.clone()),
        DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => diff_remove_layer(name),
        DxfMutation::SetLayer(set_layer::SetLayer { name, layer }) => {
            let old = base.tables.layers.iter().find(|l| &l.name == name).cloned().unwrap_or_default();
            diff_set_layer(name, layer_diff_between(&old, layer))
        }

        DxfMutation::InsertStyle(insert_style::InsertStyle { index, style }) => diff_insert_style(*index, style.clone()),
        DxfMutation::RemoveStyle(remove_style::RemoveStyle { name }) => diff_remove_style(name),
        DxfMutation::SetStyle(set_style::SetStyle { name, style }) => {
            let old = base.tables.styles.iter().find(|s| &s.name == name).cloned().unwrap_or_default();
            diff_set_style(name, style_diff_between(&old, style))
        }

        DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index, linetype }) => diff_insert_linetype(*index, linetype.clone()),
        DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name }) => diff_remove_linetype(name),
        DxfMutation::SetLinetype(set_linetype::SetLinetype { name, linetype }) => {
            let old = base.tables.linetypes.iter().find(|l| &l.name == name).cloned().unwrap_or_default();
            diff_set_linetype(name, linetype_diff_between(&old, linetype))
        }

        DxfMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => diff_insert_entity(*index, entity.clone()),
        DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index }) => diff_remove_entity(*index),
        DxfMutation::SetEntity(set_entity::SetEntity { index, entity }) => match base.entities.get(*index) {
            Some(old) => diff_set_entity(*index, entity_diff_between_pub(old, entity)),
            None => diff_insert_entity(*index, entity.clone()),
        },

        DxfMutation::InsertBlock(insert_block::InsertBlock { index, block }) => diff_insert_block(*index, block.clone()),
        DxfMutation::RemoveBlock(remove_block::RemoveBlock { index }) => diff_remove_block(*index),
        DxfMutation::SetBlock(set_block::SetBlock { index, block }) => match base.blocks.get(*index) {
            Some(old) => diff_set_block(*index, block_diff_between(old, block)),
            None => diff_insert_block(*index, block.clone()),
        },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DxfMutation, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        DxfMutation::SetSnapshot(_) => vec![DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        DxfMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<DxfSnapshot, DxfMutation>>::inverse(patch, base)?),

        DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name, .. }) => match base.header_vars.iter().find(|v| &v.name == name) {
            Some(v) => vec![DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: v.clone() })],
            None => vec![DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: name.clone() })],
        },
        DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name }) => match base.header_vars.iter().find(|v| &v.name == name) {
            Some(v) => vec![DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: v.clone() })],
            None => Vec::new(),
        },

        // 🧭️ Reads the name off the mutation's OWN payload, not `base` at `index` — `base`
        // is pre-insertion state, so `base.tables.layers[index]` (if any) is whatever WAS
        // there before, never the layer this mutation is about to insert.
        DxfMutation::InsertLayer(insert_layer::InsertLayer { layer, .. }) => vec![DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() })],
        DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => match base.tables.layers.iter().find(|l| &l.name == name) {
            Some(l) => vec![DxfMutation::InsertLayer(insert_layer::InsertLayer { index: base.tables.layers.iter().position(|x| &x.name == name).unwrap_or(base.tables.layers.len()), layer: l.clone() })],
            None => Vec::new(),
        },
        DxfMutation::SetLayer(set_layer::SetLayer { name, .. }) => match base.tables.layers.iter().find(|l| &l.name == name) {
            Some(l) => vec![DxfMutation::SetLayer(set_layer::SetLayer { name: name.clone(), layer: l.clone() })],
            None => vec![DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: name.clone() })],
        },

        DxfMutation::InsertStyle(insert_style::InsertStyle { style, .. }) => vec![DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: style.name.clone() })],
        DxfMutation::RemoveStyle(remove_style::RemoveStyle { name }) => match base.tables.styles.iter().find(|s| &s.name == name) {
            Some(s) => vec![DxfMutation::InsertStyle(insert_style::InsertStyle { index: base.tables.styles.iter().position(|x| &x.name == name).unwrap_or(base.tables.styles.len()), style: s.clone() })],
            None => Vec::new(),
        },
        DxfMutation::SetStyle(set_style::SetStyle { name, .. }) => match base.tables.styles.iter().find(|s| &s.name == name) {
            Some(s) => vec![DxfMutation::SetStyle(set_style::SetStyle { name: name.clone(), style: s.clone() })],
            None => vec![DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: name.clone() })],
        },

        DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { linetype, .. }) => vec![DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: linetype.name.clone() })],
        DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name }) => match base.tables.linetypes.iter().find(|l| &l.name == name) {
            Some(l) => vec![DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index: base.tables.linetypes.iter().position(|x| &x.name == name).unwrap_or(base.tables.linetypes.len()), linetype: l.clone() })],
            None => Vec::new(),
        },
        DxfMutation::SetLinetype(set_linetype::SetLinetype { name, .. }) => match base.tables.linetypes.iter().find(|l| &l.name == name) {
            Some(l) => vec![DxfMutation::SetLinetype(set_linetype::SetLinetype { name: name.clone(), linetype: l.clone() })],
            None => vec![DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: name.clone() })],
        },

        DxfMutation::InsertEntity(insert_entity::InsertEntity { index, .. }) => vec![DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: *index })],
        DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index }) => match base.entities.get(*index) {
            Some(e) => vec![DxfMutation::InsertEntity(insert_entity::InsertEntity { index: *index, entity: e.clone() })],
            None => Vec::new(),
        },
        DxfMutation::SetEntity(set_entity::SetEntity { index, .. }) => match base.entities.get(*index) {
            Some(e) => vec![DxfMutation::SetEntity(set_entity::SetEntity { index: *index, entity: e.clone() })],
            None => Vec::new(),
        },

        DxfMutation::InsertBlock(insert_block::InsertBlock { index, .. }) => vec![DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: *index })],
        DxfMutation::RemoveBlock(remove_block::RemoveBlock { index }) => match base.blocks.get(*index) {
            Some(b) => vec![DxfMutation::InsertBlock(insert_block::InsertBlock { index: *index, block: b.clone() })],
            None => Vec::new(),
        },
        DxfMutation::SetBlock(set_block::SetBlock { index, .. }) => match base.blocks.get(*index) {
            Some(b) => vec![DxfMutation::SetBlock(set_block::SetBlock { index: *index, block: b.clone() })],
            None => Vec::new(),
        },
    }

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs








//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `DxfMutation` values — one instance per variant (18 total, plus four
/// extra `InsertEntity` cases exercising the Polyline/Other/Solid/Insert entity kinds the base
/// `variants()` fixture didn't reach), incl. a `SetSnapshot` payload nesting a raw-retained
/// `other_tables` entry and a block with a nested entity — exercises the WHOLE grammar/protocol
/// tree end-to-end. The single source of truth reused by `op_text_binary_roundtrip_law` below AND
/// by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, so a new variant only needs adding here once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DxfMutation> {
    use crate::schema::snapshot::{DxfOtherTable, DxfTables, DxfTag, DxfValue, DxfVertex};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn demo_snapshot_for_set() -> DxfSnapshot {
        DxfSnapshot {
            schema: "stdio.dxf".into(),
            header_vars: vec![DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] }],
            tables: DxfTables { layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }], styles: vec![], linetypes: vec![] },
            other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
            blocks: vec![DxfBlock { name: "B1".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] }],
            entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }],
        }
    }

    vec![
        DxfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: demo_snapshot_for_set() }),
        DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: "$ACADVER".into(), header_var: DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1015".into() }, extra_group_codes: vec![] } }),
        DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar {
            name: "$NEWVAR".into(),
            header_var: DxfHeaderVar { name: "$NEWVAR".into(), group_code: 70, value: DxfValue::Int { value: 3 }, extra_group_codes: vec![(999, DxfValue::Str { value: "note".into() })] },
        }),
        DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: "$ACADVER".into() }),
        DxfMutation::InsertLayer(insert_layer::InsertLayer { index: 1, layer: DxfLayer { name: "L2".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] } }),
        DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: "0".into() }),
        DxfMutation::SetLayer(set_layer::SetLayer { name: "0".into(), layer: DxfLayer { name: "0".into(), color: 3, linetype: "DASHED".into(), flags: 1, unknown_group_codes: vec![] } }),
        DxfMutation::InsertStyle(insert_style::InsertStyle { index: 1, style: DxfStyle { name: "S2".into(), flags: 0, font_name: "arial".into(), unknown_group_codes: vec![] } }),
        DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: "STANDARD".into() }),
        DxfMutation::SetStyle(set_style::SetStyle { name: "STANDARD".into(), style: DxfStyle { name: "STANDARD".into(), flags: 1, font_name: "romans".into(), unknown_group_codes: vec![] } }),
        DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index: 1, linetype: DxfLinetype { name: "DASHED".into(), flags: 0, description: "Dashed".into(), unknown_group_codes: vec![] } }),
        DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: "CONTINUOUS".into() }),
        DxfMutation::SetLinetype(set_linetype::SetLinetype { name: "CONTINUOUS".into(), linetype: DxfLinetype { name: "CONTINUOUS".into(), flags: 1, description: "Solid line".into(), unknown_group_codes: vec![] } }),
        DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: DxfEntity::Arc { center: [1.0, 1.0, 0.0], radius: 2.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] } }),
        DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: 0 }),
        DxfMutation::SetEntity(set_entity::SetEntity { index: 0, entity: DxfEntity::Line { start: [9.0, 9.0, 0.0], end: [8.0, 8.0, 0.0], layer: "L2".into(), unknown_group_codes: vec![] } }),
        DxfMutation::SetEntity(set_entity::SetEntity { index: 1, entity: DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 1.0, value: "hi".into(), layer: "0".into(), unknown_group_codes: vec![] } }),
        DxfMutation::InsertBlock(insert_block::InsertBlock { index: 0, block: DxfBlock { name: "B2".into(), base_point: [1.0, 1.0, 0.0], entities: vec![], unknown_group_codes: vec![] } }),
        DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: 0 }),
        DxfMutation::SetBlock(set_block::SetBlock {
            index: 0,
            block: DxfBlock { name: "B1".into(), base_point: [5.0, 5.0, 0.0], entities: vec![DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] },
        }),
        DxfMutation::InsertEntity(insert_entity::InsertEntity {
            index: 1,
            entity: DxfEntity::Polyline {
                vertices: vec![DxfVertex { x: 0.0, y: 0.0, z: 0.0, bulge: 0.0, unknown_group_codes: vec![] }, DxfVertex { x: 1.0, y: 0.0, z: 0.0, bulge: 0.5, unknown_group_codes: vec![(8, DxfValue::Str { value: "0".into() })] }],
                closed: true,
                layer: "0".into(),
                unknown_group_codes: vec![],
            },
        }),
        DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] } }),
        // 🧭️ `index: 2` (not 3/4): `base_snapshot()` (used by `mutation_diff_law`/`inverse_law`,
        // the two OTHER generic property tests sharing this fixture) has exactly 2 entities —
        // `agg_inverse`'s `InsertEntity` arm reads the index literally off the mutation's own
        // payload (comment above, `#region 🔖️MutationTrait`), which only round-trips when the
        // requested index is `<= base.len()` (no clamping inside `generic_apply`'s `idx.min(len)`
        // needed); an out-of-range literal index here would desync `inverse_law` for a fixture
        // this demo list is also reused by, not a grammar/protocol concern.
        DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: DxfEntity::Solid { points: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], layer: "0".into(), unknown_group_codes: vec![] } }),
        DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: DxfEntity::Insert { block_name: "B1".into(), position: [1.0, 2.0, 3.0], scale: [1.0, 1.0, 1.0], rotation: 0.0, layer: "0".into(), unknown_group_codes: vec![] } }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/⭕️widens/🦀️.rs"]
mod set_snapshot_widens_the_circle_entity_radius;
//#endregion 🧪️FixtureCases
