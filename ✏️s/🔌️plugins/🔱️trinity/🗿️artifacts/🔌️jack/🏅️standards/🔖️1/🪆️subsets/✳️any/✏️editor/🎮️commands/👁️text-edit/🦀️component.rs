//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: text.to_string() }]))
}
