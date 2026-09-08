//! 👁️ 👁️ Trinity Jack app command — `set-locale`.

use crate::op::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_locale(value: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetLocale(crate::editor::jack::config::SetLocale { value: value.to_string() })])
}
