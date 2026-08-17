//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::editor::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: text.to_string() }]))
}
