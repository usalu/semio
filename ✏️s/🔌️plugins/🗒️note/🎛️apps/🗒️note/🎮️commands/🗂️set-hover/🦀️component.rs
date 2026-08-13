//! 🗂️ 🗂️ Note play app commands command — `set-hover`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{block_id, flatten_blocks};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-hover")]
pub struct SetHover {
    pub block_id: Option<String>,
}

pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::config(vec![NoteConfigMutation::SetHoveredBlock { block_id: payload.block_id.clone() }]))
}
