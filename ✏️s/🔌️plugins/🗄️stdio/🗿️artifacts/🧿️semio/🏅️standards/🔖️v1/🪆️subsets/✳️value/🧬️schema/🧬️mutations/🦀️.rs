//! 🧬️ SemioValueMutation — document mutation dispatch. Addresses a target node inside `root` via
//! a [`SemioValuePath`] (mirrors the recipe's tree-nesting rule: `NodePath` stays mutation-level,
//! each mutation's `diff()` lowers it to a nested modified-chain via [`diff_at_path`] — template
//! copied from `json`'s own `JsonMutation`/`JsonPath`, this subset's informing source). The
//! `nodes` GRAPH gets its own flat, path-free id-addressed vocabulary (`SetNode`/
//! `RemoveNode`) since it's a top-level sibling collection to `root`, not a node reachable by
//! tree descent. Every variant's `diff()` and `inverse()` is handcrafted directly against the
//! sparse [`SemioValueTreeDiff`] shape — never apply-and-capture.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets, IndexAdded, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::diff_set_snapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{
    dec_semio_value, dec_semio_value_bin, dec_semio_value_node_bin, dec_str, dec_value_id, enc_semio_value, enc_semio_value_bin, enc_semio_value_node_bin, enc_str, enc_value_id, read_str_lp, value_diff_between, write_str_lp, NamedAdded,
    SemioValueDiff, SemioValueTreeDiff,
};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{dec_semio_value_snapshot, enc_semio_value_snapshot, SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};
#[cfg(test)]
use protocol::command::DiffAlgebra;
use protocol::{Mutation, OpText};

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
#[path = "🟤️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔁set-value/🦀️.rs"]
pub mod set_value;
#[path = "🗝set-map-entry/🦀️.rs"]
pub mod set_map_entry;
#[path = "✖remove-map-entry/🦀️.rs"]
pub mod remove_map_entry;
#[path = "➕insert-list-item/🦀️.rs"]
pub mod insert_list_item;
#[path = "➖remove-list-item/🦀️.rs"]
pub mod remove_list_item;
#[path = "🧷set-node/🦀️.rs"]
pub mod set_node;
#[path = "✂remove-node/🦀️.rs"]
pub mod remove_node;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioValueSnapshot, diff = SemioValueTreeDiff, schema = "SemioValueMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioValueMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
/// `🍊️mutate-semio-value`'s exhaustive test case measures itself against.
pub const KINDS: &[&str] = &["set-snapshot", "set-value", "set-map-entry", "remove-map-entry", "insert-list-item", "remove-list-item", "set-node", "remove-node"];
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
            diff: crate::artifacts::semio::standards::v1::subsets::base::schema::triples::IndexedTripleDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![crate::artifacts::semio::standards::v1::subsets::base::schema::triples::IndexModified { index: *index, diff: wrap_at_path(rest, leaf) }],
            },
        },
    }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. The diff is the single semantics source: it's computed
