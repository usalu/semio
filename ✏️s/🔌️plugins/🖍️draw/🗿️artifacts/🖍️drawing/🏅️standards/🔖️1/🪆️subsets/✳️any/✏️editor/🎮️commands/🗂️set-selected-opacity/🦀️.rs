//! 🗂️ 🗂️ Drawing play app commands command — `set-selected-opacity`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "selected-opacity")]
pub struct SetSelectedOpacity {
    pub value: f64,
}

pub fn handle(payload: &SetSelectedOpacity, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operations: Vec<DrawingMutation> = session.interaction.ids.iter().filter(|id| find_drawing_layer(document, id).is_some()).map(|id| crate::artifacts::drawing::mutations::set_layer_opacity(id.clone(), payload.value)).collect();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(operations, "opacity"))
}
