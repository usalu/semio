//! 🧬️ SemioValueMutation — document mutation dispatch. Addresses a target node inside `root` via
//! a [`SemioValuePath`] (mirrors the recipe's tree-nesting rule: `NodePath` stays mutation-level,
//! each mutation's `diff()` lowers it to a nested modified-chain via [`diff_at_path`] — template
//! copied from `json`'s own `JsonMutation`/`JsonPath`, this subset's informing source). The
//! `nodes` GRAPH gets its own flat, path-free id-addressed vocabulary (`SetNode`/
//! `RemoveNode`) since it's a top-level sibling collection to `root`, not a node reachable by
//! tree descent. Every variant's `diff()` and `inverse()` is handcrafted directly against the
//! sparse [`SemioValueTreeDiff`] shape — never apply-and-capture.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, NamedModified, NamedTripleDiff};


use crate::standards::v1::subsets::value::schema::diff::{NamedAdded, SemioValueDiff, SemioValueTreeDiff};












use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};


#[cfg(test)]
use protocol::command::DiffAlgebra;
use protocol::{Mutation};

//#region 🔖️SemioValuePath
/// 🧭️ One step of a [`SemioValuePath`] — a map key or a list position. Struct (named-field)
/// variants throughout, never bare tuple variants — same internally-tagged runtime-serialization
/// hazard `SemioValue`'s own doc comment cites.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum SemioValuePathSegment {
    Key { key: String },
    Index { index: usize },
}

/// 🧭️ Addresses a node inside a `SemioValue` tree rooted at `root`, root-to-leaf. Never crosses a
/// `Ref` boundary — dereferencing a `Ref` is a query-time concern for consumers, not something a
/// path silently flattens.
pub type SemioValuePath = Vec<SemioValuePathSegment>;

/// 🔎️ Read-only navigation of `path` from `root`, `None` on the first unresolvable segment
/// (missing key, out-of-range index, or a segment applied to the wrong node kind).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve<'a>(root: &'a SemioValue, path: &[SemioValuePathSegment]) -> Option<&'a SemioValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (SemioValuePathSegment::Key { key }, SemioValue::Map { entries }) => &entries.iter().find(|e| &e.key == key)?.value,
            (SemioValuePathSegment::Index { index }, SemioValue::List { items }) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}
//#endregion 🔖️SemioValuePath

//#region 🔖️Mutations
#[path = "➕insert-list-item/🦀️.rs"]
pub mod insert_list_item;
#[path = "➖remove-list-item/🦀️.rs"]
pub mod remove_list_item;
#[path = "✖️remove-map-entry/🦀️.rs"]
pub mod remove_map_entry;
#[path = "✂️remove-node/🦀️.rs"]
pub mod remove_node;
#[path = "🗝️set-map-entry/🦀️.rs"]
pub mod set_map_entry;
#[path = "🧷set-node/🦀️.rs"]
pub mod set_node;
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
#[path = "🔁set-value/🦀️.rs"]
pub mod set_value;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioValueSnapshot, diff = SemioValueTreeDiff, schema = "SemioValueMutation")]
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

//#region 🔖️DiffAtPath
/// 🧩 Lowers a leaf [`SemioValueDiff`] (addressing the node found at `path`) into the nested
/// modified-chain matching the recipe's tree-nesting rule — no path addressing inside diffs
/// themselves, only at the mutation level. Always targets `root`; `nodes` is untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_at_path(path: &[SemioValuePathSegment], leaf: Option<SemioValueDiff>) -> SemioValueTreeDiff {
    SemioValueTreeDiff { root: leaf.map(|leaf| wrap_at_path(path, leaf)), nodes: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_at_path(path: &[SemioValuePathSegment], leaf: SemioValueDiff) -> SemioValueDiff {
    match path.split_first() {
        None => leaf,
        Some((SemioValuePathSegment::Key { key }, rest)) => SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] } },
        Some((SemioValuePathSegment::Index { index }, rest)) => SemioValueDiff::List {
            diff: crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![crate::standards::v1::subsets::base::schema::triples::IndexModified { index: *index, diff: wrap_at_path(rest, leaf) }],
            },
        },
    }
}
//#endregion 🔖️DiffAtPath

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_value_mutation(mutation: &SemioValueMutation, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    <SemioValueMutation as protocol::Mutation<SemioValueSnapshot>>::diff(mutation, base)
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
    <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(mutation, base)?

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
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioValueMutation> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioValueEntry { key: k.into(), value: v }).collect() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn listv(items: Vec<SemioValue>) -> SemioValue {
        SemioValue::List { items }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn intv(lexeme: &str) -> SemioValue {
        SemioValue::Int { lexeme: lexeme.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strv(s: &str) -> SemioValue {
        SemioValue::Str { value: s.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpText};
