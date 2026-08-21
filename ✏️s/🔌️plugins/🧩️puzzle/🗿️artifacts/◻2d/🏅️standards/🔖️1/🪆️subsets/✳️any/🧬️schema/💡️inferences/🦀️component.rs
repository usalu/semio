//! 💡️ Puzzle2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🎛flat-position/`, the same
//! graph-BFS-derived positioning concept `🧊️3d`'s own `🎛flat-position/` and `🔱️trinity/🔌️jack`'s own
//! `🎛flat-position/` carry for their artifacts — here reusing the existing
//! `⚙️engine/📐️layout::fastened_layout_snapshot` compose-parity math directly rather than
//! duplicating it, a plain whole-snapshot BFS pass, so no `InferredField`/incremental caching is
//! needed, matching both siblings' own "simple whole-snapshot scalars" rationale).

use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use artifact_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::flat_position::{compute_flat_position, Puzzle2dFlatPosition};

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPosition`, backed by the `🎛flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d.inference")]
pub struct Puzzle2dInference {
    #[derived]
    pub flat_position: Puzzle2dFlatPosition,
}

impl protocol::Inference<Puzzle2dSnapshot> for Puzzle2dInference {
    async fn infer(snapshot: &Puzzle2dSnapshot) -> Self {
        Self { flat_position: compute_flat_position(snapshot) }
    }
}

