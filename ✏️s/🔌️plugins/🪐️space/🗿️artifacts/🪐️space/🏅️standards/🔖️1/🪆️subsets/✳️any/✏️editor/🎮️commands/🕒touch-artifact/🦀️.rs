//! 🕒️ SpaceIndexEditor commands command — `touch-artifact`. Dispatched by the shell's own
//! post-checkpoint hook (contract §C5 "After every checkpoint the shell dispatches `TouchArtifact` to
//! the space index") — a real user-visible action too, so it is declared through the same typed
//! command channel every other row uses, not a bespoke side door.

use crate::standards::v1::subsets::any::schema::mutations::{touch_artifact, SSpaceMutation};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "touch-artifact")]
pub struct TouchArtifact {
    pub id: String,
    pub now_ms: u64,
    pub actor: String,
}

pub fn handle(payload: &TouchArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![touch_artifact(payload.id.clone(), payload.now_ms, payload.actor.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
