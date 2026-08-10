//! 💬️ Note play app commands — the window engagement input (single-block rename).

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::engine::patch_block_field;
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::config(vec![NoteConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub value: Option<String>,
    }

    pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let mut artifact_mutations = Vec::new();
        if config.selected_block_ids.len() == 1 {
            let name = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
            let target_id = config.selected_block_ids[0].clone();
            let next = patch_block_field(doc.snapshot, &target_id, "name", &Value::String(name));
            artifact_mutations.push(NoteMutation::SetBlocks { blocks: next.blocks });
        }
        Ok(Emit { artifact_mutations, config_mutations: vec![NoteConfigMutation::SetEngagementInput { value: String::new() }], ..Default::default() })
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️NavigatorEngagementInput
pub mod navigator_engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "navigator-engagement-input")]
    pub struct NavigatorEngagementInput {}

    pub fn handle(_payload: &NavigatorEngagementInput, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️NavigatorEngagementInput
