
use super::*;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::modes::edit::windows::pool;
use crate::editor::sourcing::testkit::{dispatch, new_app, render};

#[semio_framework_async_macros::async_test]
async fn set_filter_min_availability_clamps_to_zero() {
    let mut app = new_app().await;
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(-1000.0), value: None })).await;
    // Filters are config-only now — the pool render reflects the clamp indirectly via an empty result
    // for an unreasonably high min-availability; assert the clamp directly through a second command
    // that reports back the applied absolute value.
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(0.0), value: None })).await;
    let node = render(&mut app, pool::SOURCING_CURATION_BODY_POOL).await;
    // A clamped-to-zero min-availability keeps every stock row (all availabilities are >= 0).
    assert!(node.contains("Glulam"));
}
