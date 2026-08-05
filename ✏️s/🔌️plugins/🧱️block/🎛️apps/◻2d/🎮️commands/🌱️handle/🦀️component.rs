//! 🌱️ Block 2D play app commands — add/remove a rim-handle template.

pub mod add_handle {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
    use crate::artifacts::block2d::op::Block2dOperation;
    use crate::artifacts::block2d::{Block2dDefinition, Block2dHandleTemplate};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addHandle")]
    pub struct AddHandle {}

    pub fn handle(_payload: &AddHandle, doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dOperation, Block2dConfigOperation>, Fault> {
        let Some(handle_kind_id) = doc.projection.handle_kinds.first().map(|kind| kind.id.clone()) else {
            return Ok(Emit::default());
        };
        let id = crate::artifacts::block2d::engine::next_id(doc.projection.handles.iter().map(|handle| handle.id.as_str()), "handle-");
        let handle = Block2dHandleTemplate { id, handle_kind: handle_kind_id, angle: 0.0, radius: 0.36 };
        Ok(Emit::operations(vec![Block2dOperation::SetHandle { index: doc.projection.handles.len(), handle }]))
    }
}

pub mod remove_handle {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
    use crate::artifacts::block2d::op::Block2dOperation;
    use crate::artifacts::block2d::Block2dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeHandle")]
    pub struct RemoveHandle {
        pub id: String,
    }

    pub fn handle(payload: &RemoveHandle, _doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dOperation, Block2dConfigOperation>, Fault> {
        Ok(Emit::operations(vec![Block2dOperation::RemoveHandle { id: payload.id.clone() }]))
    }
}
