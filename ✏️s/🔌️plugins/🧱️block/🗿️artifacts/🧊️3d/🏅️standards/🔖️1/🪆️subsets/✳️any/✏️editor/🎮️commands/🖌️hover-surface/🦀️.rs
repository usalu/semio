//! 🖌️ 🖌️ Block 3D play app commands command — `hover-surface`.

use crate::standards::v1::subsets::any::schema::mutations::text::Block3dMutation;
use crate::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎯️ Manifest action id `worldSurfaceHover`, wire key `hoverSurface` — the two diverge (unlike
/// every other row in this plugin), preserved verbatim from the pre-migration `#[dsl(key)]`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
