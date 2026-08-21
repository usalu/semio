//! 🗂️ 🗂️ Draw play app commands command — `set-selected-opacity`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "selected-opacity")]
pub struct SetSelectedOpacity {
    pub value: f64,
}

pub async fn handle(payload: &SetSelectedOpacity, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operations: Vec<DrawMutation> = session.interaction.ids.iter().filter(|id| find_draw_layer(document, id).is_some()).map(|id| crate::artifacts::draw::mutations::set_layer_opacity(id.clone(), payload.value)).collect();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(operations, "opacity"))
}
