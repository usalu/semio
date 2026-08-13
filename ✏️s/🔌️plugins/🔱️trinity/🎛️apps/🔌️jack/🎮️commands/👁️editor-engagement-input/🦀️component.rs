//! 👁️ 👁️ Trinity Jack app command — `editor-engagement-input`.

use crate::apps::jack::config::{JackConfigMutation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn editor_engagement_input(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetEditorEngagementInput { value: value.to_string() }]))
}
