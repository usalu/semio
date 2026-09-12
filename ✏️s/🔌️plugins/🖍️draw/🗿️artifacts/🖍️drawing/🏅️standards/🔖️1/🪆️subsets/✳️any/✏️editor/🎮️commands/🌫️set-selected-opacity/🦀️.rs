//! 🗂️ 🗂️ Drawing play app commands command — `set-selected-opacity`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::find_drawing_layer;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "selected-opacity")]
pub struct SetSelectedOpacity {
    pub value: f64,
}

pub fn handle(payload: &SetSelectedOpacity, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operations: Vec<DrawingMutation> = session.interaction.ids.iter().filter(|id| find_drawing_layer(document, id).is_some()).map(|id| crate::mutations::set_layer_opacity(id.clone(), payload.value)).collect();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(operations, "opacity"))
}
