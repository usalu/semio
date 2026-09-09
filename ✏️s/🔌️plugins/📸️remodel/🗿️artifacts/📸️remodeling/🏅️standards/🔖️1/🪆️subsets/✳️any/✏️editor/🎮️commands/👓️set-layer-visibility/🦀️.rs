//! 👁️ 👁️ Remodeling play app commands command — `set-layer-visibility`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "layer-visibility")]
pub struct SetLayerVisibility {
    pub layer: String,
    pub visible: bool,
}

pub fn handle(payload: &SetLayerVisibility, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelingConfigMutation::SetLayerVisibility(crate::editor::remodeling::config::SetLayerVisibility { layer: payload.layer.clone(), visible: payload.visible })]))
}
