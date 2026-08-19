//! 💬️ 💬️ Note play app commands command — `engagement-input`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub async fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::config(vec![NoteConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
}
