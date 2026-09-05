//! 👁️ 👁️ Drawing play app commands command — `engagement-submit`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

/// ✏️ Renames the single selected layer (read from the framework's `"strokes"` interaction
/// selection, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) to the submitted
/// engagement-input text (or the config's own in-progress `engagement_input` if the caller doesn't
/// pass one) — the one `Config`-only row that actually mutates the document, mirroring the
/// pre-migration behaviour exactly.
pub fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let value = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
    let value = value.trim();
    if value.is_empty() || session.interaction.ids.len() != 1 {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![crate::artifacts::drawing::mutations::rename_layer(session.interaction.ids[0].clone(), value.into())]))
}
