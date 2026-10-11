//! 👁️ 👁️ Drawing play app commands command — `engagement-submit`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

/// ✏️ Renames the single selected layer (read from the framework's `"strokes"` interaction
/// selection, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) to the submitted
/// engagement-input text (or the config's own in-progress `engagement_input` if the caller doesn't
/// pass one) — the one `Config`-only row that actually mutates the document, mirroring the
/// pre-migration behaviour exactly.
pub fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let value = payload.value.clone().unwrap_or_else(|| session.window_transient.engagement_input.clone());
    let value = value.trim();
    if value.is_empty() || session.interaction.ids.len() != 1 {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![crate::mutations::rename_layer(session.interaction.ids[0].clone().into(), value.into())]))
}
