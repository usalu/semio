//! 👁️ 👁️ Trinity Jack app command — `editor-engagement-input`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn editor_engagement_input(value: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetEditorEngagementInput(crate::editor::jack::config::SetEditorEngagementInput { value: value.to_string() })])
}
