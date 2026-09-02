//! 🐚️ 🐚️ Note play app commands command — `load-request`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "load-request")]
pub struct LoadRequest {}

pub async fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(119), accept: ".dsl,.note.dsl,.spk,.ops,application/octet-stream,text/plain".into(), read_as: None, import_action: "setFixtureJson".into(), multiple: false }))
}
