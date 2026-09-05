//! 👁️ 👁️ Trinity Rewriting app command — `set-lod-mode`.

use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewritingConfigMutation::SetLodMode { window_id: window_id.to_string(), value: value.to_string() }]))
}
