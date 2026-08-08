//! 🎥️ Block 3D play app command — set the free/live world camera pose. Config-only.

pub mod set_camera {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use crate::BlockCamera3d;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setCamera")]
    pub struct SetCamera {
        pub camera: BlockCamera3d,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
