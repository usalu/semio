//! 🌀️ Block 3D play app commands — add/remove a rim-vortex template.

pub mod add_vortex {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addVortex")]
    pub struct AddVortex {}

    pub fn handle(_payload: &AddVortex, doc: &DocumentView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let Some(vortex_kind_id) = doc.snapshot.vortex_kinds.first().map(|kind| kind.id.clone()) else {
            return Ok(Emit::default());
        };
        let id = crate::artifacts::block3d::engine::next_id(doc.snapshot.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
        let vortex = Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 1.0], radius: 0.3, label: None };
        Ok(Emit::mutations(vec![Block3dMutation::SetVortex { index: doc.snapshot.vortices.len(), vortex }]))
    }
}

pub mod remove_vortex {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeVortex")]
    pub struct RemoveVortex {
        pub id: String,
    }

    pub fn handle(payload: &RemoveVortex, _doc: &DocumentView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![Block3dMutation::RemoveVortex { id: payload.id.clone() }]))
    }
}
