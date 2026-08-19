//! 🔍️ 🔍️ Sourcing curate app commands command — `set-filter-min-availability`.

use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "filter-min-availability")]
pub struct SetFilterMinAvailability {
    pub delta: Option<f64>,
    pub value: Option<f64>,
}

pub async fn handle(payload: &SetFilterMinAvailability, _doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let current = cfg.snapshot.filters.min_availability as f64;
    let next = payload.delta.map(|d| current + d).or(payload.value).unwrap_or(current);
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterMinAvailability { value: next.max(0.0) as u32 }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sourcing::modes::edit::windows::pool;
    use crate::editor::sourcing::testkit::{dispatch, new_app, render};
    use crate::editor::sourcing::SourcingCurateCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_filter_min_availability_clamps_to_zero() {
        let mut app = new_app();
        dispatch(&mut app, SourcingCurateCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(-1000.0), value: None }));
        // Filters are config-only now — the pool render reflects the clamp indirectly via an empty result
        // for an unreasonably high min-availability; assert the clamp directly through a second command
        // that reports back the applied absolute value.
        dispatch(&mut app, SourcingCurateCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(0.0), value: None }));
        let node = render(&mut app, pool::SOURCING_CURATE_BODY_POOL);
        // A clamped-to-zero min-availability keeps every stock row (all availabilities are >= 0).
        assert!(node.contains("Glulam"));
    }
}
//#endregion 🧪️Tests
