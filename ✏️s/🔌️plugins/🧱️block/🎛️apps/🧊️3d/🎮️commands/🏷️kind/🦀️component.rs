//! 🏷️ Block 3D play app command — patch a field on the object kind's identity.

pub mod patch_object_kind {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchObjectKind")]
    pub struct PatchObjectKind {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchObjectKind, doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        let mut object_kind = doc.projection.object_kind.clone();
        match payload.field.as_str() {
            "name" => object_kind.name = payload.value.clone(),
            "label" => object_kind.label = payload.value.clone(),
            "variant" => object_kind.variant = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            "description" => object_kind.description = payload.value.clone(),
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::operations(vec![Block3dOperation::SetObjectKind { object_kind }]))
    }
}
