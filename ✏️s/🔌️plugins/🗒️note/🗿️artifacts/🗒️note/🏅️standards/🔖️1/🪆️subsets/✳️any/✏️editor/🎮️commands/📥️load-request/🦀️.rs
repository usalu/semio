//! 🐚️ 🐚️ Note play app commands command — `load-request`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "load-request")]
pub struct LoadRequest {}

pub fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(119), accept: ".dsl,.note.dsl,.spk,.ops,application/octet-stream,text/plain".into(), read_as: None, import_action: "setFixtureJson".into(), multiple: false }))
}
