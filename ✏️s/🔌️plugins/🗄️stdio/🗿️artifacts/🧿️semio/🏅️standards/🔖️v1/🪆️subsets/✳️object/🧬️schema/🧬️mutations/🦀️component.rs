//! 🧬️ SemioObjectMutation — document mutation dispatch. Addresses a target node inside `root` via
//! a [`SemioObjectPath`] (mirrors the recipe's tree-nesting rule: `NodePath` stays mutation-level,
//! each mutation's `diff()` lowers it to a nested modified-chain via [`diff_at_path`] — template
//! copied from `json`'s own `JsonMutation`/`JsonPath`, this subset's informing source). The
//! `objects` GRAPH gets its own flat, path-free id-addressed vocabulary (`SetObject`/
//! `RemoveObject`) since it's a top-level sibling collection to `root`, not a node reachable by
//! tree descent. Every variant's `diff()` and `inverse()` is handcrafted directly against the
//! sparse [`SemioObjectDiff`] shape — never apply-and-capture.

use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets, IndexAdded, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::{
    dec_object_id, dec_semio_object_node_bin, dec_semio_value, dec_semio_value_bin, dec_str, enc_object_id,
    enc_semio_object_node_bin, enc_semio_value, enc_semio_value_bin, enc_str, value_diff_between, write_str_lp, read_str_lp,
    NamedAdded, SemioObjectDiff, SemioValueDiff,
};
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::diff_set_snapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{dec_semio_object_snapshot, enc_semio_object_snapshot, ObjectId, SemioObjectEntry, SemioObjectNode, SemioObjectSnapshot, SemioValue};
use protocol::{Mutation, MutationDiff, OpText};
#[cfg(test)]
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};

//#region 🔖️SemioObjectPath
/// 🧭️ One step of a [`SemioObjectPath`] — a map key or a list position. Struct (named-field)
/// variants throughout, never bare tuple variants — same internally-tagged runtime-serialization
/// hazard `SemioValue`'s own doc comment cites.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SemioObjectPathSegment {
    Key { key: String },
    Index { index: usize },
}

/// 🧭️ Addresses a node inside a `SemioValue` tree rooted at `root`, root-to-leaf. Never crosses a
/// `Ref` boundary — dereferencing a `Ref` is a query-time concern for consumers, not something a
/// path silently flattens.
pub type SemioObjectPath = Vec<SemioObjectPathSegment>;

/// 🔎️ Read-only navigation of `path` from `root`, `None` on the first unresolvable segment
/// (missing key, out-of-range index, or a segment applied to the wrong node kind).
fn resolve<'a>(root: &'a SemioValue, path: &[SemioObjectPathSegment]) -> Option<&'a SemioValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (SemioObjectPathSegment::Key { key }, SemioValue::Map { entries }) => &entries.iter().find(|e| &e.key == key)?.value,
            (SemioObjectPathSegment::Index { index }, SemioValue::List { items }) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}
//#endregion 🔖️SemioObjectPath

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.semio.object`. `SetValue`/`SetMapEntry`/`RemoveMapEntry`/
/// `InsertListItem`/`RemoveListItem` address `root`'s own value tree via [`SemioObjectPath`];
/// `SetObject`/`RemoveObject` address the top-level id-keyed `objects` GRAPH directly (flat, no
/// path — it is not reachable by descending `root`).
/// 🧪️ `#[derive(dsl::DslOps)]` is unusable here for the same structural reason as `SemioObjectDiff`
/// (see that file's doc comment): every variant carries a `SemioValue` and/or `SemioObjectPath`
/// directly, both data-carrying-enum-shaped payloads with no `DslField` impl. `OpText`/`OpBinary`
/// hand-rolled below (§OpCodecs), grammar template copied from `JsonMutation`'s, reusing
/// `SemioObjectDiff`'s `pub(crate)` grammar primitives.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioObjectMutation {
    NoMutation,
    SetSnapshot { snapshot: SemioObjectSnapshot },
    /// 🔁️ Replaces the whole node found at `path` (root, if empty) with `value`, regardless of
    /// its previous kind.
    SetValue { path: SemioObjectPath, value: SemioValue },
    /// ➕️ Sets (creating or overwriting) entry `key` on the map at `path` to `value`.
    SetMapEntry { path: SemioObjectPath, key: String, value: SemioValue },
    /// ➖️ Removes entry `key` from the map at `path`, if present.
    RemoveMapEntry { path: SemioObjectPath, key: String },
    /// ➕️ Inserts `value` into the list at `path` at `index` (ascending-insert-clamped, per the
    /// normative apply contract).
    InsertListItem { path: SemioObjectPath, index: usize, value: SemioValue },
    /// ➖️ Removes the element at `index` from the list at `path`, if present.
    RemoveListItem { path: SemioObjectPath, index: usize },
    /// ➕️ Sets (creating or overwriting) the graph node `id` to `value`.
    SetObject { id: ObjectId, value: SemioValue },
    /// ➖️ Removes graph node `id`, if present.
    RemoveObject { id: ObjectId },
}

