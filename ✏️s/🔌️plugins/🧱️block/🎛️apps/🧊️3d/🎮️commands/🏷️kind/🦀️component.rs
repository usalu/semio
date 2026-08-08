//! 🏷️ Block 3D play app command — patch a field on the object kind's identity.

pub mod patch_object_kind {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchObjectKind")]
    pub struct PatchObjectKind {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchObjectKind, doc: &DocumentView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let mut object_kind = doc.snapshot.object_kind.clone();
        match payload.field.as_str() {
            "name" => object_kind.name = payload.value.clone(),
            "label" => object_kind.label = payload.value.clone(),
            "variant" => object_kind.variant = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            "description" => object_kind.description = payload.value.clone(),
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::mutations(vec![Block3dMutation::SetObjectKind { object_kind }]))
    }
}
