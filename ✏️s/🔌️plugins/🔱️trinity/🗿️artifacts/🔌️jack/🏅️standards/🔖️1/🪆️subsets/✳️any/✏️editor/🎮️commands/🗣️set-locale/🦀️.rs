//! 👁️ 👁️ Trinity Jack app command — `set-locale`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_locale(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetLocale(crate::editor::jack::config::SetLocale { value: value.to_string() })]))
}
