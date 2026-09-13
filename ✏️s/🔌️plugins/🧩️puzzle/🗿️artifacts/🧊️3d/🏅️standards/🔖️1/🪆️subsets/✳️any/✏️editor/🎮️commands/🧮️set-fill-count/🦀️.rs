//! 🪣️ `set-fill-count` retained-operation helpers.

use crate::editor::puzzle3d::precompute::{Puzzle3dPrecomputeSession, FILL_LOCK_PLACEMENTS_PER_TICK};
use crate::editor::puzzle3d::{fixture_object_from_snapshot, Puzzle3dAttraction, Puzzle3dFixture};
use crate::standards::v1::subsets::any::schema::mutations::{connect_vortices, create_object, delete_object, Puzzle3dMutation};
use crate::Puzzle3dObject;
use dsl::os_pack::json::Value;
use semio_framework::kernel::Effect;
use serde_json::json;

/// 📨️ Routes text-entry requests through the retained public command.
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), action: "setFillCount".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "value": count }))), delay_ms: 0 }
}

/// 🔢️ The requested count is any `u32` — the planner plans toward exactly what was asked and reports
/// a capacity it cannot reach as a visible stall, never as a silent clamp
/// (`26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` master plan §1 decision 1).
pub(crate) fn parse_count(args: Option<&Value>) -> u32 {
    args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(Value::as_f64).map_or(0, |value| value.round().clamp(0.0, f64::from(u32::MAX)) as u32)
}

/// 🧱️ One locked placement as the editor's own document model sees it — the engine's fixture row goes
/// through the persisted twin exactly like the `create_object` mutation's payload does.
fn document_object(object: &crate::standards::v1::subsets::any::schema::FixtureObject) -> Option<Puzzle3dObject> {
    <Puzzle3dObject as dsl::FromValue>::from_value(dsl::ToValue::to_value(object)).ok()
}

fn document_attraction(attraction: &crate::standards::v1::subsets::any::schema::AttractionProps) -> Puzzle3dAttraction {
    Puzzle3dAttraction {
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
    }
}

/// 🔒️ One bounded document-side delta toward the requested count, as ready-to-publish mutations —
/// the shape the retained `setFillCount` work publishes, which has no scene to diff against.
pub(crate) fn take_locked_mutations(precompute: &mut Puzzle3dPrecomputeSession) -> Vec<Puzzle3dMutation> {
    let Some(chunk) = precompute.take_fill_locked_chunk(FILL_LOCK_PLACEMENTS_PER_TICK) else {
        return Vec::new();
    };
    let mut mutations = Vec::with_capacity(chunk.added_objects.len() + chunk.added_attractions.len() + chunk.removed_object_ids.len());
    mutations.extend(chunk.removed_object_ids.into_iter().map(delete_object));
    mutations.extend(chunk.added_objects.iter().filter_map(document_object).map(|object| create_object(object, None)));
    mutations.extend(chunk.added_attractions.iter().map(|attraction| connect_vortices(attraction.id.clone(), attraction.attracting.clone(), attraction.attracted.clone(), attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y)));
    mutations
}

/// 🔒️ The same bounded delta applied to a reducer's live scene — `fillBuildTick` runs on the ctx path,
/// where the epilogue derives the document operations from the fixture it hands back
/// (`puzzle3d_operations_from_fixture_change`), so a locked placement becomes a real
/// `create_object`/`connect_vortices` and a discarded tail a real `delete_object`. Answers whether the
/// fixture changed.
pub(crate) fn take_locked_into_fixture(precompute: &mut Puzzle3dPrecomputeSession, fixture: &mut Puzzle3dFixture) -> bool {
    let Some(chunk) = precompute.take_fill_locked_chunk(FILL_LOCK_PLACEMENTS_PER_TICK) else {
        return false;
    };
    if chunk.added_objects.is_empty() && chunk.added_attractions.is_empty() && chunk.removed_object_ids.is_empty() {
        return false;
    }
    if !chunk.removed_object_ids.is_empty() {
        let removed: std::collections::HashSet<&str> = chunk.removed_object_ids.iter().map(String::as_str).collect();
        let orphaned = |vortex_full_id: &str| vortex_full_id.split_once(':').is_some_and(|(object_id, _)| removed.contains(object_id));
        fixture.objects.retain(|object| !removed.contains(object.id.as_str()));
        fixture.attractions.retain(|attraction| !orphaned(&attraction.attracting) && !orphaned(&attraction.attracted));
    }
    fixture.objects.extend(chunk.added_objects.iter().filter_map(document_object).map(|object| fixture_object_from_snapshot(&object)));
    fixture.attractions.extend(chunk.added_attractions.iter().map(document_attraction));
    true
}
