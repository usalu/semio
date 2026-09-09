//! 👋️ Block 3D play app command — `leave-surface`.

use crate::standards::v1::subsets::any::schema::mutations::text::Block3dMutation;
use crate::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎯️ Manifest action id `worldSurfaceLeave`, wire key `leaveSurface` (from `app_commands!` alias).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct LeaveSurface {}

pub fn handle(_payload: &LeaveSurface, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::default())
}
