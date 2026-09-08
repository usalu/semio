//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-min-availability`.

use crate::op::SourcingMutation;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "filter-min-availability")]
pub struct SetFilterMinAvailability {
    pub delta: Option<f64>,
    pub value: Option<f64>,
}

pub fn handle(payload: &SetFilterMinAvailability, _doc: &ArtifactView<'_, CurationSnapshot>, cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let current = cfg.snapshot.filters.min_availability as f64;
    let next = payload.delta.map(|d| current + d).or(payload.value).unwrap_or(current);
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetFilterMinAvailability { value: next.max(0.0) as u32 }]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
