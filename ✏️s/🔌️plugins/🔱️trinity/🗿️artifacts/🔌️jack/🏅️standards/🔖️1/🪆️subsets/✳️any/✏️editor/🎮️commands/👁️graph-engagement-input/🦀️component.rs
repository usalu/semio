//! 👁️ 👁️ Trinity Jack app command — `graph-engagement-input`.

use crate::editor::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn graph_engagement_input(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetGraphEngagementInput { value: value.to_string() }]))
}
