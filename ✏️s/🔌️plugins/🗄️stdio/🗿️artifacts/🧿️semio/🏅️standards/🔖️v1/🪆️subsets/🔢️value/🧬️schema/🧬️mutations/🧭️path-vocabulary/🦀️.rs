//! 🧭️ The semio value mutation path vocabulary shared by the leaves and the aggregate: path segments, read-only navigation and the nested modified-chain a leaf diff is lowered into.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueDiff;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;

//#region 🔖️SemioValuePath
/// 🧭️ One step of a [`SemioValuePath`] — a map key or a list position. Struct (named-field)
/// variants throughout, never bare tuple variants — same internally-tagged runtime-serialization
/// hazard `SemioValue`'s own doc comment cites.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn resolve<'a>(root: &'a SemioValue, path: &[SemioValuePathSegment]) -> Option<&'a SemioValue> {
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

//#region 🔖️DiffAtPath
/// 🧩 Lowers a leaf [`SemioValueDiff`] (addressing the node found at `path`) into the nested
/// modified-chain matching the recipe's tree-nesting rule — no path addressing inside diffs
/// themselves, only at the mutation level. Always targets `root`; `nodes` is untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_path(path: &[SemioValuePathSegment], leaf: Option<SemioValueDiff>) -> SemioValueTreeDiff {
    SemioValueTreeDiff { root: leaf.map(|leaf| wrap_at_path(path, leaf)), nodes: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_at_path(path: &[SemioValuePathSegment], leaf: SemioValueDiff) -> SemioValueDiff {
    match path.split_first() {
        None => leaf,
        Some((SemioValuePathSegment::Key { key }, rest)) => SemioValueDiff::Map { diff: NamedTripleDiff { removed: Vec::new(), added: Vec::new(), modified: vec![NamedModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] } },
        Some((SemioValuePathSegment::Index { index }, rest)) => SemioValueDiff::List {
            diff: semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![semio_s_artifact_stdio_semio_base::schema::triples::IndexModified { index: *index, diff: wrap_at_path(rest, leaf) }],
            },
        },
    }
}
//#endregion 🔖️DiffAtPath
