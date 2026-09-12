//! 👁️ 👁️ Remodeling play app commands command — `set-frame-cursor`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "frame-cursor")]
pub struct SetFrameCursor {
    #[value(default)]
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

pub fn handle(_payload: &SetFrameCursor, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
