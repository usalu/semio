//! 🛍️ `set-active-example` resumable command.

use crate::artifacts::puzzle2d::mutations::{change_manifest_id, connect_handles, connect_kind_compatibility, create_node, delete_node, disconnect_handles, disconnect_kind_compatibility, replace_kind_catalogs};
use crate::artifacts::puzzle2d::op::{Puzzle2dMutation, Puzzle2dPlaySnapshot};
use crate::artifacts::puzzle2d::{Puzzle2dKindCompatibility, Puzzle2dSnapshot};
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dPlayRuntime};
use crate::editor::puzzle2d::{PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::Emit;
use serde_json::{Value, json};
use std::sync::LazyLock;

pub const STEP_ACTION_ID: &str = "setActiveExampleStep";

const STAGE_CLEAR_EDGES: &str = "clearEdges";
const STAGE_CLEAR_NODES: &str = "clearNodes";
const STAGE_MANIFEST: &str = "manifest";
const STAGE_CLEAR_COMPATIBILITY: &str = "clearCompatibility";
const STAGE_ADD_COMPATIBILITY: &str = "addCompatibility";
const STAGE_CATALOGS: &str = "catalogs";
const STAGE_NODES: &str = "nodes";
const STAGE_EDGES: &str = "edges";
pub(crate) const MAX_MUTATIONS_PER_STEP: usize = 16;

static EMPTY: LazyLock<Puzzle2dSnapshot> = LazyLock::new(Puzzle2dSnapshot::default);
static CONCRETE_FOREST: LazyLock<Puzzle2dSnapshot> = LazyLock::new(|| serde_json::from_str(crate::examples::puzzle2d::concrete_forest::SOURCE.document_json()).expect("concrete forest example json must match Puzzle2dSnapshot"));
static NAKAGIN: LazyLock<Puzzle2dSnapshot> = LazyLock::new(|| serde_json::from_str(crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE.document_json()).expect("nakagin example json must match Puzzle2dSnapshot"));

pub fn warm_examples() {
    LazyLock::force(&EMPTY);
    LazyLock::force(&CONCRETE_FOREST);
    LazyLock::force(&NAKAGIN);
}

fn canonical_example_id(example_id: &str) -> &'static str {
    match example_id {
        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID | "concrete" => PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID | "nakagin" => PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID,
        _ => "",
    }
}

fn target(example_id: &str) -> &'static Puzzle2dSnapshot {
    match example_id {
        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID => &CONCRETE_FOREST,
        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID => &NAKAGIN,
        _ => &EMPTY,
    }
}

fn queue(generation: u64, stage: &str, index: usize) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: STEP_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(json!({ "generation": generation, "stage": stage, "index": index }))),
        delay_ms: 0,
    }
}

fn step_emit(generation: u64, mutations: Vec<Puzzle2dMutation>, next: Option<(&str, usize)>, config_mutation: Option<Puzzle2dConfigMutation>) -> Emit<Puzzle2dMutation, Puzzle2dConfigMutation> {
    Emit {
        artifact_mutations: mutations,
        config_mutations: config_mutation.into_iter().collect(),
        coalesce_key: Some(format!("setActiveExample:{generation}")),
        effects: next.into_iter().map(|(stage, index)| queue(generation, stage, index)).collect(),
        ui_scope: UiDirtyScope::Full,
        ..Default::default()
    }
}

/// 🧵️ Starts a generation-tagged continuation; the interaction step only resets session config and enqueues work.
pub fn begin_active_example(config: &Puzzle2dConfig, args: Option<&Value>) -> Emit<Puzzle2dMutation, Puzzle2dConfigMutation> {
    let example_id = canonical_example_id(args.and_then(|value| value.get("exampleId")).and_then(Value::as_str).unwrap_or(""));
    let generation = config.example_load_generation.saturating_add(1);
    let mut next = Puzzle2dPlayRuntime::default();
    next.example_load_generation = generation;
    next.example_load_id = Some(example_id.to_string());
    Emit { config_mutations: vec![Puzzle2dConfigMutation::Snapshot { config: next }], effects: vec![queue(generation, STAGE_CLEAR_EDGES, 0)], ui_scope: UiDirtyScope::Full, ..Default::default() }
}

