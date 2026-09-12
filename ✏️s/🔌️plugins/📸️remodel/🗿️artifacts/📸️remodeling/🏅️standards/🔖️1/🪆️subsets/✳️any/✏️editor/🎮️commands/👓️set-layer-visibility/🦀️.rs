//! 👁️ 👁️ Remodeling play app commands command — `set-layer-visibility`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
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

pub fn handle(_payload: &SetLayerVisibility, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
