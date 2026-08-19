//! 🌳 `census` — one named inference: a real recursive fold over the value GRAPH's own variant
//! shape — `root` plus every backing `nodes[].value` (each walked as its own little tree; `Ref`
//! itself is a LEAF for this walk — it is never dereferenced into `nodes`, since that would make
//! `max_depth`/the census depend on graph connectivity rather than each stored tree's own literal
//! shape, and a `Ref` may legitimately dangle or cycle). A plain whole-snapshot fold — no
//! `InferredField`/incremental caching needed for one recursive pass (same ruling `flow`'s/
//! `graph`'s own whole-graph topology facets reach for their own graphs).

use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Census
/// 🌳️ Semio value graph variant census.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    async fn default() -> Self {
        Self { null_count: 1, bool_count: 0, int_count: 0, float_count: 0, str_count: 0, bytes_count: 0, list_count: 0, map_count: 0, ref_count: 0, node_count: 0, max_depth: 1 }
    }
}

/// 🌳️ Recursively walks `value`, tallying its own variant into `census` and returning the max
/// depth reached at or below it (`depth` is this node's own 1-based depth).
async fn walk(value: &SemioValue, census: &mut SemioValueCensus, depth: u32) -> u32 {
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
pub async fn compute_semio_value_census(snapshot: &SemioValueSnapshot) -> SemioValueCensus {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValueEntry, SemioValueNode, ValueId, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};

    /// 🌱 A hand-built, non-empty graph: a 3-deep map/list root (Map -> List -> Str, depth 3) plus
    /// one backing node holding a 2-deep value (Map -> Bool, depth 2) — exercises every variant and
    /// a genuine max-depth comparison across root vs. nodes.
    async fn populated() -> SemioValueSnapshot {
        SemioValueSnapshot {
            schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
            root: SemioValue::Map {
                entries: vec![
                    SemioValueEntry { key: "tags".into(), value: SemioValue::List { items: vec![SemioValue::Str { value: "a".into() }, SemioValue::Int { lexeme: "1".into() }] } },
                    SemioValueEntry { key: "linked".into(), value: SemioValue::Ref { id: ValueId::new("n1") } },
                ],
            },
            nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Map { entries: vec![SemioValueEntry { key: "flag".into(), value: SemioValue::Bool { value: true } }] } }],
        }
    }

    #[test]
    async fn tallies_every_variant_and_finds_the_true_max_depth() {
        let census = compute_semio_value_census(&populated());
        assert_eq!(census.map_count, 2, "root map + node's own map");
        assert_eq!(census.list_count, 1);
        assert_eq!(census.str_count, 1);
        assert_eq!(census.int_count, 1);
        assert_eq!(census.ref_count, 1);
        assert_eq!(census.bool_count, 1);
        assert_eq!(census.null_count, 0);
        assert_eq!(census.node_count, 1);
        assert_eq!(census.max_depth, 3, "root: Map(1) -> List(2) -> Str(3)");
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = populated();
        assert_eq!(compute_semio_value_census(&snapshot), compute_semio_value_census(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_value_census(&SemioValueSnapshot::default()), SemioValueCensus::default());
    }
}
//#endregion 🧪️Tests