impl Default for SemioObjectMutation {
    fn default() -> Self {
        SemioObjectMutation::NoMutation
    }
}
//#endregion 🔖️Mutations

//#region 🔖️DiffAtPath
/// 🧩 Lowers a leaf [`SemioValueDiff`] (addressing the node found at `path`) into the nested
/// modified-chain matching the recipe's tree-nesting rule — no path addressing inside diffs
/// themselves, only at the mutation level. Always targets `root`; `objects` is untouched.
fn diff_at_path(path: &[SemioObjectPathSegment], leaf: Option<SemioValueDiff>) -> SemioObjectDiff {
    SemioObjectDiff { root: leaf.map(|leaf| wrap_at_path(path, leaf)), objects: None }
}

fn wrap_at_path(path: &[SemioObjectPathSegment], leaf: SemioValueDiff) -> SemioValueDiff {
    match path.split_first() {
        None => leaf,
        Some((SemioObjectPathSegment::Key { key }, rest)) => SemioValueDiff::Map {
            diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] },
        },
        Some((SemioObjectPathSegment::Index { index }, rest)) => SemioValueDiff::List {
            diff: crate::artifacts::semio::standards::v1::engine::triples::IndexedTripleDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![crate::artifacts::semio::standards::v1::engine::triples::IndexModified { index: *index, diff: wrap_at_path(rest, leaf) }],
            },
        },
    }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. The diff is the single semantics source: it's computed
