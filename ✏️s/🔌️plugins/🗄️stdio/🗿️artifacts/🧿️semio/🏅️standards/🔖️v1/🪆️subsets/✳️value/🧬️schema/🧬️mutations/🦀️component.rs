//! 🧬️ SemioValueMutation — document mutation dispatch. Addresses a target node inside `root` via
//! a [`SemioValuePath`] (mirrors the recipe's tree-nesting rule: `NodePath` stays mutation-level,
//! each mutation's `diff()` lowers it to a nested modified-chain via [`diff_at_path`] — template
//! copied from `json`'s own `JsonMutation`/`JsonPath`, this subset's informing source). The
//! `nodes` GRAPH gets its own flat, path-free id-addressed vocabulary (`SetNode`/
//! `RemoveNode`) since it's a top-level sibling collection to `root`, not a node reachable by
//! tree descent. Every variant's `diff()` and `inverse()` is handcrafted directly against the
//! sparse [`SemioValueTreeDiff`] shape — never apply-and-capture.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets, IndexAdded, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::diff_set_snapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{
    dec_semio_value, dec_semio_value_bin, dec_semio_value_node_bin, dec_str, dec_value_id, enc_semio_value, enc_semio_value_bin, enc_semio_value_node_bin, enc_str, enc_value_id, read_str_lp, value_diff_between, write_str_lp, NamedAdded,
    SemioValueDiff, SemioValueTreeDiff,
};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{dec_semio_value_snapshot, enc_semio_value_snapshot, SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};
#[cfg(test)]
use protocol::command::DiffAlgebra;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️SemioValuePath
/// 🧭️ One step of a [`SemioValuePath`] — a map key or a list position. Struct (named-field)
/// variants throughout, never bare tuple variants — same internally-tagged runtime-serialization
/// hazard `SemioValue`'s own doc comment cites.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
/// 🧪️ `#[derive(dsl::DslOps)]` is unusable here for the same structural reason as `SemioValueTreeDiff`
/// (see that file's doc comment): every variant carries a `SemioValue` and/or `SemioValuePath`
/// directly, both data-carrying-enum-shaped payloads with no `DslField` impl. `OpText`/`OpBinary`
/// hand-rolled below (§OpCodecs), grammar template copied from `JsonMutation`'s, reusing
/// `SemioValueTreeDiff`'s `pub(crate)` grammar primitives.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioValueMutation {
    NoMutation,
    SetSnapshot {
        snapshot: SemioValueSnapshot,
    },
    /// 🔁️ Replaces the whole node found at `path` (root, if empty) with `value`, regardless of
    /// its previous kind.
    SetValue {
        path: SemioValuePath,
        value: SemioValue,
    },
    /// ➕️ Sets (creating or overwriting) entry `key` on the map at `path` to `value`.
    SetMapEntry {
        path: SemioValuePath,
        key: String,
        value: SemioValue,
    },
    /// ➖️ Removes entry `key` from the map at `path`, if present.
    RemoveMapEntry {
        path: SemioValuePath,
        key: String,
    },
    /// ➕️ Inserts `value` into the list at `path` at `index` (ascending-insert-clamped, per the
    /// normative apply contract).
    InsertListItem {
        path: SemioValuePath,
        index: usize,
        value: SemioValue,
    },
    /// ➖️ Removes the element at `index` from the list at `path`, if present.
    RemoveListItem {
        path: SemioValuePath,
        index: usize,
    },
    /// ➕️ Sets (creating or overwriting) the graph node `id` to `value`.
    SetNode {
        id: ValueId,
        value: SemioValue,
    },
    /// ➖️ Removes graph node `id`, if present.
    RemoveNode {
        id: ValueId,
    },
}

impl Default for SemioValueMutation {
    fn default() -> Self {
        SemioValueMutation::NoMutation
    }
}
//#endregion 🔖️Mutations

