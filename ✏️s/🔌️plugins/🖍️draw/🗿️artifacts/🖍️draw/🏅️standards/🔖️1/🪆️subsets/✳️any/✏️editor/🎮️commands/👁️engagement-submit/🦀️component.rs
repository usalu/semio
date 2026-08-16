//! 👁️ 👁️ Draw play app commands command — `engagement-submit`.

use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

/// ✏️ Renames the single selected layer (read from the framework's `"strokes"` interaction
/// selection, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) to the submitted
/// engagement-input text (or the config's own in-progress `engagement_input` if the caller doesn't
/// pass one) — the one `Config`-only row that actually mutates the document, mirroring the
/// pre-migration behaviour exactly.
pub fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let value = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
    let value = value.trim();
    if value.is_empty() || session.interaction.ids.len() != 1 {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::rename_layer(session.interaction.ids[0].clone(), value.into())]))
}
