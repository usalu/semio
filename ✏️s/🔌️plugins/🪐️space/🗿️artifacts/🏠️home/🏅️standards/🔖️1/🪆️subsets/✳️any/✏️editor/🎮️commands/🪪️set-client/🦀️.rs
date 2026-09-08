//! 🪪️ S Home launcher app command — `set-client`. The shell's identity bootstrap (contract §C3)
//! dispatches this once identity resolves, so `HomeConfig.client_id`/`client_name` are reachable —
//! without it `HomeConfigMutation::SetClient` would be dead code with no caller.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-client")]
pub struct SetClient {
    pub client_id: String,
    pub client_name: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &SetClient, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit { config_mutations: vec![HomeConfigMutation::SetClient { client_id: payload.client_id.clone(), client_name: payload.client_name.clone() }], ..Default::default() })
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
