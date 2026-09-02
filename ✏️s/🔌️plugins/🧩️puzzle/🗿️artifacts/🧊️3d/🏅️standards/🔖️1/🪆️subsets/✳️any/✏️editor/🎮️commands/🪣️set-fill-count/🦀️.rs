//! 🪣️ `set-fill-count` resumable command.

use crate::artifacts::puzzle3d::mutations::{connect_vortices, create_object, delete_object, Puzzle3dMutation};
use crate::artifacts::puzzle3d::Puzzle3dObject;
use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dConfigMutation};
use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::{puzzle3d_fill_build_scope, PUZZLE3D_FILL_COUNT_MAX};
use semio_framework::kernel::Effect;
use semio_framework_plugin::Emit;
use serde_json::{json, Value};

pub const STEP_ACTION_ID: &str = "setFillCountStep";
pub(crate) const MAX_PLACEMENTS_PER_STEP: usize = 1;

fn queue(generation: u64, target: u32) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: STEP_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(json!({ "generation": generation, "target": target }))),
        delay_ms: 0,
    }
}

/// 📨️ Routes non-slider entry points through the same resumable public command.
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), action: "setFillCount".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "value": count }))), delay_ms: 0 }
}

fn parse_count(args: Option<&Value>) -> u32 {
    args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(Value::as_f64).map_or(0, |value| value.round().max(0.0) as u32).min(PUZZLE3D_FILL_COUNT_MAX)
}

/// 🧵️ Commits the reveal cutoff immediately and enqueues materialization without scanning the document.
pub fn begin(precompute: &mut Puzzle3dPrecomputeSession, config: &Puzzle3dConfig, args: Option<&Value>) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
    precompute.restore_persisted_fill(&config.fill_checkpoint);
    precompute.set_fill_applied_count(config.fill_applied_count);
    let target = parse_count(args).min(precompute.fill_available_count());
    let generation = config.fill_apply_generation.saturating_add(1);
    let applied = config.fill_applied_count.min(precompute.fill_available_count());
    let effects = (applied != target).then(|| queue(generation, target)).into_iter().collect();
    Emit { config_mutations: vec![Puzzle3dConfigMutation::SetFillRequest { count: target, generation }], coalesce_key: Some(format!("fill-count:{generation}")), effects, ui_scope: puzzle3d_fill_build_scope(), ..Default::default() }
}

/// 🔁️ Emits at most one direct semantic placement and generation-checks every continuation.
pub fn step(precompute: &mut Puzzle3dPrecomputeSession, config: &Puzzle3dConfig, args: Option<&Value>) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
    let generation = args.and_then(|value| value.get("generation")).and_then(Value::as_u64).unwrap_or(0);
    let target = args.and_then(|value| value.get("target")).and_then(Value::as_u64).unwrap_or(0).min(u32::MAX as u64) as u32;
    if generation != config.fill_apply_generation || target != config.fill_count || !precompute.restore_persisted_fill(&config.fill_checkpoint) {
        return Emit::default();
    }
    precompute.set_fill_applied_count(config.fill_applied_count);
    let Some(chunk) = precompute.apply_fill_count_chunk(target, MAX_PLACEMENTS_PER_STEP) else {
        return Emit::default();
    };
    let mut mutations = Vec::with_capacity(chunk.added_objects.len() + chunk.added_attractions.len() + chunk.removed_object_ids.len());
    mutations.extend(chunk.removed_object_ids.into_iter().map(delete_object));
    mutations.extend(chunk.added_objects.into_iter().filter_map(|object| serde_json::to_value(object).ok().and_then(|value| serde_json::from_value::<Puzzle3dObject>(value).ok())).map(|object| create_object(object, None)));
    mutations.extend(
        chunk
            .added_attractions
            .into_iter()
            .map(|attraction| connect_vortices(attraction.id, attraction.attracting, attraction.attracted, attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y)),
    );
    let effects = (chunk.applied_count != target).then(|| queue(generation, target)).into_iter().collect();
    Emit {
        artifact_mutations: mutations,
        config_mutations: vec![Puzzle3dConfigMutation::SetFillAppliedCount { count: chunk.applied_count }],
        coalesce_key: Some(format!("fill-count:{generation}")),
        effects,
        ui_scope: puzzle3d_fill_build_scope(),
        ..Default::default()
    }
}
