//! 👁️ 👁️ Trinity Jack app command — `results-engagement-input`.

use crate::editor::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn results_engagement_input(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetResultsEngagementInput { value: value.to_string() }]))
}
