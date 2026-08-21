//! 👁️ 👁️ Trinity Rewrite app command — `set-lod-mode`.

use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::editor::rewrite::config::RewriteConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn set_lod_mode(window_id: &str, value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetLodMode { window_id: window_id.to_string(), value: value.to_string() }]))
}
