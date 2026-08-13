//! 👁️ 👁️ Trinity Rewrite app command — `set-locale`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_locale(value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetLocale { value: value.to_string() }]))
}
