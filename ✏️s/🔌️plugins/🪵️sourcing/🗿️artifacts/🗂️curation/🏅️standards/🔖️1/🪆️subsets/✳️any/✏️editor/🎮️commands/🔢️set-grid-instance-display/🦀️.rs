//! 🔢️ Sourcing curation app command — `set-grid-instance-display`.

use crate::op::SourcingMutation;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "grid-instance-display")]
pub struct SetGridInstanceDisplay {
    pub value: String,
}

pub fn handle(_payload: &SetGridInstanceDisplay, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(Emit::default())
}
