//! 🪣️ `set-fill-count` retained-operation helpers.

use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::PUZZLE3D_FILL_COUNT_MAX;
use crate::standards::v1::subsets::any::schema::mutations::{connect_vortices, create_object, delete_object, Puzzle3dMutation};
use crate::Puzzle3dObject;
use dsl::os_pack::json::Value;
use semio_framework::kernel::Effect;
use serde_json::json;

pub(crate) const MAX_PLACEMENTS_PER_STEP: usize = 1;

/// 📨️ Routes text-entry requests through the retained public command.
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), action: "setFillCount".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "value": count }))), delay_ms: 0 }
}

pub(crate) fn parse_count(args: Option<&Value>) -> u32 {
    args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(Value::as_f64).map_or(0, |value| value.round().max(0.0) as u32).min(PUZZLE3D_FILL_COUNT_MAX)
}

pub(crate) fn apply_chunk(precompute: &mut Puzzle3dPrecomputeSession, target: u32) -> Option<(u32, Vec<Puzzle3dMutation>)> {
    let chunk = precompute.apply_fill_count_chunk(target, MAX_PLACEMENTS_PER_STEP)?;
    let mut mutations = Vec::with_capacity(chunk.added_objects.len() + chunk.added_attractions.len() + chunk.removed_object_ids.len());
    mutations.extend(chunk.removed_object_ids.into_iter().map(delete_object));
    mutations.extend(chunk.added_objects.into_iter().filter_map(|object| <Puzzle3dObject as dsl::FromValue>::from_value(dsl::ToValue::to_value(&object)).ok()).map(|object| create_object(object, None)));
    mutations.extend(
        chunk
            .added_attractions
            .into_iter()
            .map(|attraction| connect_vortices(attraction.id, attraction.attracting, attraction.attracted, attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y)),
    );
    Some((chunk.applied_count, mutations))
}
