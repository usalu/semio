//! 🐚️ 🐚️ Note play app commands command — `load-request`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "load-request")]
pub struct LoadRequest {}

pub fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".dsl,.note.dsl,.spk,.ops,application/octet-stream,text/plain".into(), read_as: None, import_action: "setFixtureJson".into(), multiple: false }))
}
