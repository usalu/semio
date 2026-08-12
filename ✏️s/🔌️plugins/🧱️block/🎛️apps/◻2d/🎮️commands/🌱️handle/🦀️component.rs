//! 🌱️ Block 2D play app commands — add/remove a rim-handle template.

pub mod add_handle {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::{Block2dSnapshot, Block2dHandleTemplate};
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addHandle")]
    pub struct AddHandle {}

    pub fn handle(_payload: &AddHandle, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        let Some(handle_kind_id) = doc.snapshot.handle_kinds.first().map(|kind| kind.id.clone()) else {
            return Ok(Emit::default());
        };
        let id = crate::artifacts::block2d::engine::next_id(doc.snapshot.handles.iter().map(|handle| handle.id.as_str()), "handle-");
        let handle = Block2dHandleTemplate { id, handle_kind: handle_kind_id, angle: 0.0, radius: 0.36 };
        Ok(Emit::mutations(vec![crate::artifacts::block2d::mutations::create_handle(handle)]))
    }
}

pub mod remove_handle {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::Block2dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeHandle")]
    pub struct RemoveHandle {
        pub id: String,
    }

    pub fn handle(payload: &RemoveHandle, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![crate::artifacts::block2d::mutations::delete_handle(payload.id.clone())]))
    }
}
