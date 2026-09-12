//! 💬️ 💬️ Note play app commands command — `engagement-submit`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

pub fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let mut artifact_mutations = Vec::new();
    if ctx.selected_block_ids.len() == 1 {
        let name = payload.value.clone().unwrap_or_else(|| ctx.window_transient.engagement_input.clone());
        let target_id = ctx.selected_block_ids[0].clone();
        artifact_mutations.push(crate::schema::mutations::rename_block(target_id, name));
    }
    ctx.window_transient.engagement_input.clear();
    Ok(Emit { artifact_mutations, ..Default::default() })
}
