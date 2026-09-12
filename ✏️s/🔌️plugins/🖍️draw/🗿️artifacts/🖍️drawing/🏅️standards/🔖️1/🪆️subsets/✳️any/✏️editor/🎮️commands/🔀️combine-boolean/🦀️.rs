//! 🗂️ 🗂️ Drawing play app commands command — `combine-boolean`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::create_drawing_boolean_layer;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "combine-boolean")]
pub struct CombineBoolean {
    pub operation: String,
    pub ids: Vec<String>,
}

pub fn handle(payload: &CombineBoolean, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let ids: Vec<String> = if payload.ids.is_empty() { session.interaction.ids.clone() } else { payload.ids.clone() };
    if ids.len() < 2 {
        return Ok(Emit::default());
    }
    let layer = create_drawing_boolean_layer("Boolean", &payload.operation, ids);
    Ok(Emit { artifact_mutations: vec![crate::mutations::create_layer(None, Some(document.layers.len()), layer)], ..Default::default() })
}
