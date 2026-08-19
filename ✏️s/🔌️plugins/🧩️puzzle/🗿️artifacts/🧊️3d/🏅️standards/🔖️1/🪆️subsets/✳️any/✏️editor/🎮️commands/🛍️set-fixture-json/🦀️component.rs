//! 🛍️ `set-fixture-json` command.

use crate::editor::puzzle3d::{resolve_puzzle3d_attractions, Puzzle3dActionCtx, Puzzle3dFixture};
use serde_json::Value;

pub async fn set_fixture_json(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
        if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(json_text) {
            ctx.scene.fixture = fixture;
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
        }
    }
}