/// once from the pre-mutation state, applied to produce the new state, and returned.
pub fn apply_semio_object_mutation(snapshot: &mut SemioObjectSnapshot, mutation: &SemioObjectMutation) -> SemioObjectDiff {
    let diff = <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioObjectDiff as MutationDiff<SemioObjectSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SemioObjectSnapshot> for SemioObjectMutation {
    type Diff = SemioObjectDiff;

    fn diff(&self, base: &SemioObjectSnapshot) -> Self::Diff {
        match self {
            SemioObjectMutation::NoMutation => SemioObjectDiff::default(),
            SemioObjectMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            SemioObjectMutation::SetValue { path, value } => match resolve(&base.root, path) {
                Some(old) if old != value => diff_at_path(path, Some(SemioValueDiff::Replace { value: value.clone() })),
                _ => SemioObjectDiff::default(),
            },

            SemioObjectMutation::SetMapEntry { path, key, value } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
                    Some(existing) => {
                        let leaf = value_diff_between(&existing.value, value);
                        diff_at_path(path, leaf.map(|diff| SemioValueDiff::Map {
                            diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff }] },
                        }))
                    }
                    None => diff_at_path(path, Some(SemioValueDiff::Map {
                        diff: NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: entries.len(), item: SemioObjectEntry { key: key.clone(), value: value.clone() } }] },
                    })),
                },
                _ => SemioObjectDiff::default(),
            },

            SemioObjectMutation::RemoveMapEntry { path, key } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) if entries.iter().any(|e| &e.key == key) => diff_at_path(path, Some(SemioValueDiff::Map {
                    diff: NamedTripleDiff { removed: vec![key.clone()], modified: Vec::new(), added: Vec::new() },
                })),
                _ => SemioObjectDiff::default(),
            },

            SemioObjectMutation::InsertListItem { path, index, value } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) => diff_at_path(path, Some(SemioValueDiff::List {
                    diff: crate::artifacts::semio::standards::v1::engine::triples::IndexedTripleDiff {
                        removed: Vec::new(),
                        modified: Vec::new(),
                        added: vec![IndexAdded { index: (*index).min(items.len()), item: value.clone() }],
                    },
                })),
                _ => SemioObjectDiff::default(),
            },

            SemioObjectMutation::RemoveListItem { path, index } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) if *index < items.len() => diff_at_path(path, Some(SemioValueDiff::List {
                    diff: crate::artifacts::semio::standards::v1::engine::triples::IndexedTripleDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() },
                })),
                _ => SemioObjectDiff::default(),
            },

            SemioObjectMutation::SetObject { id, value } => match base.objects.iter().find(|n| &n.id == id) {
                Some(existing) => match value_diff_between(&existing.value, value) {
                    Some(diff) => SemioObjectDiff { root: None, objects: Some(NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: id.clone(), diff }] }) },
                    None => SemioObjectDiff::default(),
                },
                None => SemioObjectDiff {
                    root: None,
                    objects: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.objects.len(), item: SemioObjectNode { id: id.clone(), value: value.clone() } }] }),
                },
            },

            SemioObjectMutation::RemoveObject { id } => {
                if base.objects.iter().any(|n| &n.id == id) {
                    SemioObjectDiff { root: None, objects: Some(NamedTripleDiff { removed: vec![id.clone()], modified: Vec::new(), added: Vec::new() }) }
                } else {
                    SemioObjectDiff::default()
                }
            }
        }
    }

    /// ↩️ Handcrafted mutation-level inverse, key/index/id-aware — reads the pre-mutation `base`
    /// state to recover the exact undo.
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<Self> {
        match self {
            SemioObjectMutation::NoMutation => vec![SemioObjectMutation::NoMutation],
            SemioObjectMutation::SetSnapshot { .. } => vec![SemioObjectMutation::SetSnapshot { snapshot: base.clone() }],

            SemioObjectMutation::SetValue { path, .. } => match resolve(&base.root, path) {
                Some(old) => vec![SemioObjectMutation::SetValue { path: path.clone(), value: old.clone() }],
                None => vec![SemioObjectMutation::NoMutation],
            },

            SemioObjectMutation::SetMapEntry { path, key, .. } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) => match entries.iter().find(|e| &e.key == key) {
                    Some(existing) => vec![SemioObjectMutation::SetMapEntry { path: path.clone(), key: key.clone(), value: existing.value.clone() }],
                    None => vec![SemioObjectMutation::RemoveMapEntry { path: path.clone(), key: key.clone() }],
                },
                _ => vec![SemioObjectMutation::NoMutation],
            },

            // ↩️ `SetMapEntry` on an absent key always APPENDS (see `diff()` above), so naively
            // reinverting to a single `SetMapEntry` would restore the VALUE but lose the ORIGINAL
            // POSITION whenever other entries follow it — restore exact position by first removing
            // every entry that originally followed `key`, then re-adding `key` and each of them
            // back in original order (every re-add is an append, landing them exactly where they
            // started). Same shape `json`'s `RemoveMember` inverse documents.
            SemioObjectMutation::RemoveMapEntry { path, key } => match resolve(&base.root, path) {
                Some(SemioValue::Map { entries }) => match entries.iter().position(|e| &e.key == key) {
                    Some(pos) => {
                        let tail: Vec<SemioObjectEntry> = entries[pos + 1..].to_vec();
                        let mut steps: Vec<SemioObjectMutation> = tail.iter().rev()
                            .map(|e| SemioObjectMutation::RemoveMapEntry { path: path.clone(), key: e.key.clone() })
                            .collect();
                        steps.push(SemioObjectMutation::SetMapEntry { path: path.clone(), key: key.clone(), value: entries[pos].value.clone() });
                        steps.extend(tail.into_iter().map(|e| SemioObjectMutation::SetMapEntry { path: path.clone(), key: e.key, value: e.value }));
                        steps
                    }
                    None => vec![SemioObjectMutation::NoMutation],
                },
                _ => vec![SemioObjectMutation::NoMutation],
            },

            SemioObjectMutation::InsertListItem { path, index, .. } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) => vec![SemioObjectMutation::RemoveListItem { path: path.clone(), index: (*index).min(items.len()) }],
                _ => vec![SemioObjectMutation::NoMutation],
            },

            SemioObjectMutation::RemoveListItem { path, index } => match resolve(&base.root, path) {
                Some(SemioValue::List { items }) => match items.get(*index) {
                    Some(item) => vec![SemioObjectMutation::InsertListItem { path: path.clone(), index: *index, value: item.clone() }],
                    None => vec![SemioObjectMutation::NoMutation],
                },
                _ => vec![SemioObjectMutation::NoMutation],
            },

            SemioObjectMutation::SetObject { id, .. } => match base.objects.iter().find(|n| &n.id == id) {
                Some(existing) => vec![SemioObjectMutation::SetObject { id: id.clone(), value: existing.value.clone() }],
                None => vec![SemioObjectMutation::RemoveObject { id: id.clone() }],
            },

            SemioObjectMutation::RemoveObject { id } => match base.objects.iter().find(|n| &n.id == id) {
                Some(existing) => vec![SemioObjectMutation::SetObject { id: id.clone(), value: existing.value.clone() }],
                None => vec![SemioObjectMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `SemioObjectMutation` (`#[derive(dsl::DslOps)]` blocked,
/// see the enum doc comment above) — reuses `SemioObjectDiff`'s `pub(crate)` grammar primitives
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant — same shape `JsonMutation`'s hand-rolled codec
/// uses.
fn enc_path_segment(seg: &SemioObjectPathSegment) -> String {
    match seg {
        SemioObjectPathSegment::Key { key } => format!("K[{}]", enc_str(key)),
        SemioObjectPathSegment::Index { index } => format!("I[{index}]"),
    }
}
fn dec_path_segment(s: &str) -> Result<SemioObjectPathSegment, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "K" => Ok(SemioObjectPathSegment::Key { key: dec_str(strip_brackets(rest)?)? }),
        "I" => Ok(SemioObjectPathSegment::Index { index: strip_brackets(rest)?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        other => Err(format!("semio object path segment: unknown tag {other:?}")),
    }
}
fn enc_path(p: &SemioObjectPath) -> String {
    format!("[{}]", p.iter().map(enc_path_segment).collect::<Vec<_>>().join(","))
}
fn dec_path(s: &str) -> Result<SemioObjectPath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}
/// 🧭️ `enc_semio_snapshot`/`dec_semio_snapshot` — thin aliases for the single-source-of-truth
/// `SemioObjectSnapshot` text codec now owned by the sibling `📸️snapshot/🦀️component.rs` (also
/// reused there by `ArtifactDsl`/`ArtifactPack`), rather than a second independent copy.
fn enc_semio_snapshot(s: &SemioObjectSnapshot) -> String {
    enc_semio_object_snapshot(s)
}
fn dec_semio_snapshot(s: &str) -> Result<SemioObjectSnapshot, String> {
    dec_semio_object_snapshot(s)
}

fn print_object_mutation(m: &SemioObjectMutation) -> String {
    match m {
        SemioObjectMutation::NoMutation => "no-mutation".to_string(),
        SemioObjectMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_semio_snapshot(snapshot)),
        SemioObjectMutation::SetValue { path, value } => format!("set-value path={} value={}", enc_path(path), enc_semio_value(value)),
        SemioObjectMutation::SetMapEntry { path, key, value } => {
            format!("set-map-entry path={} key={} value={}", enc_path(path), enc_str(key), enc_semio_value(value))
        }
        SemioObjectMutation::RemoveMapEntry { path, key } => format!("remove-map-entry path={} key={}", enc_path(path), enc_str(key)),
        SemioObjectMutation::InsertListItem { path, index, value } => {
            format!("insert-list-item path={} index={index} value={}", enc_path(path), enc_semio_value(value))
        }
        SemioObjectMutation::RemoveListItem { path, index } => format!("remove-list-item path={} index={index}", enc_path(path)),
        SemioObjectMutation::SetObject { id, value } => format!("set-object id={} value={}", enc_object_id(id), enc_semio_value(value)),
        SemioObjectMutation::RemoveObject { id } => format!("remove-object id={}", enc_object_id(id)),
    }
}
fn parse_object_mutation(line: &str) -> Result<SemioObjectMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioObjectMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("semio object mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio object mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SemioObjectMutation::SetSnapshot { snapshot: dec_semio_snapshot(arg("snapshot")?)? }),
        "set-value" => Ok(SemioObjectMutation::SetValue { path: dec_path(arg("path")?)?, value: dec_semio_value(arg("value")?)? }),
        "set-map-entry" => Ok(SemioObjectMutation::SetMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)?, value: dec_semio_value(arg("value")?)? }),
        "remove-map-entry" => Ok(SemioObjectMutation::RemoveMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)? }),
        "insert-list-item" => Ok(SemioObjectMutation::InsertListItem {
            path: dec_path(arg("path")?)?,
            index: usize_arg("index")?,
            value: dec_semio_value(arg("value")?)?,
        }),
        "remove-list-item" => Ok(SemioObjectMutation::RemoveListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")? }),
        "set-object" => Ok(SemioObjectMutation::SetObject { id: dec_object_id(arg("id")?)?, value: dec_semio_value(arg("value")?)? }),
        "remove-object" => Ok(SemioObjectMutation::RemoveObject { id: dec_object_id(arg("id")?)? }),
        other => Err(format!("semio object mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioObjectMutation {
    fn print_op(&self) -> String {
        print_object_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_object_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryPrimitives
/// 🧭️ Real recursive binary twin of [`enc_path`]/[`dec_path`] — a varint segment COUNT, then per
/// segment a 1-byte kind tag (`0`=Key/`1`=Index) and its own real payload. Template copied from
/// json's own `enc_json_path_bin`/`dec_json_path_bin`.
fn enc_semio_path_bin(path: &[SemioObjectPathSegment], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, path.len() as u64);
    for segment in path {
        match segment {
            SemioObjectPathSegment::Key { key } => {
                out.push(0);
                write_str_lp(out, key);
            }
            SemioObjectPathSegment::Index { index } => {
                out.push(1);
                store::pack_rt::write_varint_u64(out, *index as u64);
            }
        }
    }
}
fn dec_semio_path_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioObjectPath, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let tag = reader.read_u8().map_err(|e| e.to_string())?;
        match tag {
            0 => path.push(SemioObjectPathSegment::Key { key: read_str_lp(reader)? }),
            1 => path.push(SemioObjectPathSegment::Index { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
            other => return Err(format!("semio object path binary: unknown segment tag {other}")),
        }
    }
    Ok(path)
}

/// 🧭️ Real recursive binary twin of [`enc_semio_snapshot`]/[`dec_semio_snapshot`] — used ONLY by
/// `SetSnapshot`'s own `OpBinary` payload (the sibling `📸️snapshot/🦀️component.rs`'s own
/// `ArtifactPack` stays text-native, matching `json`'s exact precedent — see that file's doc
/// comment).
fn enc_semio_object_snapshot_bin(s: &SemioObjectSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    enc_semio_value_bin(&s.root, out);
    store::pack_rt::write_varint_u64(out, s.objects.len() as u64);
    for node in &s.objects {
        enc_semio_object_node_bin(node, out);
    }
}
fn dec_semio_object_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioObjectSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let root = dec_semio_value_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut objects = Vec::with_capacity(count as usize);
    for _ in 0..count {
        objects.push(dec_semio_object_node_bin(reader)?);
    }
    Ok(SemioObjectSnapshot { schema, root, objects })
}
//#endregion 🔖️OpBinaryPrimitives

/// 🧪️ Real binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the `print_op().into_bytes()` text-as-binary shortcut this facet started with.
/// `tag` is the `SemioObjectMutation` variant ordinal, in the same 0-8 order
/// `print_object_mutation`'s own keyword match uses. Every variant's own path/key/value/id payload
/// is real LEB128-varint-framed binary (never text-as-bytes) — same treatment json's own
/// `JsonMutation::encode_op`/`decode_op` uses.
impl protocol::OpBinary for SemioObjectMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            SemioObjectMutation::NoMutation => 0,
            SemioObjectMutation::SetSnapshot { .. } => 1,
            SemioObjectMutation::SetValue { .. } => 2,
            SemioObjectMutation::SetMapEntry { .. } => 3,
            SemioObjectMutation::RemoveMapEntry { .. } => 4,
            SemioObjectMutation::InsertListItem { .. } => 5,
            SemioObjectMutation::RemoveListItem { .. } => 6,
            SemioObjectMutation::SetObject { .. } => 7,
            SemioObjectMutation::RemoveObject { .. } => 8,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            SemioObjectMutation::NoMutation => {}
            SemioObjectMutation::SetSnapshot { snapshot } => enc_semio_object_snapshot_bin(snapshot, &mut out),
            SemioObjectMutation::SetValue { path, value } => {
                enc_semio_path_bin(path, &mut out);
                enc_semio_value_bin(value, &mut out);
            }
            SemioObjectMutation::SetMapEntry { path, key, value } => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
                enc_semio_value_bin(value, &mut out);
            }
            SemioObjectMutation::RemoveMapEntry { path, key } => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
            }
            SemioObjectMutation::InsertListItem { path, index, value } => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_semio_value_bin(value, &mut out);
            }
            SemioObjectMutation::RemoveListItem { path, index } => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            SemioObjectMutation::SetObject { id, value } => {
                write_str_lp(&mut out, &id.value);
                enc_semio_value_bin(value, &mut out);
            }
            SemioObjectMutation::RemoveObject { id } => {
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
            0 => Ok(SemioObjectMutation::NoMutation),
            1 => {
                let snapshot = dec_semio_object_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(SemioObjectMutation::SetSnapshot { snapshot })
            }
            2 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioObjectMutation::SetValue { path, value })
            }
            3 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioObjectMutation::SetMapEntry { path, key, value })
            }
            4 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                Ok(SemioObjectMutation::RemoveMapEntry { path, key })
            }
            5 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioObjectMutation::InsertListItem { path, index, value })
            }
            6 => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(SemioObjectMutation::RemoveListItem { path, index })
            }
            7 => {
                let id = ObjectId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioObjectMutation::SetObject { id, value })
            }
            8 => {
                let id = ObjectId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                Ok(SemioObjectMutation::RemoveObject { id })
            }
            other => Err(malformed("op tag", 1, format!("unknown op tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ Representative `SemioObjectMutation` values, one per variant, incl. nested/list/map payload
/// values, a `Ref`/`Bytes` payload, and a multi-segment `SemioObjectPath` mixing both segment
/// kinds — the single source of truth reused by `op_text_binary_roundtrip_law` below AND by
/// `🎹️composer/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, same convention json's own `demo_mutation_cases` uses.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioObjectMutation> {
    fn snap(root: SemioValue, objects: Vec<SemioObjectNode>) -> SemioObjectSnapshot {
        SemioObjectSnapshot { schema: crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(), root, objects }
    }
    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioObjectEntry { key: k.into(), value: v }).collect() }
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
    fn node(id: &str, value: SemioValue) -> SemioObjectNode {
        SemioObjectNode { id: ObjectId::new(id), value }
    }

    let mixed_path = vec![SemioObjectPathSegment::Key { key: "outer".into() }, SemioObjectPathSegment::Index { index: 2 }, SemioObjectPathSegment::Key { key: "inner".into() }];
    vec![
        SemioObjectMutation::NoMutation,
        SemioObjectMutation::SetSnapshot {
            snapshot: snap(
                mapv(vec![("a", intv("1")), ("b", listv(vec![strv("x"), SemioValue::Null, SemioValue::Bool { value: true }]))]),
                vec![node("n1", SemioValue::Bytes { value: vec![1, 2, 3] })],
            ),
        },
        SemioObjectMutation::SetValue { path: vec![], value: SemioValue::Ref { id: ObjectId::new("n1") } },
        SemioObjectMutation::SetMapEntry { path: vec![], key: "a".into(), value: SemioValue::Float { lexeme: "2.5e10".into() } },
        SemioObjectMutation::SetMapEntry { path: mixed_path.clone(), key: "k".into(), value: mapv(vec![("nested", strv("v"))]) },
        SemioObjectMutation::RemoveMapEntry { path: vec![SemioObjectPathSegment::Key { key: "outer".into() }], key: "gone".into() },
        SemioObjectMutation::InsertListItem { path: vec![SemioObjectPathSegment::Key { key: "list".into() }], index: 1, value: listv(vec![intv("1"), intv("2")]) },
        SemioObjectMutation::RemoveListItem { path: vec![SemioObjectPathSegment::Index { index: 0 }], index: 3 },
        SemioObjectMutation::SetValue { path: mixed_path, value: SemioValue::Null },
        SemioObjectMutation::SetObject { id: ObjectId::new("n1"), value: SemioValue::Bytes { value: vec![255, 0, 128] } },
        SemioObjectMutation::RemoveObject { id: ObjectId::new("n1") },
    ]
}
//#endregion 🔖️DemoCases

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA;

    fn snap(root: SemioValue, objects: Vec<SemioObjectNode>) -> SemioObjectSnapshot {
        SemioObjectSnapshot { schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(), root, objects }
    }

    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioObjectEntry { key: k.into(), value: v }).collect() }
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

    fn node(id: &str, value: SemioValue) -> SemioObjectNode {
        SemioObjectNode { id: ObjectId::new(id), value }
    }

    fn base_fixture() -> SemioObjectSnapshot {
        snap(
            mapv(vec![("a", intv("1")), ("list", listv(vec![intv("1"), intv("2")]))]),
            vec![node("n1", strv("hello"))],
        )
    }

    fn apply_and_check(base: &SemioObjectSnapshot, mutation: SemioObjectMutation) -> (SemioObjectSnapshot, SemioObjectDiff) {
        let mut via_apply = base.clone();
        let returned = apply_semio_object_mutation(&mut via_apply, &mutation);
        let expected_diff = mutation.diff(base);
        assert_eq!(returned, expected_diff, "apply_semio_object_mutation must return mutation.diff(base)");
        let via_diff_apply = expected_diff.apply(base);
        assert_eq!(via_apply, via_diff_apply, "m.diff(base).apply(base) must equal apply_semio_object_mutation's result");
        (via_apply, returned)
    }

    //#region mutation_diff_law
    #[test]
    fn mutation_diff_law_all_variants() {
        let base = base_fixture();

        apply_and_check(&base, SemioObjectMutation::NoMutation);
        apply_and_check(&base, SemioObjectMutation::SetSnapshot { snapshot: snap(SemioValue::Bool { value: true }, vec![]) });
        apply_and_check(&base, SemioObjectMutation::SetValue { path: vec![SemioObjectPathSegment::Key { key: "a".into() }], value: intv("2") });
        apply_and_check(&base, SemioObjectMutation::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") });
        apply_and_check(&base, SemioObjectMutation::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") });
        apply_and_check(&base, SemioObjectMutation::RemoveMapEntry { path: vec![], key: "a".into() });
        apply_and_check(&base, SemioObjectMutation::InsertListItem { path: vec![SemioObjectPathSegment::Key { key: "list".into() }], index: 1, value: intv("99") });
        apply_and_check(&base, SemioObjectMutation::RemoveListItem { path: vec![SemioObjectPathSegment::Key { key: "list".into() }], index: 0 });
        apply_and_check(&base, SemioObjectMutation::SetObject { id: ObjectId::new("n1"), value: strv("updated") });
        apply_and_check(&base, SemioObjectMutation::SetObject { id: ObjectId::new("n2"), value: strv("brand-new") });
        apply_and_check(&base, SemioObjectMutation::RemoveObject { id: ObjectId::new("n1") });
    }

    #[test]
    fn set_map_entry_on_missing_key_adds_at_end() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let (result, _) = apply_and_check(&base, SemioObjectMutation::SetMapEntry { path: vec![], key: "b".into(), value: intv("2") });
        assert_eq!(result.root, mapv(vec![("a", intv("1")), ("b", intv("2"))]));
    }

    #[test]
    fn remove_map_entry_missing_key_is_noop() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let (result, diff) = apply_and_check(&base, SemioObjectMutation::RemoveMapEntry { path: vec![], key: "missing".into() });
        assert_eq!(result, base);
        assert!(diff.is_empty());
    }

    #[test]
    fn nested_path_targets_inner_entry() {
        let base = snap(mapv(vec![("outer", mapv(vec![("inner", intv("1"))]))]), vec![]);
        let (result, _) = apply_and_check(&base, SemioObjectMutation::SetMapEntry {
            path: vec![SemioObjectPathSegment::Key { key: "outer".into() }],
            key: "inner".into(),
            value: intv("42"),
        });
        assert_eq!(result.root, mapv(vec![("outer", mapv(vec![("inner", intv("42"))]))]));
    }

    #[test]
    fn set_object_on_missing_id_inserts() {
        let base = snap(SemioValue::Null, vec![node("n1", strv("a"))]);
        let (result, _) = apply_and_check(&base, SemioObjectMutation::SetObject { id: ObjectId::new("n2"), value: strv("b") });
        assert_eq!(result.objects, vec![node("n1", strv("a")), node("n2", strv("b"))]);
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[test]
    fn inverse_law_mutation_level_round_trips() {
        let base = base_fixture();
        let mutations = vec![
            SemioObjectMutation::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") },
            SemioObjectMutation::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") },
            SemioObjectMutation::RemoveMapEntry { path: vec![], key: "a".into() },
            SemioObjectMutation::InsertListItem { path: vec![SemioObjectPathSegment::Key { key: "list".into() }], index: 1, value: intv("99") },
            SemioObjectMutation::RemoveListItem { path: vec![SemioObjectPathSegment::Key { key: "list".into() }], index: 0 },
            SemioObjectMutation::SetValue { path: vec![SemioObjectPathSegment::Key { key: "a".into() }], value: strv("replaced") },
            SemioObjectMutation::SetObject { id: ObjectId::new("n1"), value: strv("updated") },
            SemioObjectMutation::SetObject { id: ObjectId::new("n9"), value: strv("brand-new") },
            SemioObjectMutation::RemoveObject { id: ObjectId::new("n1") },
        ];
        for mutation in mutations {
            let mut state = base.clone();
            apply_semio_object_mutation(&mut state, &mutation);
            for undo in <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::inverse(&mutation, &base) {
                apply_semio_object_mutation(&mut state, &undo);
            }
            assert_eq!(state, base, "mutation {mutation:?} did not round-trip via its inverse");
        }
    }

    #[test]
    fn inverse_law_diff_level_matches_mutation_diff() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mutation = SemioObjectMutation::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") };
        let diff = mutation.diff(&base);
        let mid = diff.apply(&base);
        let inv = diff.inverse(&base);
        assert_eq!(inv.apply(&mid), base);
    }
    //#endregion inverse_law

    //#region 🔖️OpCodecTests
    /// 🧪️ op_text_binary_roundtrip_law: exercises every variant, incl. nested/list/map payload
    /// values, a `Ref`/`Bytes` payload, and a multi-segment `SemioObjectPath` mixing both segment
    /// kinds.
    #[test]
    fn op_text_binary_roundtrip_law() {
        use protocol::{OpBinary, OpText};

        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioObjectMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <SemioObjectMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch");
        }
    }
    //#endregion 🔖️OpCodecTests
}
//#endregion 🔖️Tests
