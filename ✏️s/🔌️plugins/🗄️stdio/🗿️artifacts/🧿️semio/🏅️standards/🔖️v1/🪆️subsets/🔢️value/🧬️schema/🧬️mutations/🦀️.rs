//! 🧬️ SemioValueMutation — document mutation dispatch. Addresses a target node inside `root` via
//! a [`SemioValuePath`] (mirrors the recipe's tree-nesting rule: `NodePath` stays mutation-level,
//! each mutation's `diff()` lowers it to a nested modified-chain via [`diff_at_path`] — template
//! copied from `json`'s own `JsonMutation`/`JsonPath`, this subset's informing source). The
//! `nodes` GRAPH gets its own flat, path-free id-addressed vocabulary (`SetNode`/
//! `RemoveNode`) since it's a top-level sibling collection to `root`, not a node reachable by
//! tree descent. Every variant's `diff()` and `inverse()` is handcrafted directly against the
//! sparse [`SemioValueTreeDiff`] shape — never apply-and-capture.


use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueNode;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::ValueId;

#[cfg(test)]
use protocol::command::DiffAlgebra;
use protocol::{ColdMutation, Mutation};


//#region 🔖️Mutations
pub use semio_s_artifact_stdio_semio_value_mutation_leaves::path::{SemioValuePath, SemioValuePathSegment};
pub use semio_s_artifact_stdio_semio_value_mutation_leaves::{insert_list_item, remove_list_item, remove_map_entry, remove_node, set_map_entry, set_node, set_value};
/// 📐️ Typed content mutation for `stdio.semio.value`. `SetValue`/`SetMapEntry`/`RemoveMapEntry`/
/// `InsertListItem`/`RemoveListItem` address `root`'s own value tree via [`SemioValuePath`];
/// `SetNode`/`RemoveNode` address the top-level id-keyed `nodes` GRAPH directly (flat, no
/// path — it is not reachable by descending `root`).
/// 🪆️ Mutation-leaf migration: each variant now wraps its own `dsl::MutationLeaf` payload type
/// (`🧬️mutations/<emoji><kind>/🦀️.rs`), and `#[derive(dsl::Mutations)]` synthesizes
/// `DESCRIPTORS`/`descriptor()` from that leaf roster — required by `protocol::Mutation<P>`
/// (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:105`). `NoMutation` is dropped: the
/// derive requires every variant to wrap exactly one leaf payload, and `no` is not an approved
/// semantic verb. `OpText`/`OpBinary` stay hand-rolled below (§OpCodecs) — every variant still
/// carries a `SemioValue` and/or `SemioValuePath`, both data-carrying-enum-shaped payloads with no
/// `DslField` impl, same structural reason `SemioValueTreeDiff`'s own doc comment cites — reusing
/// `SemioValueTreeDiff`'s `pub(crate)` grammar primitives.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioValueSnapshot, diff = SemioValueTreeDiff, schema = "SemioValueMutation", reference = SemioValueSnapshot, reference_diff = SemioValueTreeDiff)]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioValueMutation {
    /// 🔁️ Replaces the whole node found at `path` (root, if empty) with `value`, regardless of
    /// its previous kind.
    SetValue(set_value::SetValue),
    /// ➕️ Sets (creating or overwriting) entry `key` on the map at `path` to `value`.
    SetMapEntry(set_map_entry::SetMapEntry),
    /// ➖️ Removes entry `key` from the map at `path`, if present.
    RemoveMapEntry(remove_map_entry::RemoveMapEntry),
    /// ➕️ Inserts `value` into the list at `path` at `index` (ascending-insert-clamped, per the
    /// normative apply contract).
    InsertListItem(insert_list_item::InsertListItem),
    /// ➖️ Removes the element at `index` from the list at `path`, if present.
    RemoveListItem(remove_list_item::RemoveListItem),
    /// ➕️ Sets (creating or overwriting) the graph node `id` to `value`.
    SetNode(set_node::SetNode),
    /// ➖️ Removes graph node `id`, if present.
    RemoveNode(remove_node::RemoveNode),
}

/// 🏷️ Kebab-case spelling of every `SemioValueMutation` variant, in declaration order — the
/// vocabulary the `semio-v1-value` mutation catalog (`../../🔣️oracle.json`) declares and
/// `🔢️mutate-semio-value`'s exhaustive test case measures itself against.
pub const KINDS: &[&str] = &["set-value", "set-map-entry", "remove-map-entry", "insert-list-item", "remove-list-item", "set-node", "remove-node"];
//#endregion 🔖️Mutations


/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_value_mutation(mutation: &SemioValueMutation, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    <SemioValueMutation as ColdMutation<SemioValueSnapshot>>::diff(mutation, base)
}

/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `🔢️mutate-semio-value`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_value_mutation(mutation: &SemioValueMutation, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioValueMutation as ColdMutation<SemioValueSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🔖️Apply

//#endregion 🔖️MutationTrait

//#region OpCodecs

//#region 🔖️OpBinaryPrimitives

//#endregion 🔖️OpBinaryPrimitives

//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ Representative `SemioValueMutation` values, one per variant, incl. nested/list/map payload
/// values, a `Ref`/`Bytes` payload, and a multi-segment `SemioValuePath` mixing both segment
/// kinds — the single source of truth reused by `op_text_binary_roundtrip_law` below AND by
/// `🎹️composer/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, same convention json's own `demo_mutation_cases` uses.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_mutation_cases() -> Vec<SemioValueMutation> {
    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: semio_s_artifact_stdio_semio_value_model::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
    }
    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioValueEntry { key: k.into(), value: v }).collect() }
    }
    fn listv(items: Vec<SemioValue>) -> SemioValue {
        SemioValue::List { items }
    }
    fn intv(lexeme: &str) -> SemioValue {
        SemioValue::Int { lexeme: lexeme.into() }
    }
    fn strv(s: &str) -> SemioValue {
        SemioValue::Str { value: s.into() }
    }
    fn node(id: &str, value: SemioValue) -> SemioValueNode {
        SemioValueNode { id: ValueId::new(id), value }
    }

    let mixed_path = vec![SemioValuePathSegment::Key { key: "outer".into() }, SemioValuePathSegment::Index { index: 2 }, SemioValuePathSegment::Key { key: "inner".into() }];
    vec![
        SemioValueMutation::SetValue(set_value::SetValue { path: vec![], value: SemioValue::Ref { id: ValueId::new("n1") } }),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: SemioValue::Float { lexeme: "2.5e10".into() }, at: None }),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: mixed_path.clone(), key: "k".into(), value: mapv(vec![("nested", strv("v"))]), at: None }),
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![SemioValuePathSegment::Key { key: "outer".into() }], key: "gone".into() }),
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: listv(vec![intv("1"), intv("2")]) }),
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: vec![SemioValuePathSegment::Index { index: 0 }], index: 3 }),
        SemioValueMutation::SetValue(set_value::SetValue { path: mixed_path, value: SemioValue::Null }),
        SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n1"), value: SemioValue::Bytes { value: vec![255, 0, 128] }, at: None }),
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n1") }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🔖️Tests
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpText};
