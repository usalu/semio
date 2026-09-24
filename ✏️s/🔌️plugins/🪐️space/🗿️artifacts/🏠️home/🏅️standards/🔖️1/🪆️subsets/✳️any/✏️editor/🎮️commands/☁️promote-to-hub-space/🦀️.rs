//! ☁️ S Home launcher app command — `promote-to-hub-space`.
//! Event-sourced promotion of an ephemeral local-only studio onto the hub directory
//! (`os.directory.create-space`). Share/collaboration stay blocked until the hub fold lands.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "promote-to-hub-space")]
pub struct PromoteToHubSpace {
    pub space_id: String,
    pub name: String,
}

pub fn handle(payload: &PromoteToHubSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let name = if payload.name.trim().is_empty() { payload.space_id.clone() } else { payload.name.clone() };
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "name": name, "spaceKind": "atelier", "visibility": "private", "sourceSpaceId": payload.space_id.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.create-space".into(), args }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
