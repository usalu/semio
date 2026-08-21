//! 👁️ 👁️ Trinity Rewrite app command — `set-locale`.

use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::editor::rewrite::config::RewriteConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn set_locale(value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetLocale { value: value.to_string() }]))
}
