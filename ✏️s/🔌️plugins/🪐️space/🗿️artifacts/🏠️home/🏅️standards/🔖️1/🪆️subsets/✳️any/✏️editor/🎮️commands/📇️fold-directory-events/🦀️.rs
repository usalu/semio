//! 📇️ S Home launcher app command — `fold-directory-events`. The shell's directory lane posts every
//! `/directory/ws` event batch here as a JSON array (contract §C6); each event is folded into
//! `HomeConfig.directory_json` via one `HomeConfigMutation::FoldDirectoryEvent` per event, in order.
//! View action only: never an artifact mutation, config lane, persisted local-only, no undo (matches
//! `HomeConfigMutation::FoldDirectoryEvent`'s own doc — the fold is the SOLE writer of the read model).

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "fold-directory-events")]
pub struct FoldDirectoryEvents {
    /// 📇️ A JSON array of `DirectoryEvent` (contract §C1), as received over `/directory/ws`.
    pub events_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &FoldDirectoryEvents, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let events: Vec<store::os_directory::DirectoryEvent> = pack::from_json_str(&payload.events_json).unwrap_or_default();
    let config_mutations = events.iter().map(pack::to_json_string).map(|event_json| HomeConfigMutation::FoldDirectoryEvent { event_json }).collect();
    Ok(Emit { config_mutations, ..Default::default() })
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
