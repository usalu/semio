//! 🗂️ 🗂️ Draw play app commands command — `add-layer`.

use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::create_layer_by_kind;
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_by_kind(&payload.kind);
    Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(None, Some(document.layers.len()), layer)], ..Default::default() })
}
