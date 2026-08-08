//! 🔘️ Block 3D play app commands — add/remove a vortex-kind catalog row.

pub mod add_vortex_kind {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addVortexKind")]
    pub struct AddVortexKind {}

    pub fn handle(_payload: &AddVortexKind, doc: &DocumentView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let id = crate::artifacts::block3d::engine::next_id(doc.snapshot.vortex_kinds.iter().map(|kind| kind.id.as_str()), "vortex-kind-");
        let vortex_kind = Block3dVortexKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_cable_kind: "cable.link".into() };
        Ok(Emit::mutations(vec![Block3dMutation::SetVortexKind { index: doc.snapshot.vortex_kinds.len(), vortex_kind }]))
    }
}

pub mod remove_vortex_kind {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeVortexKind")]
    pub struct RemoveVortexKind {
        pub id: String,
    }

    pub fn handle(payload: &RemoveVortexKind, _doc: &DocumentView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![Block3dMutation::RemoveVortexKind { id: payload.id.clone() }]))
    }
}
