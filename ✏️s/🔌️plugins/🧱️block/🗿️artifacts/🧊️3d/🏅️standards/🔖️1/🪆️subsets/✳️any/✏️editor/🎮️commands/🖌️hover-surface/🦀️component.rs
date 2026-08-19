//! 🖌️ 🖌️ Block 3D play app commands command — `hover-surface`.

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dBrushPreview;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🎯️ Manifest action id `worldSurfaceHover`, wire key `hoverSurface` — the two diverge (unlike
/// every other row in this plugin), preserved verbatim from the pre-migration `#[dsl(key)]`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "hoverSurface")]
pub struct HoverSurface {
    pub window_id: String,
    pub object_id: String,
    pub position: [f64; 3],
    pub normal: [f64; 3],
}

pub async fn handle(payload: &HoverSurface, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetBrushPreview { preview: Some(Block3dBrushPreview { position: payload.position, direction: payload.normal }) }]))
}
