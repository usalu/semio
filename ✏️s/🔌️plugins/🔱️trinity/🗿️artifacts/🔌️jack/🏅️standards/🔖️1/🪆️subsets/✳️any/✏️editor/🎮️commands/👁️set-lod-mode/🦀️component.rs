//! 👁️ 👁️ Trinity Jack app command — `set-lod-mode`.

use crate::editor::jack::config::{JackConfigMutation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetLodMode { window_id: window_id.to_string(), value: value.to_string() }]))
}
