//! 👁️ 👁️ Trinity Jack app command — `text-select`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::editor::jack::config::{JackConfigMutation, JackEditorSelection};
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn text_select(start: u64, end: u64) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetEditorSelection { selection: Some(JackEditorSelection { start, end }) }]))
}
