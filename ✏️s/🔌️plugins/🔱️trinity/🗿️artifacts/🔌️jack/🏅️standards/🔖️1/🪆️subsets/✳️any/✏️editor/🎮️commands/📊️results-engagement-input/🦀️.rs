//! 👁️ 👁️ Trinity Jack app command — `results-engagement-input`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn results_engagement_input(value: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetResultsEngagementInput(crate::editor::jack::config::SetResultsEngagementInput { value: value.to_string() })])
}
