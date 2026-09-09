//! 🖌️ `add-brush-object` command.

use crate::standards::v1::subsets::any::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::fixture_from_engine_fixture;
use crate::editor::puzzle3d::puzzle3d_rederive_all_attractions;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT;
use dsl::os_pack::json::Value;

/// 🧱️ Places an explicit `BrushPlacePayload` (the viewport's own click-to-place path) and re-selects
/// what it placed through `Emit.interaction_writes` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn add_brush_object(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let Some(payload) = args.and_then(|value| <BrushPlacePayload as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value)).ok()) else {
        return;
    };
    let before: Vec<String> = ctx.scene.fixture.objects.iter().map(|object| object.id.clone()).collect();
    let outcome = ctx.app.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
    // 🧯️ The engine refuses a placement that collides or exceeds the overlap budget
    // (`Puzzle3dError::BrushPlacementRejected`) and a fixture the app model cannot adopt is equally a
    // non-placement — both used to fall out of an `if let Ok(Fixture(_))` with nothing on screen at all.
    let placed_scene = match outcome {
        Ok(Puzzle3dEngineOutcome::Fixture(fixture)) => fixture_from_engine_fixture(ctx.scene, &fixture),
        _ => None,
    };
    match placed_scene {
        Some(next) => {
            *ctx.scene = next;
            puzzle3d_rederive_all_attractions(&mut ctx.scene.fixture);
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
            let placed: Vec<String> = ctx.scene.fixture.objects.iter().map(|object| object.id.clone()).filter(|id| !before.contains(id)).collect();
            ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, placed);
        }
        None => ctx.notice(|labels| labels.placement_rejected.as_str()),
    }
}
