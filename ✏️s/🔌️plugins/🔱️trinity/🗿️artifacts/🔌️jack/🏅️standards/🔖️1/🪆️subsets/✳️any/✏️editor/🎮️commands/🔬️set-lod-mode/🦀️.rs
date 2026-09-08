//! 👁️ 👁️ Trinity Jack app command — `set-lod-mode`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetLodMode(crate::editor::jack::config::SetLodMode { window_id: window_id.to_string(), value: value.to_string() })])
}
