//! 🌱️ SpaceIndexEditor command — `create-artifact`. The isolated guest relays only the
//! user's catalog-choice token and name. The host re-resolves the kind, mints the identity, and owns
//! the durable creation saga before any open effect.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "create-artifact")]
pub struct CreateArtifact {
    pub name: String,
    pub kind_choice: String,
}

/// 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: mirrors Home's
/// `createSpace` handler (`🏠️home/…/🎮️commands/🌱create-space/🦀️.rs`) — a raw toolbar-button
/// click (`#s-space-create-artifact`, contract §C0) dispatches with no args at all, and this must open
/// the already-declared `createArtifact` dialog instead of failing on an unknown empty `kind_id`.
pub fn handle(payload: &CreateArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    if payload.name.trim().is_empty() || payload.kind_choice.trim().is_empty() {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(130), dialog_id: "createArtifact".into(), args: None }));
    }
    Ok(Emit::effect(Effect::ReplayShellCommand {
        action_id: "os.create-space-artifact".into(),
        args: Some(pack::json_to_dsl_value(&pack::json!({ "kindChoice": payload.kind_choice, "name": payload.name.trim() }))),
    }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
