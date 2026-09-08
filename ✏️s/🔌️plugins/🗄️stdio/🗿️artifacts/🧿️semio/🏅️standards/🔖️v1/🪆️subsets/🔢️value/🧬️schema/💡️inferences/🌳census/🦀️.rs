//! 🌳 `census` — one named inference: a real recursive fold over the value GRAPH's own variant
//! shape — `root` plus every backing `nodes[].value` (each walked as its own little tree; `Ref`
//! itself is a LEAF for this walk — it is never dereferenced into `nodes`, since that would make
//! `max_depth`/the census depend on graph connectivity rather than each stored tree's own literal
//! shape, and a `Ref` may legitimately dangle or cycle). A plain whole-snapshot fold — no
//! `InferredField`/incremental caching needed for one recursive pass (same ruling `flow`'s/
//! `graph`'s own whole-graph topology facets reach for their own graphs).

use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot};

//#region 🔖️Census
/// 🌳️ Semio value graph variant census.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioValueCensus {
    pub null_count: u32,
    pub bool_count: u32,
    pub int_count: u32,
    pub float_count: u32,
    pub str_count: u32,
    pub bytes_count: u32,
    pub list_count: u32,
    pub map_count: u32,
    pub ref_count: u32,
    pub node_count: u32,
    pub max_depth: u32,
}

/// 🩹 Hand-rolled, NOT derived — `root` is never absent (`SemioValueSnapshot::default().root ==
/// SemioValue::Null`), so an empty graph still contains ONE real value node at depth 1. Matches
/// `compute_semio_value_census(&SemioValueSnapshot::default())` exactly (proven by
/// `inference_default_law` below) — the same non-empty-default correction `flow`'s own
/// `SemioFlowTopology::default()` documents for its own zero case.
impl Default for SemioValueCensus {
    fn default() -> Self {
        Self { null_count: 1, bool_count: 0, int_count: 0, float_count: 0, str_count: 0, bytes_count: 0, list_count: 0, map_count: 0, ref_count: 0, node_count: 0, max_depth: 1 }
    }
}

/// 🌳️ Recursively walks `value`, tallying its own variant into `census` and returning the max
/// depth reached at or below it (`depth` is this node's own 1-based depth).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk(value: &SemioValue, census: &mut SemioValueCensus, depth: u32) -> u32 {
    match value {
        SemioValue::Null => {
            census.null_count += 1;
            depth
        }
        SemioValue::Bool { .. } => {
            census.bool_count += 1;
            depth
        }
        SemioValue::Int { .. } => {
            census.int_count += 1;
            depth
        }
        SemioValue::Float { .. } => {
            census.float_count += 1;
            depth
        }
        SemioValue::Str { .. } => {
            census.str_count += 1;
            depth
        }
        SemioValue::Bytes { .. } => {
            census.bytes_count += 1;
            depth
        }
        SemioValue::List { items } => {
            census.list_count += 1;
            items.iter().fold(depth, |acc, item| acc.max(walk(item, census, depth + 1)))
        }
        SemioValue::Map { entries } => {
            census.map_count += 1;
            entries.iter().fold(depth, |acc, entry| acc.max(walk(&entry.value, census, depth + 1)))
        }
        SemioValue::Ref { .. } => {
            census.ref_count += 1;
            depth
        }
    }
}

/// 🌳️ Computes [`SemioValueCensus`] — pure, total, O(root's tree size + every node's own tree
/// size). `root` and every `nodes[].value` are each walked as an independent tree rooted at
/// depth 1 — see module doc comment for why `Ref` is never dereferenced.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_value_census(snapshot: &SemioValueSnapshot) -> SemioValueCensus {
    let mut census = SemioValueCensus { null_count: 0, bool_count: 0, int_count: 0, float_count: 0, str_count: 0, bytes_count: 0, list_count: 0, map_count: 0, ref_count: 0, node_count: 0, max_depth: 0 };
    let mut max_depth = walk(&snapshot.root, &mut census, 1);
    for node in &snapshot.nodes {
        max_depth = max_depth.max(walk(&node.value, &mut census, 1));
    }
    census.node_count = snapshot.nodes.len() as u32;
    census.max_depth = max_depth;
    census
}
//#endregion 🔖️Census

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
