//! 💬️ 💬️ Note play app commands command — `engagement-submit`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

pub async fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut artifact_mutations = Vec::new();
    if ctx.selected_block_ids.len() == 1 {
        let name = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
        let target_id = ctx.selected_block_ids[0].clone();
        artifact_mutations.push(crate::artifacts::note::schema::mutations::rename_block(target_id, name));
    }
    Ok(Emit { artifact_mutations, config_mutations: vec![NoteConfigMutation::SetEngagementInput { value: String::new() }], ..Default::default() })
}
