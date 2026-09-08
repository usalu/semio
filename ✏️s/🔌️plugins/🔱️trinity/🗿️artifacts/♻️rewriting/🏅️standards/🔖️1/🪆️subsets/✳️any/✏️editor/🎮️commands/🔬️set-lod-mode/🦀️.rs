//! 👁️ 👁️ Trinity Rewriting app command — `set-lod-mode`.

use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Emit<RewriteRuleMutation, RewritingConfigMutation> {
    Emit::config(vec![RewritingConfigMutation::SetLodMode(crate::editor::rewriting::config::SetLodMode { window_id: window_id.to_string(), value: value.to_string() })])
}
