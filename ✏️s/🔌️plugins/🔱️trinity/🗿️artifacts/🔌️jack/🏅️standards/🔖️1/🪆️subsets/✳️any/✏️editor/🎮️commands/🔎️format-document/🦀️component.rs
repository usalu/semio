//! 🔎️ 🔎️ Trinity Jack app command — `format-document`.

use crate::editor::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::core;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn format_document(jack_query: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match core::format(jack_query) {
        Ok(formatted) => Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: formatted }])),
        Err(_) => Ok(Emit::default()),
    }
}