//#region 🔖️DiffAtPath
/// 🧩 Lowers a leaf [`SemioValueDiff`] (addressing the node found at `path`) into the nested
/// modified-chain matching the recipe's tree-nesting rule — no path addressing inside diffs
/// themselves, only at the mutation level. Always targets `root`; `nodes` is untouched.
fn diff_at_path(path: &[SemioValuePathSegment], leaf: Option<SemioValueDiff>) -> SemioValueTreeDiff {
    SemioValueTreeDiff { root: leaf.map(|leaf| wrap_at_path(path, leaf)), nodes: None }
}

fn wrap_at_path(path: &[SemioValuePathSegment], leaf: SemioValueDiff) -> SemioValueDiff {
    match path.split_first() {
        None => leaf,
        Some((SemioValuePathSegment::Key { key }, rest)) => SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] } },
        Some((SemioValuePathSegment::Index { index }, rest)) => SemioValueDiff::List {
            diff: crate::artifacts::semio::standards::v1::subsets::any::schema::triples::IndexedTripleDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![crate::artifacts::semio::standards::v1::subsets::any::schema::triples::IndexModified { index: *index, diff: wrap_at_path(rest, leaf) }],
            },
        },
    }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. The diff is the single semantics source: it's computed
/// once from the pre-mutation state, applied to produce the new state, and returned.
pub fn apply_semio_value_mutation(snapshot: &mut SemioValueSnapshot, mutation: &SemioValueMutation) -> protocol::MutationOutcome<SemioValueTreeDiff> {
    let outcome = <SemioValueMutation as Mutation<SemioValueSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SemioValueSnapshot> for SemioValueMutation {
    type Diff = SemioValueTreeDiff;

    fn diff(&self, base: &SemioValueSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            SemioValueMutation::NoMutation => SemioValueTreeDiff::default(),
            SemioValueMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            SemioValueMutation::SetValue { path, value } => match resolve(&base.root, path) {
                Some(old) if old != value => diff_at_path(path, Some(SemioValueDiff::Replace { value: value.clone() })),
                _ => SemioValueTreeDiff::default(),
            },

            SemioValueMutation::SetMapEntry { path, key, value } => match resolve(&base.root, path) {
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

            SemioValueMutation::RemoveMapEntry { path, key } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) if entries.iter().any(|e| &e.key == key) => diff_at_path(path, Some(SemioValueDiff::Map { diff: NamedTripleDiff { removed: vec![key.clone()], modified: Vec::new(), added: Vec::new() } })),
                _ => SemioValueTreeDiff::default(),
            },

            SemioValueMutation::InsertListItem { path, index, value } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) => diff_at_path(
                    path,
                    Some(SemioValueDiff::List {
                        diff: crate::artifacts::semio::standards::v1::subsets::any::schema::triples::IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: (*index).min(items.len()), item: value.clone() }] },
                    }),
                ),
                _ => SemioValueTreeDiff::default(),
            },

            SemioValueMutation::RemoveListItem { path, index } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) if *index < items.len() => {
                    diff_at_path(path, Some(SemioValueDiff::List { diff: crate::artifacts::semio::standards::v1::subsets::any::schema::triples::IndexedTripleDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() } }))
                }
                _ => SemioValueTreeDiff::default(),
            },

            SemioValueMutation::SetNode { id, value } => match base.nodes.iter().find(|n| &n.id == id) {
                Some(existing) => match value_diff_between(&existing.value, value) {
                    Some(diff) => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: id.clone(), diff }] }) },
                    None => SemioValueTreeDiff::default(),
                },
                None => SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.nodes.len(), item: SemioValueNode { id: id.clone(), value: value.clone() } }] }) },
            },

            SemioValueMutation::RemoveNode { id } => {
                if base.nodes.iter().any(|n| &n.id == id) {
                    SemioValueTreeDiff { root: None, nodes: Some(NamedTripleDiff { removed: vec![id.clone()], modified: Vec::new(), added: Vec::new() }) }
                } else {
                    SemioValueTreeDiff::default()
                }
            }
        })
    }

    /// ↩️ Handcrafted mutation-level inverse, key/index/id-aware — reads the pre-mutation `base`
    /// state to recover the exact undo.
    fn inverse(&self, base: &SemioValueSnapshot) -> Vec<Self> {
        match self {
            SemioValueMutation::NoMutation => vec![SemioValueMutation::NoMutation],
            SemioValueMutation::SetSnapshot { .. } => vec![SemioValueMutation::SetSnapshot { snapshot: base.clone() }],

            SemioValueMutation::SetValue { path, .. } => match resolve(&base.root, path) {
                Some(old) => vec![SemioValueMutation::SetValue { path: path.clone(), value: old.clone() }],
                None => vec![SemioValueMutation::NoMutation],
            },

            SemioValueMutation::SetMapEntry { path, key, .. } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
                    Some(existing) => vec![SemioValueMutation::SetMapEntry { path: path.clone(), key: key.clone(), value: existing.value.clone() }],
                    None => vec![SemioValueMutation::RemoveMapEntry { path: path.clone(), key: key.clone() }],
                },
                _ => vec![SemioValueMutation::NoMutation],
            },

            // ↩️ `SetMapEntry` on an absent key always APPENDS (see `diff()` above), so naively
            // reinverting to a single `SetMapEntry` would restore the VALUE but lose the ORIGINAL
            // POSITION whenever other entries follow it — restore exact position by first removing
            // every entry that originally followed `key`, then re-adding `key` and each of them
            // back in original order (every re-add is an append, landing them exactly where they
            // started). Same shape `json`'s `RemoveMember` inverse documents.
            SemioValueMutation::RemoveMapEntry { path, key } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) => match entries.iter().position(|e| &e.key == key) {
                    Some(pos) => {
                        let tail: Vec<SemioValueEntry> = entries[pos + 1..].to_vec();
                        let mut steps: Vec<SemioValueMutation> = tail.iter().rev().map(|e| SemioValueMutation::RemoveMapEntry { path: path.clone(), key: e.key.clone() }).collect();
                        steps.push(SemioValueMutation::SetMapEntry { path: path.clone(), key: key.clone(), value: entries[pos].value.clone() });
                        steps.extend(tail.into_iter().map(|e| SemioValueMutation::SetMapEntry { path: path.clone(), key: e.key, value: e.value }));
                        steps
                    }
                    None => vec![SemioValueMutation::NoMutation],
                },
                _ => vec![SemioValueMutation::NoMutation],
            },

            SemioValueMutation::InsertListItem { path, index, .. } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) => vec![SemioValueMutation::RemoveListItem { path: path.clone(), index: (*index).min(items.len()) }],
                _ => vec![SemioValueMutation::NoMutation],
            },

            SemioValueMutation::RemoveListItem { path, index } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) => match items.get(*index) {
                    Some(item) => vec![SemioValueMutation::InsertListItem { path: path.clone(), index: *index, value: item.clone() }],
                    None => vec![SemioValueMutation::NoMutation],
                },
                _ => vec![SemioValueMutation::NoMutation],
            },

            SemioValueMutation::SetNode { id, .. } => match base.nodes.iter().find(|n| &n.id == id) {
                Some(existing) => vec![SemioValueMutation::SetNode { id: id.clone(), value: existing.value.clone() }],
                None => vec![SemioValueMutation::RemoveNode { id: id.clone() }],
            },

            SemioValueMutation::RemoveNode { id } => match base.nodes.iter().find(|n| &n.id == id) {
                Some(existing) => vec![SemioValueMutation::SetNode { id: id.clone(), value: existing.value.clone() }],
                None => vec![SemioValueMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `SemioValueMutation` (`#[derive(dsl::DslOps)]` blocked,
/// see the enum doc comment above) — reuses `SemioValueTreeDiff`'s `pub(crate)` grammar primitives
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant — same shape `JsonMutation`'s hand-rolled codec
/// uses.
fn enc_path_segment(seg: &SemioValuePathSegment) -> String {
    match seg {
        SemioValuePathSegment::Key { key } => format!("K[{}]", enc_str(key)),
        SemioValuePathSegment::Index { index } => format!("I[{index}]"),
    }
}
fn dec_path_segment(s: &str) -> Result<SemioValuePathSegment, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "K" => Ok(SemioValuePathSegment::Key { key: dec_str(strip_brackets(rest)?)? }),
        "I" => Ok(SemioValuePathSegment::Index { index: strip_brackets(rest)?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        other => Err(format!("semio value path segment: unknown tag {other:?}")),
    }
}
fn enc_path(p: &SemioValuePath) -> String {
    format!("[{}]", p.iter().map(enc_path_segment).collect::<Vec<_>>().join(","))
}
fn dec_path(s: &str) -> Result<SemioValuePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}
/// 🧭️ `enc_semio_snapshot`/`dec_semio_snapshot` — thin aliases for the single-source-of-truth
/// `SemioValueSnapshot` text codec now owned by the sibling `📸️snapshot/🦀️component.rs` (also
/// reused there by `ArtifactDsl`/`ArtifactPack`), rather than a second independent copy.
fn enc_semio_snapshot(s: &SemioValueSnapshot) -> String {
    enc_semio_value_snapshot(s)
}
fn dec_semio_snapshot(s: &str) -> Result<SemioValueSnapshot, String> {
    dec_semio_value_snapshot(s)
}

fn print_value_mutation(m: &SemioValueMutation) -> String {
    match m {
        SemioValueMutation::NoMutation => "no-mutation".to_string(),
        SemioValueMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_semio_snapshot(snapshot)),
        SemioValueMutation::SetValue { path, value } => format!("set-value path={} value={}", enc_path(path), enc_semio_value(value)),
        SemioValueMutation::SetMapEntry { path, key, value } => {
            format!("set-map-entry path={} key={} value={}", enc_path(path), enc_str(key), enc_semio_value(value))
        }
        SemioValueMutation::RemoveMapEntry { path, key } => format!("remove-map-entry path={} key={}", enc_path(path), enc_str(key)),
        SemioValueMutation::InsertListItem { path, index, value } => {
            format!("insert-list-item path={} index={index} value={}", enc_path(path), enc_semio_value(value))
        }
        SemioValueMutation::RemoveListItem { path, index } => format!("remove-list-item path={} index={index}", enc_path(path)),
        SemioValueMutation::SetNode { id, value } => format!("set-node id={} value={}", enc_value_id(id), enc_semio_value(value)),
        SemioValueMutation::RemoveNode { id } => format!("remove-node id={}", enc_value_id(id)),
    }
}
fn parse_value_mutation(line: &str) -> Result<SemioValueMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioValueMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio value mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio value mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SemioValueMutation::SetSnapshot { snapshot: dec_semio_snapshot(arg("snapshot")?)? }),
        "set-value" => Ok(SemioValueMutation::SetValue { path: dec_path(arg("path")?)?, value: dec_semio_value(arg("value")?)? }),
        "set-map-entry" => Ok(SemioValueMutation::SetMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)?, value: dec_semio_value(arg("value")?)? }),
        "remove-map-entry" => Ok(SemioValueMutation::RemoveMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)? }),
        "insert-list-item" => Ok(SemioValueMutation::InsertListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")?, value: dec_semio_value(arg("value")?)? }),
        "remove-list-item" => Ok(SemioValueMutation::RemoveListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")? }),
        "set-node" => Ok(SemioValueMutation::SetNode { id: dec_value_id(arg("id")?)?, value: dec_semio_value(arg("value")?)? }),
        "remove-node" => Ok(SemioValueMutation::RemoveNode { id: dec_value_id(arg("id")?)? }),
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
/// `SetSnapshot`'s own `OpBinary` payload (the sibling `📸️snapshot/🦀️component.rs`'s own
/// `ArtifactPack` stays text-native, matching `json`'s exact precedent — see that file's doc
/// comment).
fn enc_semio_value_snapshot_bin(s: &SemioValueSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    enc_semio_value_bin(&s.root, out);
    store::pack_rt::write_varint_u64(out, s.nodes.len() as u64);
    for node in &s.nodes {
        enc_semio_value_node_bin(node, out);
    }
}
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
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the `print_op().into_bytes()` text-as-binary shortcut this facet started with.
/// `tag` is the `SemioValueMutation` variant ordinal, in the same 0-8 order
/// `print_value_mutation`'s own keyword match uses. Every variant's own path/key/value/id payload
/// is real LEB128-varint-framed binary (never text-as-bytes) — same treatment json's own
/// `JsonMutation::encode_op`/`decode_op` uses.
impl protocol::OpBinary for SemioValueMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            SemioValueMutation::NoMutation => 0,
            SemioValueMutation::SetSnapshot { .. } => 1,
            SemioValueMutation::SetValue { .. } => 2,
            SemioValueMutation::SetMapEntry { .. } => 3,
            SemioValueMutation::RemoveMapEntry { .. } => 4,
            SemioValueMutation::InsertListItem { .. } => 5,
            SemioValueMutation::RemoveListItem { .. } => 6,
            SemioValueMutation::SetNode { .. } => 7,
            SemioValueMutation::RemoveNode { .. } => 8,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            SemioValueMutation::NoMutation => {}
            SemioValueMutation::SetSnapshot { snapshot } => enc_semio_value_snapshot_bin(snapshot, &mut out),
            SemioValueMutation::SetValue { path, value } => {
                enc_semio_path_bin(path, &mut out);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::SetMapEntry { path, key, value } => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveMapEntry { path, key } => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
            }
            SemioValueMutation::InsertListItem { path, index, value } => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveListItem { path, index } => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            SemioValueMutation::SetNode { id, value } => {
                write_str_lp(&mut out, &id.value);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveNode { id } => {
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
            0 => Ok(SemioValueMutation::NoMutation),
            1 => {
                let snapshot = dec_semio_value_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(SemioValueMutation::SetSnapshot { snapshot })
            }
            2 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetValue { path, value })
            }
            3 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetMapEntry { path, key, value })
            }
            4 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                Ok(SemioValueMutation::RemoveMapEntry { path, key })
            }
            5 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::InsertListItem { path, index, value })
            }
            6 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(SemioValueMutation::RemoveListItem { path, index })
            }
            7 => {
                let id = ValueId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetNode { id, value })
            }
            8 => {
                let id = ValueId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                Ok(SemioValueMutation::RemoveNode { id })
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
/// `🎹️composer/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, same convention json's own `demo_mutation_cases` uses.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioValueMutation> {
    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
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
        SemioValueMutation::NoMutation,
        SemioValueMutation::SetSnapshot { snapshot: snap(mapv(vec![("a", intv("1")), ("b", listv(vec![strv("x"), SemioValue::Null, SemioValue::Bool { value: true }]))]), vec![node("n1", SemioValue::Bytes { value: vec![1, 2, 3] })]) },
        SemioValueMutation::SetValue { path: vec![], value: SemioValue::Ref { id: ValueId::new("n1") } },
        SemioValueMutation::SetMapEntry { path: vec![], key: "a".into(), value: SemioValue::Float { lexeme: "2.5e10".into() } },
        SemioValueMutation::SetMapEntry { path: mixed_path.clone(), key: "k".into(), value: mapv(vec![("nested", strv("v"))]) },
        SemioValueMutation::RemoveMapEntry { path: vec![SemioValuePathSegment::Key { key: "outer".into() }], key: "gone".into() },
        SemioValueMutation::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: listv(vec![intv("1"), intv("2")]) },
        SemioValueMutation::RemoveListItem { path: vec![SemioValuePathSegment::Index { index: 0 }], index: 3 },
        SemioValueMutation::SetValue { path: mixed_path, value: SemioValue::Null },
        SemioValueMutation::SetNode { id: ValueId::new("n1"), value: SemioValue::Bytes { value: vec![255, 0, 128] } },
        SemioValueMutation::RemoveNode { id: ValueId::new("n1") },
    ]
}
//#endregion 🔖️DemoCases

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
    use protocol::MutationDiff;

    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
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

    fn base_fixture() -> SemioValueSnapshot {
        snap(mapv(vec![("a", intv("1")), ("list", listv(vec![intv("1"), intv("2")]))]), vec![node("n1", strv("hello"))])
    }

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
    #[test]
    fn mutation_diff_law_all_variants() {
        let base = base_fixture();

        apply_and_check(&base, SemioValueMutation::NoMutation);
        apply_and_check(&base, SemioValueMutation::SetSnapshot { snapshot: snap(SemioValue::Bool { value: true }, vec![]) });
        apply_and_check(&base, SemioValueMutation::SetValue { path: vec![SemioValuePathSegment::Key { key: "a".into() }], value: intv("2") });
        apply_and_check(&base, SemioValueMutation::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") });
        apply_and_check(&base, SemioValueMutation::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") });
        apply_and_check(&base, SemioValueMutation::RemoveMapEntry { path: vec![], key: "a".into() });
        apply_and_check(&base, SemioValueMutation::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: intv("99") });
        apply_and_check(&base, SemioValueMutation::RemoveListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 0 });
        apply_and_check(&base, SemioValueMutation::SetNode { id: ValueId::new("n1"), value: strv("updated") });
        apply_and_check(&base, SemioValueMutation::SetNode { id: ValueId::new("n2"), value: strv("brand-new") });
        apply_and_check(&base, SemioValueMutation::RemoveNode { id: ValueId::new("n1") });
    }

    #[test]
    fn set_map_entry_on_missing_key_adds_at_end() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let (result, _) = apply_and_check(&base, SemioValueMutation::SetMapEntry { path: vec![], key: "b".into(), value: intv("2") });
        assert_eq!(result.root, mapv(vec![("a", intv("1")), ("b", intv("2"))]));
    }

    #[test]
    fn remove_map_entry_missing_key_is_noop() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let (result, diff) = apply_and_check(&base, SemioValueMutation::RemoveMapEntry { path: vec![], key: "missing".into() });
        assert_eq!(result, base);
        assert!(diff.diff().is_empty());
    }

    #[test]
    fn nested_path_targets_inner_entry() {
        let base = snap(mapv(vec![("outer", mapv(vec![("inner", intv("1"))]))]), vec![]);
        let (result, _) = apply_and_check(&base, SemioValueMutation::SetMapEntry { path: vec![SemioValuePathSegment::Key { key: "outer".into() }], key: "inner".into(), value: intv("42") });
        assert_eq!(result.root, mapv(vec![("outer", mapv(vec![("inner", intv("42"))]))]));
    }

    #[test]
    fn set_node_on_missing_id_inserts() {
        let base = snap(SemioValue::Null, vec![node("n1", strv("a"))]);
        let (result, _) = apply_and_check(&base, SemioValueMutation::SetNode { id: ValueId::new("n2"), value: strv("b") });
        assert_eq!(result.nodes, vec![node("n1", strv("a")), node("n2", strv("b"))]);
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[test]
    fn inverse_law_mutation_level_round_trips() {
        let base = base_fixture();
        let mutations = vec![
            SemioValueMutation::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") },
            SemioValueMutation::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") },
            SemioValueMutation::RemoveMapEntry { path: vec![], key: "a".into() },
            SemioValueMutation::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: intv("99") },
            SemioValueMutation::RemoveListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 0 },
            SemioValueMutation::SetValue { path: vec![SemioValuePathSegment::Key { key: "a".into() }], value: strv("replaced") },
            SemioValueMutation::SetNode { id: ValueId::new("n1"), value: strv("updated") },
            SemioValueMutation::SetNode { id: ValueId::new("n9"), value: strv("brand-new") },
            SemioValueMutation::RemoveNode { id: ValueId::new("n1") },
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

    #[test]
    fn inverse_law_diff_level_matches_mutation_diff() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mutation = SemioValueMutation::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") };
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
    #[test]
    fn op_text_binary_roundtrip_law() {
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
}
//#endregion 🔖️Tests
