//! 💬️ Note play app commands — the window engagement input (single-block rename).

use crate::apps::note::config::{NoteConfig, NoteConfigOperation};
use crate::artifacts::note::engine::patch_block_field;
use crate::artifacts::note::op::NoteOperation;
use crate::artifacts::note::NoteDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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

    pub fn handle(payload: &EngagementInput, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        Ok(Emit::config(vec![NoteConfigOperation::SetEngagementInput { value: payload.value.clone() }]))
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

    pub fn handle(payload: &EngagementSubmit, doc: &DocumentView<'_, NoteDocument>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        let config = cfg.projection;
        let mut document_operations = Vec::new();
        if config.selected_block_ids.len() == 1 {
            let name = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
            let target_id = config.selected_block_ids[0].clone();
            let next = patch_block_field(doc.projection, &target_id, "name", &Value::String(name));
            document_operations.push(NoteOperation::SetBlocks { blocks: next.blocks });
        }
        Ok(Emit { document_operations, config_operations: vec![NoteConfigOperation::SetEngagementInput { value: String::new() }], ..Default::default() })
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️NavigatorEngagementInput
pub mod navigator_engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "navigator-engagement-input")]
    pub struct NavigatorEngagementInput {}

    pub fn handle(_payload: &NavigatorEngagementInput, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️NavigatorEngagementInput
