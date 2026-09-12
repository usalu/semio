//! 📄️ 📄️ Drawing play app commands command — `commit-document`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "commit-document")]
pub struct CommitDocument {
    #[dsl(block)]
    pub snapshot: DrawingSnapshot,
}

pub fn handle(
    payload: &CommitDocument,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, NoConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&payload.snapshot)], ..Default::default() })
}
