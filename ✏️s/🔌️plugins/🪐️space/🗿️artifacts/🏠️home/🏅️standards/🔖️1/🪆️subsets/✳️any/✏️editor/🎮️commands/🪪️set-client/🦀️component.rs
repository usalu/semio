//! 🪪️ S Home launcher app command — `set-client`. The shell's identity bootstrap (contract §C3)
//! dispatches this once identity resolves, so `HomeConfig.client_id`/`client_name` are reachable —
//! without it `HomeConfigMutation::SetClient` would be dead code with no caller.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-client")]
pub struct SetClient {
    pub client_id: String,
    pub client_name: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub async fn handle(payload: &SetClient, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit { config_mutations: vec![HomeConfigMutation::SetClient { client_id: payload.client_id.clone(), client_name: payload.client_name.clone() }], ..Default::default() })
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn set_client_emits_exactly_one_config_mutation() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&SetClient { client_id: "u1".into(), client_name: "Ada".into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_mutations, vec![HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() }]);
        assert!(emit.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