/// once from the pre-mutation state, applied to produce the new state, and returned.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_value_mutation(snapshot: &mut SemioValueSnapshot, mutation: &SemioValueMutation) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    let outcome = <SemioValueMutation as Mutation<SemioValueSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `🍊️mutate-semio-value`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_value_mutation(mutation: &SemioValueMutation, base: &SemioValueSnapshot) -> Vec<SemioValueMutation> {
    <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioValueMutation, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    protocol::MutationOutcome::new(match this {
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),

        SemioValueMutation::SetValue(set_value::SetValue { path, value }) => match resolve(&base.root, path) {
            Some(old) if old != value => diff_at_path(path, Some(SemioValueDiff::Replace { value: value.clone() })),
            _ => SemioValueTreeDiff::default(),
        },

        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value }) => match resolve(&base.root, path) {
            Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
                Some(existing) => {
                    let leaf = value_diff_between(&existing.value, value);
                    diff_at_path(path, leaf.map(|diff| SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff }] } }))
                }
                None => diff_at_path(
                    path,
                    Some(SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: entries.len(), item: SemioValueEntry { key: key.clone(), value: value.clone() } }] } }),
                ),
            },
            _ => SemioValueTreeDiff::default(),
        },

        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }) => match resolve(&base.root, path) {
            Some(SemioValue::Map { entries }) if entries.iter().any(|e| &e.key == key) => diff_at_path(path, Some(SemioValueDiff::Map { diff: NamedTripleDiff { removed: vec![key.clone()], modified: Vec::new(), added: Vec::new() } })),
            _ => SemioValueTreeDiff::default(),
        },

        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }) => match resolve(&base.root, path) {
            Some(SemioValue::List { items }) => diff_at_path(
                path,
                Some(SemioValueDiff::List {
                    diff: crate::artifacts::semio::standards::v1::subsets::base::schema::triples::IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: (*index).min(items.len()), item: value.clone() }] },
                }),
            ),
            _ => SemioValueTreeDiff::default(),
        },

        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }) => match resolve(&base.root, path) {
            Some(SemioValue::List { items }) if *index < items.len() => {
                diff_at_path(path, Some(SemioValueDiff::List { diff: crate::artifacts::semio::standards::v1::subsets::base::schema::triples::IndexedTripleDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() } }))
            }
            _ => SemioValueTreeDiff::default(),
        },

        SemioValueMutation::SetNode(set_node::SetNode { id, value }) => match base.nodes.iter().find(|n| &n.id == id) {
            Some(existing) => match value_diff_between(&existing.value, value) {
                Some(diff) => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: id.clone(), diff }] }) },
                None => SemioValueTreeDiff::default(),
            },
            None => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.nodes.len(), item: SemioValueNode { id: id.clone(), value: value.clone() } }] }) },
        },

        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }) => {
            if base.nodes.iter().any(|n| &n.id == id) {
                SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: vec![id.clone()], modified: Vec::new(), added: Vec::new() }) }
            } else {
                SemioValueTreeDiff::default()
            }
        }
    })
}

