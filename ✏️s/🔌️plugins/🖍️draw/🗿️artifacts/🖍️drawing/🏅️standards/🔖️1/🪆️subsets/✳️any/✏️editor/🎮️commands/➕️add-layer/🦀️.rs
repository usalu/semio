//! 🗂️ 🗂️ Drawing play app commands command — `add-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::create_layer_by_kind;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_by_kind(&payload.kind);
    Ok(Emit { artifact_mutations: vec![crate::mutations::create_layer(None, Some(document.layers.len()), layer)], ..Default::default() })
}
