//! 🔎️ 🔎️ Trinity Jack app command — `format-document`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::core;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn format_document(jack_query: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match core::format(jack_query) {
        Ok(formatted) => Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: formatted }])),
        Err(_) => Ok(Emit::default()),
    }
}