/// ↩️ Handcrafted mutation-level inverse, key/index/id-aware — reads the pre-mutation `base` state to
/// recover the exact undo. `Vec::new()` where there is nothing to restore (the target was already
/// absent), matching the convention every other migrated subset's `agg_inverse` uses.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioValueMutation, base: &SemioValueSnapshot) -> Vec<SemioValueMutation> {
    match this {
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => vec![SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],

        SemioValueMutation::SetValue(set_value::SetValue { path, .. }) => match resolve(&base.root, path) {
            Some(old) => vec![SemioValueMutation::SetValue(set_value::SetValue { path: path.clone(), value: old.clone() })],
            None => Vec::new(),
        },

        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, .. }) => match resolve(&base.root, path) {
            Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
                Some(existing) => vec![SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: existing.value.clone() })],
                None => vec![SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: path.clone(), key: key.clone() })],
            },
            _ => Vec::new(),
        },

        // ↩️ `SetMapEntry` on an absent key always APPENDS (see `agg_diff` above), so naively
        // reinverting to a single `SetMapEntry` would restore the VALUE but lose the ORIGINAL
        // POSITION whenever other entries follow it — restore exact position by first removing
        // every entry that originally followed `key`, then re-adding `key` and each of them
        // back in original order (every re-add is an append, landing them exactly where they
        // started). Same shape `json`'s `RemoveMember` inverse documents.
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }) => match resolve(&base.root, path) {
            Some(SemioValue::Map { entries }) => match entries.iter().position(|e| &e.key == key) {
                Some(pos) => {
                    let tail: Vec<SemioValueEntry> = entries[pos + 1..].to_vec();
                    let mut steps: Vec<SemioValueMutation> = tail.iter().rev().map(|e| SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: path.clone(), key: e.key.clone() })).collect();
                    steps.push(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: entries[pos].value.clone() }));
                    steps.extend(tail.into_iter().map(|e| SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: e.key, value: e.value })));
                    steps
                }
                None => Vec::new(),
            },
            _ => Vec::new(),
        },

        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, .. }) => match resolve(&base.root, path) {
            Some(SemioValue::List { items }) => vec![SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: path.clone(), index: (*index).min(items.len()) })],
            _ => Vec::new(),
        },

        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }) => match resolve(&base.root, path) {
            Some(SemioValue::List { items }) => match items.get(*index) {
                Some(item) => vec![SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: path.clone(), index: *index, value: item.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        },

        SemioValueMutation::SetNode(set_node::SetNode { id, .. }) => match base.nodes.iter().find(|n| &n.id == id) {
            Some(existing) => vec![SemioValueMutation::SetNode(set_node::SetNode { id: id.clone(), value: existing.value.clone() })],
            None => vec![SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: id.clone() })],
        },

        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }) => match base.nodes.iter().find(|n| &n.id == id) {
            Some(existing) => vec![SemioValueMutation::SetNode(set_node::SetNode { id: id.clone(), value: existing.value.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `SemioValueMutation` (`#[derive(dsl::DslOps)]` blocked,
/// see the enum doc comment above) — reuses `SemioValueTreeDiff`'s `pub(crate)` grammar primitives
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant — same shape `JsonMutation`'s hand-rolled codec
/// uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_segment(seg: &SemioValuePathSegment) -> String {
    match seg {
        SemioValuePathSegment::Key { key } => format!("K[{}]", enc_str(key)),
        SemioValuePathSegment::Index { index } => format!("I[{index}]"),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_segment(s: &str) -> Result<SemioValuePathSegment, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "K" => Ok(SemioValuePathSegment::Key { key: dec_str(strip_brackets(rest)?)? }),
        "I" => Ok(SemioValuePathSegment::Index { index: strip_brackets(rest)?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        other => Err(format!("semio value path segment: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path(p: &SemioValuePath) -> String {
    format!("[{}]", p.iter().map(enc_path_segment).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path(s: &str) -> Result<SemioValuePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}
/// 🧭️ `enc_semio_snapshot`/`dec_semio_snapshot` — thin aliases for the single-source-of-truth
/// `SemioValueSnapshot` text codec now owned by the sibling `📸️snapshot/🦀️.rs` (also
/// reused there by `ArtifactDsl`/`ArtifactPack`), rather than a second independent copy.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_semio_snapshot(s: &SemioValueSnapshot) -> String {
    enc_semio_value_snapshot(s)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_semio_snapshot(s: &str) -> Result<SemioValueSnapshot, String> {
    dec_semio_value_snapshot(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_value_mutation(m: &SemioValueMutation) -> String {
    match m {
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_semio_snapshot(snapshot)),
        SemioValueMutation::SetValue(set_value::SetValue { path, value }) => format!("set-value path={} value={}", enc_path(path), enc_semio_value(value)),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value }) => {
            format!("set-map-entry path={} key={} value={}", enc_path(path), enc_str(key), enc_semio_value(value))
        }
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }) => format!("remove-map-entry path={} key={}", enc_path(path), enc_str(key)),
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }) => {
            format!("insert-list-item path={} index={index} value={}", enc_path(path), enc_semio_value(value))
        }
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }) => format!("remove-list-item path={} index={index}", enc_path(path)),
        SemioValueMutation::SetNode(set_node::SetNode { id, value }) => format!("set-node id={} value={}", enc_value_id(id), enc_semio_value(value)),
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }) => format!("remove-node id={}", enc_value_id(id)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_value_mutation(line: &str) -> Result<SemioValueMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio value mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio value mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_semio_snapshot(arg("snapshot")?)? })),
        "set-value" => Ok(SemioValueMutation::SetValue(set_value::SetValue { path: dec_path(arg("path")?)?, value: dec_semio_value(arg("value")?)? })),
        "set-map-entry" => Ok(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)?, value: dec_semio_value(arg("value")?)? })),
        "remove-map-entry" => Ok(SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)? })),
        "insert-list-item" => Ok(SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")?, value: dec_semio_value(arg("value")?)? })),
        "remove-list-item" => Ok(SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")? })),
        "set-node" => Ok(SemioValueMutation::SetNode(set_node::SetNode { id: dec_value_id(arg("id")?)?, value: dec_semio_value(arg("value")?)? })),
        "remove-node" => Ok(SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: dec_value_id(arg("id")?)? })),
        other => Err(format!("semio value mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioValueMutation {
    fn print_op(&self) -> String {
        print_value_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_value_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryPrimitives
/// 🧭️ Real recursive binary twin of [`enc_path`]/[`dec_path`] — a varint segment COUNT, then per
/// segment a 1-byte kind tag (`0`=Key/`1`=Index) and its own real payload. Template copied from
/// json's own `enc_json_path_bin`/`dec_json_path_bin`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_semio_path_bin(path: &[SemioValuePathSegment], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, path.len() as u64);
    for segment in path {
        match segment {
            SemioValuePathSegment::Key { key } => {
                out.push(0);
                write_str_lp(out, key);
            }
            SemioValuePathSegment::Index { index } => {
                out.push(1);
                store::pack_rt::write_varint_u64(out, *index as u64);
            }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_semio_path_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValuePath, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let tag = reader.read_u8().map_err(|e| e.to_string())?;
        match tag {
            0 => path.push(SemioValuePathSegment::Key { key: read_str_lp(reader)? }),
            1 => path.push(SemioValuePathSegment::Index { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
            other => return Err(format!("semio value path binary: unknown segment tag {other}")),
        }
    }
    Ok(path)
}

/// 🧭️ Real recursive binary twin of [`enc_semio_snapshot`]/[`dec_semio_snapshot`] — used ONLY by
/// `SetSnapshot`'s own `OpBinary` payload (the sibling `📸️snapshot/🦀️.rs`'s own
/// `ArtifactPack` stays text-native, matching `json`'s exact precedent — see that file's doc
/// comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_semio_value_snapshot_bin(s: &SemioValueSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    enc_semio_value_bin(&s.root, out);
    store::pack_rt::write_varint_u64(out, s.nodes.len() as u64);
    for node in &s.nodes {
        enc_semio_value_node_bin(node, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_semio_value_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValueSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let root = dec_semio_value_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut nodes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        nodes.push(dec_semio_value_node_bin(reader)?);
    }
    Ok(SemioValueSnapshot { schema, root, nodes })
}
//#endregion 🔖️OpBinaryPrimitives

/// 🧪️ Real binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the `print_op().into_bytes()` text-as-binary shortcut this facet started with.
/// `tag` is the `SemioValueMutation` variant ordinal, in the same 0-7 order
/// `print_value_mutation`'s own keyword match uses. Every variant's own path/key/value/id payload
/// is real LEB128-varint-framed binary (never text-as-bytes) — same treatment json's own
/// `JsonMutation::encode_op`/`decode_op` uses.
impl protocol::OpBinary for SemioValueMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => 0,
            SemioValueMutation::SetValue(set_value::SetValue { .. }) => 1,
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { .. }) => 2,
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { .. }) => 3,
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { .. }) => 4,
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { .. }) => 5,
            SemioValueMutation::SetNode(set_node::SetNode { .. }) => 6,
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { .. }) => 7,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_semio_value_snapshot_bin(snapshot, &mut out),
            SemioValueMutation::SetValue(set_value::SetValue { path, value }) => {
                enc_semio_path_bin(path, &mut out);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value }) => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }) => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
            }
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }) => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }) => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            SemioValueMutation::SetNode(set_node::SetNode { id, value }) => {
                write_str_lp(&mut out, &id.value);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }) => {
                write_str_lp(&mut out, &id.value);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => {
                let snapshot = dec_semio_value_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            1 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetValue(set_value::SetValue { path, value }))
            }
            2 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value }))
            }
            3 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                Ok(SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }))
            }
            4 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }))
            }
            5 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }))
            }
            6 => {
                let id = ValueId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetNode(set_node::SetNode { id, value }))
            }
            7 => {
                let id = ValueId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                Ok(SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }))
            }
            other => Err(malformed("op tag", 1, format!("unknown op tag {other}"))),
        }
    }
}
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
        SemioValueSnapshot { schema: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
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
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snap(mapv(vec![("a", intv("1")), ("b", listv(vec![strv("x"), SemioValue::Null, SemioValue::Bool { value: true }]))]), vec![node("n1", SemioValue::Bytes { value: vec![1, 2, 3] })]) }),
        SemioValueMutation::SetValue(set_value::SetValue { path: vec![], value: SemioValue::Ref { id: ValueId::new("n1") } }),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: SemioValue::Float { lexeme: "2.5e10".into() } }),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: mixed_path.clone(), key: "k".into(), value: mapv(vec![("nested", strv("v"))]) }),
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![SemioValuePathSegment::Key { key: "outer".into() }], key: "gone".into() }),
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: listv(vec![intv("1"), intv("2")]) }),
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: vec![SemioValuePathSegment::Index { index: 0 }], index: 3 }),
        SemioValueMutation::SetValue(set_value::SetValue { path: mixed_path, value: SemioValue::Null }),
        SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n1"), value: SemioValue::Bytes { value: vec![255, 0, 128] } }),
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n1") }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_fixture() -> SemioValueSnapshot {
        snap(mapv(vec![("a", intv("1")), ("list", listv(vec![intv("1"), intv("2")]))]), vec![node("n1", strv("hello"))])
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply_and_check(base: &SemioValueSnapshot, mutation: SemioValueMutation) -> (SemioValueSnapshot, protocol::MutationOutcome<SemioValueTreeDiff>) {
        let mut via_apply = base.clone();
        let returned = apply_semio_value_mutation(&mut via_apply, &mutation);
        let expected_diff = mutation.diff(base);
        assert_eq!(returned, expected_diff, "apply_semio_value_mutation must return mutation.diff(base)");
        let via_diff_apply = expected_diff.diff().apply(base).expect("apply must succeed for a well-formed fixture");
        assert_eq!(via_apply, via_diff_apply, "m.diff(base).diff().apply(base) must equal apply_semio_value_mutation's result");
        (via_apply, returned)
    }

    //#region mutation_diff_law
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_all_variants() {
        let base = base_fixture();

        apply_and_check(&base, SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snap(SemioValue::Bool { value: true }, vec![]) }));
        apply_and_check(&base, SemioValueMutation::SetValue(set_value::SetValue { path: vec![SemioValuePathSegment::Key { key: "a".into() }], value: intv("2") }));
        apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") }));
        apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") }));
        apply_and_check(&base, SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![], key: "a".into() }));
        apply_and_check(&base, SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: intv("99") }));
        apply_and_check(&base, SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 0 }));
        apply_and_check(&base, SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n1"), value: strv("updated") }));
        apply_and_check(&base, SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n2"), value: strv("brand-new") }));
        apply_and_check(&base, SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n1") }));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_map_entry_on_missing_key_adds_at_end() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let (result, _) = apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "b".into(), value: intv("2") }));
        assert_eq!(result.root, mapv(vec![("a", intv("1")), ("b", intv("2"))]));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_map_entry_missing_key_is_noop() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let (result, diff) = apply_and_check(&base, SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![], key: "missing".into() }));
        assert_eq!(result, base);
        assert!(diff.diff().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_path_targets_inner_entry() {
        let base = snap(mapv(vec![("outer", mapv(vec![("inner", intv("1"))]))]), vec![]);
        let (result, _) = apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![SemioValuePathSegment::Key { key: "outer".into() }], key: "inner".into(), value: intv("42") }));
        assert_eq!(result.root, mapv(vec![("outer", mapv(vec![("inner", intv("42"))]))]));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_node_on_missing_id_inserts() {
        let base = snap(SemioValue::Null, vec![node("n1", strv("a"))]);
        let (result, _) = apply_and_check(&base, SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n2"), value: strv("b") }));
        assert_eq!(result.nodes, vec![node("n1", strv("a")), node("n2", strv("b"))]);
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_mutation_level_round_trips() {
        let base = base_fixture();
        let mutations = vec![
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") }),
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") }),
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![], key: "a".into() }),
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: intv("99") }),
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 0 }),
            SemioValueMutation::SetValue(set_value::SetValue { path: vec![SemioValuePathSegment::Key { key: "a".into() }], value: strv("replaced") }),
            SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n1"), value: strv("updated") }),
            SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n9"), value: strv("brand-new") }),
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n1") }),
        ];
        for mutation in mutations {
            let mut state = base.clone();
            apply_semio_value_mutation(&mut state, &mutation);
            for undo in <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(&mutation, &base) {
                apply_semio_value_mutation(&mut state, &undo);
            }
            assert_eq!(state, base, "mutation {mutation:?} did not round-trip via its inverse");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law_diff_level_matches_mutation_diff() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mutation = SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") });
        let diff = mutation.diff(&base);
        let mid = diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture");
        let inv = diff.diff().inverse(&base);
        assert_eq!(inv.apply(&mid).expect("apply must succeed for a well-formed fixture"), base);
    }
    //#endregion inverse_law

    //#region 🔖️OpCodecTests
    /// 🧪️ op_text_binary_roundtrip_law: exercises every variant, incl. nested/list/map payload
    /// values, a `Ref`/`Bytes` payload, and a multi-segment `SemioValuePath` mixing both segment
    /// kinds.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        use protocol::{OpBinary, OpText};

        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioValueMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <SemioValueMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch");
        }
    }
    //#endregion 🔖️OpCodecTests

    //#region 🔖️CatalogLaw
    /// 🏷️ The wildcard-free spelling map that makes [`KINDS`] compiler-checked: a new variant has
    /// no arm here, so the crate stops building until both this match and `KINDS` name it.
    fn kind_of(mutation: &SemioValueMutation) -> &'static str {
        match mutation {
            SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => "set-snapshot",
            SemioValueMutation::SetValue(set_value::SetValue { .. }) => "set-value",
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { .. }) => "set-map-entry",
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { .. }) => "remove-map-entry",
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { .. }) => "insert-list-item",
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { .. }) => "remove-list-item",
            SemioValueMutation::SetNode(set_node::SetNode { .. }) => "set-node",
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { .. }) => "remove-node",
        }
    }

    /// 🏷️ `KINDS` must name every declared variant, in declaration order and in the exact spelling
    /// the committed `semio-v1-value` catalog carries — the framework never parses Rust, so this is
    /// the only thing that keeps the catalog honest against the enum.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let one_per_variant = [
            SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioValueSnapshot::default() }),
            SemioValueMutation::SetValue(set_value::SetValue { path: Vec::new(), value: SemioValue::Null }),
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: Vec::new(), key: "status".into(), value: SemioValue::Null }),
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: Vec::new(), key: "status".into() }),
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: Vec::new(), index: 0, value: SemioValue::Null }),
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: Vec::new(), index: 0 }),
            SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n-1"), value: SemioValue::Null }),
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n-1") }),
        ];
        assert_eq!(KINDS.len(), one_per_variant.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, mutation) in KINDS.iter().zip(one_per_variant.iter()) {
            assert_eq!(*kind, kind_of(mutation), "KINDS must follow the enum's own declaration order and kebab-case spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🔖️CatalogLaw
}
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `🟤️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "🟤️set-snapshot/🧪️tests/🌳️retypes-a-map-member-and-repoints-a-graph-node/🦀️.rs"]
mod set_snapshot_retypes_a_map_member_and_repoints_a_graph_node;
//#endregion 🧪️FixtureCases
