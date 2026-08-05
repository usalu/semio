//! 🛍️ Puzzle 3d play app commands — loading a document: the raw `setFixtureJson` escape hatch and the
//! example picker (concrete-forest / nakagin / empty). Both replace the whole fixture and re-resolve
//! every attraction so the loaded poses are internally consistent.

use crate::apps::puzzle3d::{default_fixture, drive_precompute, empty_fixture, nakagin_fixture, resolve_puzzle3d_attractions, Puzzle3dActionCtx, Puzzle3dFixture, PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN};
use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use serde_json::Value;

pub fn set_fixture_json(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
        if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(json_text) {
            ctx.scene.fixture = fixture;
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
        }
    }
}

pub fn set_active_example(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
    let next = if example_id.is_empty() {
        Some(empty_fixture())
    } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
        Some(default_fixture())
    } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
        Some(nakagin_fixture())
    } else {
        None
    };
    if let Some(fixture) = next {
        ctx.scene.fixture = fixture;
        ctx.scene.runtime = Puzzle3dRuntime::default();
    }
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}
