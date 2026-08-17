//! 🖌️ `add-brush-object` command.

use crate::artifacts::puzzle3d::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::fixture_from_engine_fixture;
use crate::editor::puzzle3d::puzzle3d_rederive_all_attractions;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;

/// 🧱️ Places an explicit `BrushPlacePayload` (the viewport's own click-to-place path).
pub fn add_brush_object(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let Some(payload) = args.and_then(|value| serde_json::from_value::<BrushPlacePayload>(value.clone()).ok()) else {
        return;
    };
    let outcome = ctx.app.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
    if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = outcome {
        if let Some(next) = fixture_from_engine_fixture(ctx.scene, &fixture) {
            *ctx.scene = next;
            puzzle3d_rederive_all_attractions(&mut ctx.scene.fixture);
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
        }
    }
}
