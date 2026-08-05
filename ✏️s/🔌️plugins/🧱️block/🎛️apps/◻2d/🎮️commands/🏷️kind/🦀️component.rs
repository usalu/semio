//! 🏷️ Block 2D play app command — patch a field on the node kind's identity.

pub mod patch_node_kind {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
    use crate::artifacts::block2d::op::Block2dOperation;
    use crate::artifacts::block2d::Block2dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchNodeKind")]
    pub struct PatchNodeKind {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchNodeKind, doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dOperation, Block2dConfigOperation>, Fault> {
        let mut node_kind = doc.projection.node_kind.clone();
        match payload.field.as_str() {
            "name" => node_kind.name = payload.value.clone(),
            "label" => node_kind.label = payload.value.clone(),
            "variant" => node_kind.variant = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            "description" => node_kind.description = payload.value.clone(),
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::operations(vec![Block2dOperation::SetNodeKind { node_kind }]))
    }
}
