//! 👥️ S Home launcher app command — `presence-heartbeat`. Deliberate scope cut, documented (not
//! silent): contract §C0's presence scope is `(space_id, document_id, surface)` and Home is neither
//! inside a space nor bound to a hub document, so there is nothing here for a heartbeat to attach to
//! yet — this is a genuine, dispatchable no-op stub kept structurally ready for the day Home's own
//! table surface gets a presence roster (owned by `👥️PresenceBar`, lane 2-F/3-A territory), not a
//! placeholder that silently drops the action id.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "presence-heartbeat")]
pub struct PresenceHeartbeat {}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub async fn handle(_payload: &PresenceHeartbeat, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit::default())
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn heartbeat_is_dispatchable_and_emits_nothing() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&PresenceHeartbeat {}, &doc, &cfg).expect("handle");
        assert!(emit.artifact_mutations.is_empty());
        assert!(emit.config_mutations.is_empty());
        assert!(emit.effects.is_empty());
    }
}
//#endregion 🧪️Tests
