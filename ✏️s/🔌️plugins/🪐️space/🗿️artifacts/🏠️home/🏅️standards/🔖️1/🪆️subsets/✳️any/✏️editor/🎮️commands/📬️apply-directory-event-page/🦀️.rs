//! 📄️ Accepts one authenticated, receipt-sealed directory page as one local config replacement.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "apply-directory-event-page")]
pub struct ApplyDirectoryEventPage {
    /// 📄️ Canonical `DirectoryEventPageV1` JSON returned by the authenticated hub.
    pub page_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
/// 📬️ The editor's route into the one shared page emission ([`HomeConfig::directory_event_page_emit`]), which the
/// read-only viewer answers through too.
pub fn handle(payload: &ApplyDirectoryEventPage, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    cfg.snapshot.directory_event_page_emit(&payload.page_json)
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
