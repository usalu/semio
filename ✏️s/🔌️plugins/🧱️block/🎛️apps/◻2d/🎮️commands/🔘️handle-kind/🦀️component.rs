//! 🔘️ Block 2D play app commands — add/remove a handle-kind catalog row.

pub mod add_handle_kind {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
    use crate::artifacts::block2d::op::Block2dOperation;
    use crate::artifacts::block2d::{Block2dDefinition, Block2dHandleKind};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addHandleKind")]
    pub struct AddHandleKind {}

    pub fn handle(_payload: &AddHandleKind, doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dOperation, Block2dConfigOperation>, Fault> {
        let id = crate::artifacts::block2d::engine::next_id(doc.projection.handle_kinds.iter().map(|kind| kind.id.as_str()), "handle-kind-");
        let handle_kind = Block2dHandleKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_wire_kind: "cable.link".into() };
        Ok(Emit::operations(vec![Block2dOperation::SetHandleKind { index: doc.projection.handle_kinds.len(), handle_kind }]))
    }
}

pub mod remove_handle_kind {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
    use crate::artifacts::block2d::op::Block2dOperation;
    use crate::artifacts::block2d::Block2dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeHandleKind")]
    pub struct RemoveHandleKind {
        pub id: String,
    }

    pub fn handle(payload: &RemoveHandleKind, _doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dOperation, Block2dConfigOperation>, Fault> {
        Ok(Emit::operations(vec![Block2dOperation::RemoveHandleKind { id: payload.id.clone() }]))
    }
}
