//! 🎯️ 🎯️ Remodeling play app commands command — `remove-gcp`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::mutations::delete_gcp;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-gcp")]
pub struct RemoveGcp {
    pub gcp_id: String,
}

pub fn handle(payload: &RemoveGcp, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_gcp(payload.gcp_id.clone())]))
}
