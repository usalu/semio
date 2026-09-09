//! 🔎️ 🔎️ Trinity Jack app command — `format-document`.

use crate::core;
use crate::editor::jack::config::JackConfigMutation;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::Emit;

pub(crate) fn format_document(jack_query: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    match core::format(jack_query) {
        Ok(formatted) => Emit::config(vec![JackConfigMutation::SetQuery(crate::editor::jack::config::SetQuery { value: formatted })]),
        Err(_) => Emit::default(),
    }
}
