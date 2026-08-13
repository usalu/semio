//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::apps::jack::config::{JackConfigMutation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: text.to_string() }]))
}
