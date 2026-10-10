//! 🧬️ DxfMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `DxfDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, name/index-aware, reading the pre-state it needs from `base`.
//! `SetLayer`/`SetStyle`/`SetLinetype`/`SetEntity`/`SetBlock` each set a WHOLE item (not a
//! single sub-field) — their `diff()` still constructs a sparse per-field patch by comparing
//! against `base`'s current value, never a full-item replace.
//!
//! 🧪️ F6: `OpText`/`OpBinary` for `DxfMutation` are **hand-rolled** — `#[derive(dsl::DslOps)]`
//! confirmed rejected by a real `cargo check` (independent of `DxfDiff`'s own rejection above):
//! `InsertEntity`/`SetEntity`'s `entity: DxfEntity` / `InsertBlock`/`SetBlock`'s `block: DxfBlock`
//! (which itself contains `Vec<DxfEntity>`) carry the same data-carrying-enum payload DIRECTLY as
//! a variant field — `error[E0277]: the trait bound 'DxfEntity: DslField' is not satisfied` at
//! `InsertEntity{entity:DxfEntity}`/`SetEntity{entity:DxfEntity}` (recon report §3a, mutation-side
//! twin of the diff-side blocker: the derive requires `DslField` on every reachable type whether
//! it arrives via a Diff struct field or a Mutation variant field). Grammar: `keyword arg=value
//! ...` (space-separated, same shape the derive's own handcrafted-wrapper convention uses),
//! reusing `🔺️diff`'s `pub(crate)` grammar primitives rather than duplicating them a second time.

use crate::schema::diff::{block_field_changes, // 🧪️ P2-FG1: real recursive binary twins backing the upgraded `` impl below (see
    // `🔺️diff/🦀️.rs`'s `#region 🔖️ItemBinaryCodecs`/`#region 🔖️BinaryPrimitives`).
    diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_block, diff_remove_entity, diff_remove_header_var, diff_remove_layer, diff_remove_linetype, diff_remove_style, diff_set_block, diff_set_entity, diff_set_header_var, diff_set_layer, diff_set_linetype, diff_set_style, entity_field_changes, layer_field_changes, linetype_field_changes, style_field_changes, DxfDiff};































use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle};
use crate::DxfSnapshot;

use protocol::{Mutation, MutationDiff};

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
#[path = "📑set-other-tables/🦀️.rs"]
pub mod set_other_tables;
#[path = "🔧set-entity/🦀️.rs"]
pub mod set_entity;
#[path = "🏷️set-header-var/🦀️.rs"]
pub mod set_header_var;
#[path = "🎚️set-layer/🦀️.rs"]
pub mod set_layer;
#[path = "🪡set-linetype/🦀️.rs"]
pub mod set_linetype;
#[path = "🖌️set-style/🦀️.rs"]
pub mod set_style;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.dxf`. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none, and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned)]
#[mutations(snapshot = DxfSnapshot, diff = DxfDiff, schema = "DxfMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum DxfMutation {

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
    /// 📑 Replaces the raw-retained unmodeled `TABLE`s.
    SetOtherTables(set_other_tables::SetOtherTables),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `DxfMutation` variant, declaration order — the single source of
/// truth `../../🔮️oracles/🔣️.json`'s `mutationCatalogs[].kinds` and every test-case adapter
/// duplicate against (per ticket 26/08/23/END-TO-END-TESTING-REFACTOR wave 7's registration rule:
/// the framework never parses Rust, so this constant plus `kinds_const_matches_enum_variants` below
/// is what keeps the manifest honest).
pub const KINDS: &[&str] = &[
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
    "set-other-tables",
];
//#endregion 🔖️Kinds


//#endregion 🔖️Apply



//#region OpCodecs








//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `DxfMutation` values — one instance per variant (18 total, plus four
/// extra `InsertEntity` cases exercising the Polyline/Other/Solid/Insert entity kinds the base
/// `variants()` fixture didn't reach), incl. a `SetBlock` payload nesting a block with a nested entity — exercises the WHOLE grammar/protocol
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
        DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: DxfEntity::Solid { points: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], layer: "0".into(), unknown_group_codes: vec![] } }),
        DxfMutation::SetOtherTables(set_other_tables::SetOtherTables { other_tables: vec![DxfOtherTable { name: "VIEW".into(), tags: vec![DxfTag { code: 2, value: "*TOP".into() }] }] }),
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
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
