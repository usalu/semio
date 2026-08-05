//! 🎥️ Block 3D play app command — set the free/live world camera pose. Config-only.

pub mod set_camera {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use crate::core::BlockCamera3d;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setCamera")]
    pub struct SetCamera {
        pub camera: BlockCamera3d,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigOperation::SetCamera { camera: payload.camera.clone() }]))
    }
}
