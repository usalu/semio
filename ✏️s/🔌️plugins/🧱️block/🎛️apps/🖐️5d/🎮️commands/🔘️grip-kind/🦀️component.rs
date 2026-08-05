//! 🔘️ Block 5D play app commands — add/remove a grip-kind catalog row.

pub mod add_grip_kind {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigOperation};
    use crate::artifacts::block5d::op::Block5dOperation;
    use crate::artifacts::block5d::{Block5dDefinition, Block5dGripKind};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addGripKind")]
    pub struct AddGripKind {}

    pub fn handle(_payload: &AddGripKind, doc: &DocumentView<'_, Block5dDefinition>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dOperation, Block5dConfigOperation>, Fault> {
        let id = crate::artifacts::block5d::engine::next_id(doc.projection.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
        let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
        Ok(Emit::operations(vec![Block5dOperation::SetGripKind { index: doc.projection.grip_kinds.len(), grip_kind }]))
    }
}

pub mod remove_grip_kind {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigOperation};
    use crate::artifacts::block5d::op::Block5dOperation;
    use crate::artifacts::block5d::Block5dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeGripKind")]
    pub struct RemoveGripKind {
        pub id: String,
    }

    pub fn handle(payload: &RemoveGripKind, _doc: &DocumentView<'_, Block5dDefinition>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dOperation, Block5dConfigOperation>, Fault> {
        Ok(Emit::operations(vec![Block5dOperation::RemoveGripKind { id: payload.id.clone() }]))
    }
}
