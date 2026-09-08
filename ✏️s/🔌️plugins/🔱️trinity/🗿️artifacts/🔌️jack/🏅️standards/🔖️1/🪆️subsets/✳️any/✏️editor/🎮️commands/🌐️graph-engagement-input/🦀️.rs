//! 👁️ 👁️ Trinity Jack app command — `graph-engagement-input`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn graph_engagement_input(value: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetGraphEngagementInput(crate::editor::jack::config::SetGraphEngagementInput { value: value.to_string() })])
}
