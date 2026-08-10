//! 🏷️ Block 2D play app command — patch a field on the node kind's identity.

pub mod patch_node_kind {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::Block2dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchNodeKind")]
    pub struct PatchNodeKind {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchNodeKind, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        let mut node_kind = doc.snapshot.node_kind.clone();
        match payload.field.as_str() {
            "name" => node_kind.name = payload.value.clone(),
            "label" => node_kind.label = payload.value.clone(),
            "variant" => node_kind.variant = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            "description" => node_kind.description = payload.value.clone(),
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::mutations(vec![Block2dMutation::SetNodeKind { node_kind }]))
    }
}
