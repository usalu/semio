//! ❔️ SpaceIndexEditor commands command — `request-delete-artifact`. View-only: opens the
//! `deleteArtifact` confirm dialog (worker-brief task 2, "delete-artifact (confirm dialog first)")
//! pre-seeded with the target `id`; the dialog's own submit re-dispatches the real, undecorated
//! `🗑️delete-artifact` command (unchanged — see that file's own doc comment).

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "request-delete-artifact")]
pub struct RequestDeleteArtifact {
    pub id: String,
}

pub fn handle(payload: &RequestDeleteArtifact, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let row = doc.snapshot.artifacts.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.mutation.target-missing"), format!("artifact `{}` not found", payload.id)))?;
    Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(128), dialog_id: "deleteArtifact".into(), args: Some(pack::json_to_dsl_value(&pack::json!({ "id": row.id.clone(), "name": row.name.clone() }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
