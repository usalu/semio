//! 📄️ Accepts one authenticated, receipt-sealed directory page as one local config replacement.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{DirectoryProjectionReceiptV1, HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{AppEvent, ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "apply-directory-event-page")]
pub struct ApplyDirectoryEventPage {
    /// 📄️ Canonical `DirectoryEventPageV1` JSON returned by the authenticated hub.
    pub page_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &ApplyDirectoryEventPage, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let page = store::os_directory::DirectoryEventPageV1::parse_canonical_json(&payload.page_json).map_err(|_| Fault::from("s.home.directory-event-page-invalid"))?;
    let next = cfg.snapshot.apply_directory_event_page(&page)?;
    let receipt = next.directory_projection_receipt().ok_or_else(|| Fault::from("s.home.directory-projection-receipt-invalid"))?;
    let event = AppEvent { kind: DirectoryProjectionReceiptV1::SCHEMA.into(), payload: protocol::ToValue::to_value(&receipt) };
    if next == *cfg.snapshot {
        return Ok(Emit { events: vec![event], ..Default::default() });
    }
    Ok(Emit {
        config_mutations: vec![HomeConfigMutation::ReplaceDirectoryProjection {
            directory_json: next.directory_json,
            session_binding_sha256: next.directory_session_binding_sha256,
            authorization_generation: next.directory_authorization_generation,
            receipt_sha256: next.directory_receipt_sha256,
        }],
        events: vec![event],
        ..Default::default()
    })
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
