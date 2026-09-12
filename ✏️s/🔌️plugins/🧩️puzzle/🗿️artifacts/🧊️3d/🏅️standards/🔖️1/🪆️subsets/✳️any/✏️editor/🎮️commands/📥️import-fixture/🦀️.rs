//! 📥 Replaces the document with imported fixture JSON.

use crate::editor::puzzle3d::{Puzzle3dActionCtx, Puzzle3dFixture, PUZZLE3D_FIXTURE_SCHEMA};
use dsl::json;
use dsl::os_pack::json::{parse, Value};
use dsl::FromValue;

/// 📥 Replaces the live fixture with the supplied JSON as one document edit.
pub fn import_fixture(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(args) = args else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        ctx.abort = true;
        return;
    };
    let value = args
        .get("payload")
        .and_then(Value::as_str)
        .and_then(|text| parse(text).ok())
        .or_else(|| args.get("json").cloned().filter(|value| value.as_object().is_some()))
        .or_else(|| args.get("fixture").cloned().filter(|value| value.as_object().is_some()))
        .or_else(|| args.get("payload").cloned().filter(|value| value.as_object().is_some()));
    let Some(value) = value else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        ctx.abort = true;
        return;
    };
    let Ok(mut fixture) = Puzzle3dFixture::from_value(json::to_dsl_value(&value)) else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        ctx.abort = true;
        return;
    };
    if fixture.schema.is_empty() {
        fixture.schema = PUZZLE3D_FIXTURE_SCHEMA.into();
    }
    ctx.scene.fixture = fixture;
}
