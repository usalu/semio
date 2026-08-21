//! 📇️ S Home launcher app command — `fold-directory-events`. The shell's directory lane posts every
//! `/directory/ws` event batch here as a JSON array (contract §C6); each event is folded into
//! `HomeConfig.directory_json` via one `HomeConfigMutation::FoldDirectoryEvent` per event, in order.
//! View action only: never an artifact mutation, config lane, persisted local-only, no undo (matches
//! `HomeConfigMutation::FoldDirectoryEvent`'s own doc — the fold is the SOLE writer of the read model).

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "fold-directory-events")]
pub struct FoldDirectoryEvents {
    /// 📇️ A JSON array of `DirectoryEvent` (contract §C1), as received over `/directory/ws`.
    pub events_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub async fn handle(payload: &FoldDirectoryEvents, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let events: Vec<store::os_directory::DirectoryEvent> = serde_json::from_str(&payload.events_json).unwrap_or_default();
    let config_mutations = events.iter().filter_map(|event| serde_json::to_string(event).ok()).map(|event_json| HomeConfigMutation::FoldDirectoryEvent { event_json }).collect();
    Ok(Emit { config_mutations, ..Default::default() })
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn one_config_mutation_per_event() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let events_json = serde_json::json!([
            {"seq": 1, "id": "e1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
             "body": {"kind": "space.created", "spaceId": "sp-1", "name": "A", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"}, "recordedAtMs": 1},
            {"seq": 2, "id": "e2", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-2",
             "body": {"kind": "space.created", "spaceId": "sp-2", "name": "B", "spaceKind": "studio", "visibility": "public", "ownerUserId": "u2"}, "recordedAtMs": 2}
        ])
        .to_string();
        let emit = handle(&FoldDirectoryEvents { events_json }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_mutations.len(), 2);
        assert!(emit.artifact_mutations.is_empty(), "never an artifact mutation");
        assert!(emit.effects.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_events_json_yields_no_mutations() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&FoldDirectoryEvents { events_json: "not json".into() }, &doc, &cfg).expect("handle");
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
