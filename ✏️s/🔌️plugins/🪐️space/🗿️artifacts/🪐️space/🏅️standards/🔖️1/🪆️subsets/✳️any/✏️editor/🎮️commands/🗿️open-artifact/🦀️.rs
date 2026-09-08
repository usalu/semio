//! 📂️ SpaceIndexEditor commands command — `open-artifact`. Relays `os.open-artifact` with no
//! explicit `role`, so the shell resolves the user's `OpeningPreferences` default (contract §C3/C6,
//! worker-brief task 2: "respects the user's `OpeningPreferences` default" — the space app has no
//! access to that host-side config facet, so it deliberately omits `role` rather than guessing one;
//! see `👁️set-visibility`-sibling command `🗃️open-artifact-with` for the explicit-role "Open with…"
//! chooser).

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "open-artifact")]
pub struct OpenArtifact {
    pub id: String,
}

pub fn handle(payload: &OpenArtifact, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let row = cfg.snapshot.indexed_artifacts.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.index.target-missing"), format!("indexed artifact `{}` not found", payload.id)))?;
    let artifact_ref = format!("{}@{}/{}", row.dialect.artifact_kind, row.dialect.standard, row.dialect.subset);
    Ok(Emit::effect(Effect::ReplayShellCommand {
        action_id: "os.open-artifact".into(),
        args: Some(pack::json_to_dsl_value(&pack::json!({ "artifactRef": artifact_ref, "documentId": row.id.clone(), "spaceId": doc.snapshot.space_id.clone(), "schema": row.schema.clone() }))),
    }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
