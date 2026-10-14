//! 🖌️ 🖌️ Block 3D play app commands command — `hover-surface`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎯️ Manifest action id `worldSurfaceHover`, wire key `hoverSurface` — the two diverge (unlike
/// every other row in this plugin), preserved verbatim from the pre-migration `#[dsl(key)]`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "hoverSurface")]
pub struct HoverSurface {
    pub window_id: String,
    pub object_id: String,
    pub position: [f64; 3],
    pub normal: [f64; 3],
}

pub fn handle(payload: &HoverSurface, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let _ = payload;
    Ok(Emit::default())
}
