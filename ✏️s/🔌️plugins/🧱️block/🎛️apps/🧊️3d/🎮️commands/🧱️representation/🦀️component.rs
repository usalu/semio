//! 🧱️ Block 3D play app commands — add/remove/patch a representation (mesh at a LOD/tag combination).

pub mod add_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use crate::core::BlockRepresentation;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addRepresentation")]
    pub struct AddRepresentation {}

    pub fn handle(_payload: &AddRepresentation, doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        let id = crate::artifacts::block3d::engine::next_id(doc.projection.representations.iter().map(|representation| representation.id.as_str()), "representation-");
        let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
        Ok(Emit::operations(vec![Block3dOperation::SetRepresentation { index: doc.projection.representations.len(), representation }]))
    }
}

pub mod remove_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeRepresentation")]
    pub struct RemoveRepresentation {
        pub id: String,
    }

    pub fn handle(payload: &RemoveRepresentation, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        Ok(Emit::operations(vec![Block3dOperation::RemoveRepresentation { id: payload.id.clone() }]))
    }
}

pub mod patch_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchRepresentation")]
    pub struct PatchRepresentation {
        pub id: String,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchRepresentation, doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        let Some(index) = doc.projection.representations.iter().position(|representation| representation.id == payload.id) else {
            return Ok(Emit::default());
        };
        let mut representation = doc.projection.representations[index].clone();
        match payload.field.as_str() {
            "name" => representation.name = payload.value.clone(),
            "meshUrl" | "mesh_url" => representation.mesh_url = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::operations(vec![Block3dOperation::SetRepresentation { index, representation }]))
    }
}
