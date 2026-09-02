//! 👁️ 👁️ Trinity Rewriting app command — `set-locale`.

use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_locale(value: &str) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewritingConfigMutation::SetLocale { value: value.to_string() }]))
}
