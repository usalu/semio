//! 🔘️ Block 5D play app commands — add/remove a grip-kind catalog row.

pub mod add_grip_kind {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::{Block5dSnapshot, Block5dGripKind};
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addGripKind")]
    pub struct AddGripKind {}

    pub fn handle(_payload: &AddGripKind, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        let id = crate::artifacts::block5d::engine::next_id(doc.snapshot.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
        let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
        Ok(Emit::mutations(vec![Block5dMutation::SetGripKind { index: doc.snapshot.grip_kinds.len(), grip_kind }]))
    }
}

pub mod remove_grip_kind {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::Block5dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeGripKind")]
    pub struct RemoveGripKind {
        pub id: String,
    }

    pub fn handle(payload: &RemoveGripKind, _doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![Block5dMutation::RemoveGripKind { id: payload.id.clone() }]))
    }
}