/// 🔁️ Advances one fixed-size mutation chunk, using document+config as the resumable checkpoint.
pub fn step_active_example(doc: &Puzzle2dPlaySnapshot, config: &Puzzle2dConfig, args: Option<&Value>) -> Emit<Puzzle2dMutation, Puzzle2dConfigMutation> {
    let generation = args.and_then(|value| value.get("generation")).and_then(Value::as_u64).unwrap_or(0);
    let stage = args.and_then(|value| value.get("stage")).and_then(Value::as_str).unwrap_or(STAGE_CLEAR_EDGES);
    let index = args.and_then(|value| value.get("index")).and_then(Value::as_u64).unwrap_or(0) as usize;
    if generation != config.example_load_generation {
        return Emit::default();
    }
    let Some(example_id) = config.example_load_id.as_deref() else { return Emit::default() };
    let target = target(example_id);
    match stage {
        STAGE_CLEAR_EDGES => {
            let edges = doc.0.get("edges").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
            let mutations = edges.iter().take(MAX_MUTATIONS_PER_STEP).filter_map(|edge| edge.get("id").and_then(Value::as_str)).map(|id| disconnect_handles(id.to_string())).collect();
            let next = if edges.len() > MAX_MUTATIONS_PER_STEP { (STAGE_CLEAR_EDGES, 0) } else { (STAGE_CLEAR_NODES, 0) };
            step_emit(generation, mutations, Some(next), None)
        }
        STAGE_CLEAR_NODES => {
            let nodes = doc.0.get("nodes").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
            let mutations = nodes.iter().take(MAX_MUTATIONS_PER_STEP).filter_map(|node| node.get("id").and_then(Value::as_str)).map(|id| delete_node(id.to_string())).collect();
            let next = if nodes.len() > MAX_MUTATIONS_PER_STEP { (STAGE_CLEAR_NODES, 0) } else { (STAGE_MANIFEST, 0) };
            step_emit(generation, mutations, Some(next), None)
        }
        STAGE_MANIFEST => {
            let current = doc.0.get("meta").and_then(|meta| meta.get("manifestId")).and_then(Value::as_str);
            let mutation = (current != target.meta.manifest_id.as_deref()).then(|| change_manifest_id(target.meta.manifest_id.clone()));
            step_emit(generation, mutation.into_iter().collect(), Some((STAGE_CLEAR_COMPATIBILITY, 0)), None)
        }
        STAGE_CLEAR_COMPATIBILITY => {
            let rows = doc.0.get("meta").and_then(|meta| meta.get("kindCompatibility")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
            let mutations = rows.iter().take(MAX_MUTATIONS_PER_STEP).filter_map(|row| serde_json::from_value::<Puzzle2dKindCompatibility>(row.clone()).ok()).map(|row| disconnect_kind_compatibility(row.source, row.target)).collect();
            let next = if rows.len() > MAX_MUTATIONS_PER_STEP { (STAGE_CLEAR_COMPATIBILITY, 0) } else { (STAGE_ADD_COMPATIBILITY, 0) };
            step_emit(generation, mutations, Some(next), None)
        }
        STAGE_ADD_COMPATIBILITY => {
            let end = index.saturating_add(MAX_MUTATIONS_PER_STEP).min(target.meta.kind_compatibility.len());
            let mutations = target.meta.kind_compatibility[index.min(end)..end].iter().map(|row| connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)).collect();
            let next = if end < target.meta.kind_compatibility.len() { (STAGE_ADD_COMPATIBILITY, end) } else { (STAGE_CATALOGS, 0) };
            step_emit(generation, mutations, Some(next), None)
        }
        STAGE_CATALOGS => {
            let current = doc.0.get("meta").and_then(|meta| meta.get("kindCatalogs"));
            let target_json = target.meta.kind_catalogs.as_ref().and_then(|catalogs| serde_json::to_value(catalogs).ok());
            let mutation = (current != target_json.as_ref()).then(|| replace_kind_catalogs(target.meta.kind_catalogs.clone()));
            step_emit(generation, mutation.into_iter().collect(), Some((STAGE_NODES, 0)), None)
        }
        STAGE_NODES => {
            let end = index.saturating_add(MAX_MUTATIONS_PER_STEP).min(target.nodes.len());
            let mutations = target.nodes[index.min(end)..end].iter().cloned().map(|node| create_node(node, None)).collect();
            let next = if end < target.nodes.len() { (STAGE_NODES, end) } else { (STAGE_EDGES, 0) };
            step_emit(generation, mutations, Some(next), None)
        }
        STAGE_EDGES => {
            let end = index.saturating_add(MAX_MUTATIONS_PER_STEP).min(target.edges.len());
            let mutations = target.edges[index.min(end)..end]
                .iter()
                .map(|edge| {
                    connect_handles(
                        edge.id.clone(),
                        edge.source.clone(),
                        edge.target.clone(),
                        edge.edge_kind.clone(),
                        edge.gap,
                        edge.shift,
                        edge.rise,
                        edge.rotation,
                        edge.turn,
                        edge.tilt,
                        edge.x,
                        edge.y,
                        edge.source_tip.clone(),
                        edge.target_tip.clone(),
                    )
                })
                .collect();
            if end < target.edges.len() {
                step_emit(generation, mutations, Some((STAGE_EDGES, end)), None)
            } else {
                let mut next = config.clone();
                next.example_load_id = None;
                step_emit(generation, mutations, None, Some(Puzzle2dConfigMutation::Snapshot { config: next }))
            }
        }
        _ => step_emit(generation, Vec::new(), Some((STAGE_CLEAR_EDGES, 0)), None),
    }
}
