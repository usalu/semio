//! 🗂️ 🗂️ Draw play app commands command — `add-layer`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::create_layer_by_kind;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_by_kind(&payload.kind);
    Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(None, Some(document.layers.len()), layer)], ..Default::default() })
}
