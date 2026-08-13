//! 👁️ 👁️ Trinity Jack app command — `set-locale`.

use crate::apps::jack::config::{JackConfigMutation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_locale(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetLocale { value: value.to_string() }]))
}
