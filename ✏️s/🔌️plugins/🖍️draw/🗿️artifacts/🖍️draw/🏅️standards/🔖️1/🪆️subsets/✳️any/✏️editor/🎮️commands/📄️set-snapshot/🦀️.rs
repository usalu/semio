//! 📄️ 📄️ Draw play app commands command — `set-snapshot`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: DrawSnapshot,
}

pub fn handle(
    payload: &SetSnapshot,
    _doc: &ArtifactView<'_, DrawSnapshot>,
    _cfg: &ConfigView<'_, DrawConfig>,
    _session: &mut crate::editor::draw::commands::canvas_pointer_down::DrawSession,
) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    Ok(Emit { effects: vec![crate::editor::draw::draw_reset_document_effect(&payload.snapshot)], ..Default::default() })
}