impl protocol::InferenceSpec<Puzzle2dSnapshot> for Puzzle2dInference {
    async fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle2d.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.puzzle.puzzle2d.inference.flatPosition", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: the underlying `fastened_layout_snapshot` BFS re-runs in one pass over the whole
/// graph — the default `infer_cached` passthrough (just calls `infer`) is exactly right here, no
/// `InferredField` chain needed (mirrors jack's own `🎛flat-position`/`🧭topology` rationale).
impl ArtifactInferrer for crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::Puzzle2dBuilder {
    type Snapshot = Puzzle2dSnapshot;
    type Inference = Puzzle2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle2d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle2d_artifact_schema_descriptor`'s registration.
pub async fn puzzle2d_artifact_inference_descriptor() -> artifact_schema::ArtifactInferenceDescriptor {
    artifact_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle2d.inference",
        inference: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn parent_child_snapshot() -> Puzzle2dSnapshot {
        // p (Fixed, off-origin) --e-- c (Derived): edge x/y offsets place c relative to p.
        let p = Puzzle2dNode { id: "p".into(), x: 5.0, y: 7.0, anchor: Puzzle2dNodeAnchor::Fixed, handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }], ..Default::default() };
        let c = Puzzle2dNode { id: "c".into(), anchor: Puzzle2dNodeAnchor::Derived, handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }], ..Default::default() };
        let e = Puzzle2dEdge { id: "e".into(), source: "p:h".into(), target: "c:h".into(), x: 3.0, y: -2.0, ..Default::default() };
        Puzzle2dSnapshot { schema: crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA.to_string(), camera: Default::default(), nodes: vec![p, c], edges: vec![e], meta: Default::default() }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = parent_child_snapshot();
        assert_eq!(Puzzle2dInference::infer(&snapshot), Puzzle2dInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(Puzzle2dInference::infer(&Puzzle2dSnapshot::default()), Puzzle2dInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_matches_compute_flat_position_directly() {
        let snapshot = parent_child_snapshot();
        let inferred = Puzzle2dInference::infer(&snapshot);
        assert_eq!(inferred.flat_position, compute_flat_position(&snapshot));
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests

//#region 🔖️FastenedLayout
use crate::artifacts::puzzle2d::{Puzzle2dNode, Puzzle2dNodeAnchor};
/// 🔗️ Rehomed from the deleted `⚙️engine/📐️layout` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e): compose-parity fastened layout, pure derived
/// compute over a `Puzzle2dSnapshot` — sole consumer is `🎛flat-position`'s own `compute_flat_position`
/// (see that file's own `use super::fastened_layout_snapshot;`), so it lives at the inference family
/// root rather than being duplicated into the slug dir.
use crate::artifacts::puzzle3d::schema::inferences::flatten::{DIAGRAM_HORIZONTAL_SCALE, DIAGRAM_RADIUS};
use std::collections::{HashMap, HashSet, VecDeque};

async fn round_f(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

async fn parse_endpoint(endpoint: &str) -> Option<(&str, &str)> {
    endpoint.split_once(':')
}

/// 🔗 Compose-parity fastened layout: places nodes from edge gap/shift/rise/rotation/turn/tilt + x/y using the diagram-center rule.
pub async fn fastened_layout_snapshot(snapshot: &mut Puzzle2dSnapshot) {
    if snapshot.nodes.is_empty() {
        return;
    }
    let node_map: HashMap<&str, &Puzzle2dNode> = snapshot.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut adjacency: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (index, edge) in snapshot.edges.iter().enumerate() {
        let Some((_source_id, _)) = parse_endpoint(&edge.source).or_else(|| Some((edge.source.as_str(), ""))) else { continue };
        let Some((_target_id, _)) = parse_endpoint(&edge.target).or_else(|| Some((edge.target.as_str(), ""))) else { continue };
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
                let parent_t = handle_id.and_then(|id| current_node.handles.iter().find(|handle| handle.id == id)).map(|handle| handle.angle / (2.0 * std::f64::consts::PI)).unwrap_or(0.0);
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
mod fastened_tests {
    use super::*;
    use crate::artifacts::puzzle2d::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dNodeAnchor, Puzzle2dSnapshot};

    #[semio_framework_async_macros::async_test]
    async fn fastened_layout_places_child_from_origin_parent_by_handle_angle() {
        let mut snapshot = Puzzle2dSnapshot {
            schema: "puzzle.2d".into(),
            camera: Puzzle2dCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                Puzzle2dNode {
                    id: "p".into(),
                    node_kind: None,
                    shape: None,
                    x: 0.0,
                    y: 0.0,
                    radius: None,
                    width: None,
                    height: None,
                    text: None,
                    icon_kind: None,
                    root: None,
                    scale: None,
                    visible: None,
                    locked: None,
                    anchor: Puzzle2dNodeAnchor::Fixed,
                    handles: vec![crate::artifacts::puzzle2d::Puzzle2dHandle { id: "h".into(), handle_kind: None, angle: 0.0, radius: None, color: None, icon_kind: None, scale: None, visible: None, locked: None }],
                },
                Puzzle2dNode {
                    id: "c".into(),
                    node_kind: None,
                    shape: None,
                    x: 0.0,
                    y: 0.0,
                    radius: None,
                    width: None,
                    height: None,
                    text: None,
                    icon_kind: None,
                    root: None,
                    scale: None,
                    visible: None,
                    locked: None,
                    anchor: Puzzle2dNodeAnchor::Derived,
                    handles: vec![crate::artifacts::puzzle2d::Puzzle2dHandle { id: "h".into(), handle_kind: None, angle: 0.0, radius: None, color: None, icon_kind: None, scale: None, visible: None, locked: None }],
                },
            ],
            edges: vec![Puzzle2dEdge {
                id: "e".into(),
                source: "p:h".into(),
                target: "c:h".into(),
                edge_kind: None,
                source_tip: None,
                target_tip: None,
                visible: None,
                locked: None,
                gap: 0.0,
                shift: 0.0,
                rise: 0.0,
                rotation: 0.0,
                turn: 0.0,
                tilt: 0.0,
                x: 0.0,
                y: 0.0,
            }],
            meta: Puzzle2dMeta::default(),
        };
        fastened_layout_snapshot(&mut snapshot);
        let child = snapshot.nodes.iter().find(|node| node.id == "c").expect("c");
        assert_eq!(child.x, 0.0);
        assert_eq!(child.y, DIAGRAM_RADIUS);
    }
}
//#endregion 🔖️FastenedLayout
