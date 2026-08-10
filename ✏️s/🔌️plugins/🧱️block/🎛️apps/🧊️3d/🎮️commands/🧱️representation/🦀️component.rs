//! 🧱️ Block 3D play app commands — add/remove/patch a representation (mesh at a LOD/tag combination).

pub mod add_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use crate::BlockRepresentation;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addRepresentation")]
    pub struct AddRepresentation {}

    pub fn handle(_payload: &AddRepresentation, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let id = crate::artifacts::block3d::engine::next_id(doc.snapshot.representations.iter().map(|representation| representation.id.as_str()), "representation-");
        let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
        Ok(Emit::mutations(vec![Block3dMutation::SetRepresentation { index: doc.snapshot.representations.len(), representation }]))
    }
}

pub mod remove_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeRepresentation")]
    pub struct RemoveRepresentation {
        pub id: String,
    }

    pub fn handle(payload: &RemoveRepresentation, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![Block3dMutation::RemoveRepresentation { id: payload.id.clone() }]))
    }
}

pub mod patch_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patchRepresentation")]
    pub struct PatchRepresentation {
        pub id: String,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchRepresentation, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let Some(index) = doc.snapshot.representations.iter().position(|representation| representation.id == payload.id) else {
            return Ok(Emit::default());
        };
        let mut representation = doc.snapshot.representations[index].clone();
        match payload.field.as_str() {
            "name" => representation.name = payload.value.clone(),
            "meshUrl" | "mesh_url" => representation.mesh_url = if payload.value.is_empty() { None } else { Some(payload.value.clone()) },
            _ => return Ok(Emit::default()),
        }
        Ok(Emit::mutations(vec![Block3dMutation::SetRepresentation { index, representation }]))
    }
}
