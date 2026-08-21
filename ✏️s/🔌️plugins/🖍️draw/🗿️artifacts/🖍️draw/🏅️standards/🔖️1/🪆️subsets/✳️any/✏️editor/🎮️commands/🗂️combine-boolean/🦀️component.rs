//! 🗂️ 🗂️ Draw play app commands command — `combine-boolean`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::create_draw_boolean_layer;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "combine-boolean")]
pub struct CombineBoolean {
    pub operation: String,
    pub ids: Vec<String>,
}

pub async fn handle(payload: &CombineBoolean, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let ids: Vec<String> = if payload.ids.is_empty() { session.interaction.ids.clone() } else { payload.ids.clone() };
    if ids.len() < 2 {
        return Ok(Emit::default());
    }
    let layer = create_draw_boolean_layer("Boolean", &payload.operation, ids);
    Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(None, Some(document.layers.len()), layer)], ..Default::default() })
}
