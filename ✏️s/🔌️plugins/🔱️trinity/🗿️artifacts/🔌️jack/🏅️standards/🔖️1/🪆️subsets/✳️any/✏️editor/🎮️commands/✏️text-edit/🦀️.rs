//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::editor::jack::config::JackConfigMutation;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::Emit;

pub(crate) fn text_edit(text: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetQuery(crate::editor::jack::config::SetQuery { value: text.to_string() })])
}
