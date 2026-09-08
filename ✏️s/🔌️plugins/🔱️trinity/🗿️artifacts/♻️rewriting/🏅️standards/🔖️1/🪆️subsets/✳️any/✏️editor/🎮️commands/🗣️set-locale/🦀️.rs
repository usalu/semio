//! 👁️ 👁️ Trinity Rewriting app command — `set-locale`.

use crate::op::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_locale(value: &str) -> Emit<RewriteRuleMutation, RewritingConfigMutation> {
    Emit::config(vec![RewritingConfigMutation::SetLocale(crate::editor::rewriting::config::SetLocale { value: value.to_string() })])
}
