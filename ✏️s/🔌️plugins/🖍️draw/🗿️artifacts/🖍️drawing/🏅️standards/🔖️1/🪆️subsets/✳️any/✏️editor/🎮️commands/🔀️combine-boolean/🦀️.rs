//! 🗂️ 🗂️ Drawing play app commands command — `combine-boolean`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::schema::create_drawing_boolean_layer;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "combine-boolean")]
pub struct CombineBoolean {
    pub operation: String,
    pub ids: Vec<String>,
}

pub fn handle(payload: &CombineBoolean, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let ids: Vec<String> = if payload.ids.is_empty() { session.interaction.ids.clone() } else { payload.ids.clone() };
    if ids.len() < 2 {
        return Ok(Emit::default());
    }
    let layer = create_drawing_boolean_layer("Boolean", &payload.operation, ids);
    Ok(Emit { artifact_mutations: vec![crate::artifacts::drawing::mutations::create_layer(None, Some(document.layers.len()), layer)], ..Default::default() })
}
