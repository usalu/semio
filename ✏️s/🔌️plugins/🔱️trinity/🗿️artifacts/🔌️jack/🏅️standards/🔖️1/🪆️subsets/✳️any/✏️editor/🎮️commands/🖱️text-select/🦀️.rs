//! 👁️ 👁️ Trinity Jack app command — `text-select`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::editor::jack::config::{JackConfigMutation, JackEditorSelection};
use semio_framework_plugin::Emit;

pub(crate) fn text_select(start: u64, end: u64) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetEditorSelection(crate::editor::jack::config::SetEditorSelection { selection: Some(JackEditorSelection { start, end }) })])
}
