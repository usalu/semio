//! 💡️ Puzzle2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🎛️flat-position/`, the same
//! graph-BFS-derived positioning concept `🧊️3d`'s own `🎛flat-position/` and `🔱️trinity/🔌️jack`'s own
//! `🎛flat-position/` carry for their artifacts — here reusing the existing
//! `⚙️engine/📐️layout::fastened_layout_snapshot` compose-parity math directly rather than
//! duplicating it, a plain whole-snapshot BFS pass, so no `InferredField`/incremental caching is
//! needed, matching both siblings' own "simple whole-snapshot scalars" rationale).

use crate::Puzzle2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::flat_position::{compute_flat_position, Puzzle2dFlatPosition};

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPosition`, backed by the `🎛️flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d.inference")]
pub struct Puzzle2dInference {
    #[derived]
    pub flat_position: Puzzle2dFlatPosition,
}

impl protocol::Inference<Puzzle2dSnapshot> for Puzzle2dInference {
    fn infer(snapshot: &Puzzle2dSnapshot) -> Self {
        Self { flat_position: compute_flat_position(snapshot) }
    }
}

impl protocol::InferenceSpec<Puzzle2dSnapshot> for Puzzle2dInference {
    fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.puzzle.puzzle2d.inference.flatPosition", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: the underlying `fastened_layout_snapshot` BFS re-runs in one pass over the whole
/// graph — the default `infer_cached` passthrough (just calls `infer`) is exactly right here, no
/// `InferredField` chain needed (mirrors jack's own `🎛flat-position`/`🧭topology` rationale).
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Puzzle2dBuilder {
    type Snapshot = Puzzle2dSnapshot;
    type Inference = Puzzle2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle2d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle2d_artifact_schema_descriptor`'s registration.
pub fn puzzle2d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle2d.inference",
        inference: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️FastenedLayout
use crate::{Puzzle2dNode, Puzzle2dNodeAnchor};
/// 🔗️ Rehomed from the deleted `⚙️engine/📐️layout` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e): compose-parity fastened layout, pure derived
/// compute over a `Puzzle2dSnapshot` — sole consumer is `🎛️flat-position`'s own `compute_flat_position`
/// (see that file's own `use super::fastened_layout_snapshot;`), so it lives at the inference family
/// root rather than being duplicated into the slug dir.
use semio_s_artifact_puzzle_3d::{DIAGRAM_HORIZONTAL_SCALE, DIAGRAM_RADIUS};
use std::collections::{HashMap, HashSet, VecDeque};

fn round_f(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

fn parse_endpoint(endpoint: &str) -> Option<(&str, &str)> {
    endpoint.split_once(':')
}

/// 🔗 Compose-parity fastened layout: places nodes from edge gap/shift/rise/rotation/turn/tilt + x/y using the diagram-center rule.
pub fn fastened_layout_snapshot(snapshot: &mut Puzzle2dSnapshot) {
    if snapshot.nodes.is_empty() {
        return;
    }
    let node_map: HashMap<&str, &Puzzle2dNode> = snapshot.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut adjacency: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (index, edge) in snapshot.edges.iter().enumerate() {
        let Some((_source_id, _)) = parse_endpoint(&edge.source).or(Some((edge.source.as_str(), ""))) else { continue };
        let Some((_target_id, _)) = parse_endpoint(&edge.target).or(Some((edge.target.as_str(), ""))) else { continue };
        // Edges may be bare node ids or node:handle.
        let source_id = edge.source.split(':').next().unwrap_or(edge.source.as_str());
        let target_id = edge.target.split(':').next().unwrap_or(edge.target.as_str());
        if node_map.contains_key(source_id) && node_map.contains_key(target_id) {
            adjacency.entry(source_id.to_string()).or_default().push((target_id.to_string(), index));
            adjacency.entry(target_id.to_string()).or_default().push((source_id.to_string(), index));
        }
    }
    let mut centers: HashMap<String, [f64; 2]> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    for node in &snapshot.nodes {
        if visited.contains(&node.id) {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(node.id.clone());
        visited.insert(node.id.clone());
        centers.insert(node.id.clone(), [node.x, node.y]);
        while let Some(current_id) = queue.pop_front() {
            let parent_center = *centers.get(&current_id).unwrap_or(&[0.0, 0.0]);
            let neighbors = adjacency.get(&current_id).cloned().unwrap_or_default();
            for (neighbor_id, edge_index) in neighbors {
                if visited.contains(&neighbor_id) {
                    continue;
                }
                visited.insert(neighbor_id.clone());
                let edge = &snapshot.edges[edge_index];
                let current_node = node_map.get(current_id.as_str()).expect("current");
                // Parent handle angle → t.
                let handle_id = if edge.source.starts_with(&format!("{current_id}:")) {
                    edge.source.split(':').nth(1)
                } else if edge.target.starts_with(&format!("{current_id}:")) {
                    edge.target.split(':').nth(1)
                } else {
                    None
                };
                let parent_t = handle_id.and_then(|id| current_node.handles.iter().find(|handle| handle.id == id)).map_or(0.0, |handle| handle.angle / (2.0 * std::f64::consts::PI));
                // 2d has no parent direction z; treat as horizontal unless encoded otherwise → use horizontal scale branch when parent not at origin.
                let (child_x, child_y) = if parent_center[0] == 0.0 && parent_center[1] == 0.0 {
                    let angle = 2.0 * std::f64::consts::PI * parent_t;
                    (DIAGRAM_RADIUS * angle.sin(), DIAGRAM_RADIUS * angle.cos())
                } else {
                    (parent_center[0] + edge.x * DIAGRAM_HORIZONTAL_SCALE, parent_center[1] + edge.y * DIAGRAM_HORIZONTAL_SCALE)
                };
                centers.insert(neighbor_id.clone(), [round_f(child_x), round_f(child_y)]);
                queue.push_back(neighbor_id);
            }
        }
    }
    for node in &mut snapshot.nodes {
        if let Some(center) = centers.get(&node.id) {
            if !matches!(node.anchor, Puzzle2dNodeAnchor::Fixed) || adjacency.contains_key(&node.id) {
                // Fixed roots keep stored coords; derived/children take computed centers.
            }
            if !matches!(node.anchor, Puzzle2dNodeAnchor::Fixed) {
                node.x = center[0];
                node.y = center[1];
            } else if centers.get(&node.id).is_some() {
                // Keep fixed root; still update non-roots only.
            }
        }
    }
    // Apply computed centers to non-fixed nodes only; fixed keep authored coords.
    for node in &mut snapshot.nodes {
        if matches!(node.anchor, Puzzle2dNodeAnchor::Fixed) {
            continue;
        }
        if let Some(center) = centers.get(&node.id) {
            node.x = center[0];
            node.y = center[1];
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️fastened/🦀️.rs"]
mod fastened_tests;
//#endregion 🔖️FastenedLayout
