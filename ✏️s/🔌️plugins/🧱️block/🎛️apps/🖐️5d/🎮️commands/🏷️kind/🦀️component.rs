//! 🏷️ Block 5D play app command — patch a field on the part kind's identity.

pub mod patch_part_kind {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::Block5dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchPartKind")]
    pub struct PatchPartKind {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchPartKind, doc: &DocumentView<'_, Block5dDefinition>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        let mut part_kind = doc.projection.part_kind.clone();
        match payload.field.as_str() {
            "name" => part_kind.name = payload.value.clone(),
            "label" => part_kind.label = payload.value.clone(),
            "variant" => part_kind.variant = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            "description" => part_kind.description = payload.value.clone(),
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::mutations(vec![Block5dMutation::SetPartKind { part_kind }]))
    }
}
