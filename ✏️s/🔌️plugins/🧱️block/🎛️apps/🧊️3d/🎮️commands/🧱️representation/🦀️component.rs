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
        Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::create_representation(representation)]))
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
        Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::delete_representation(payload.id.clone())]))
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
        if !doc.snapshot.representations.iter().any(|representation| representation.id == payload.id) {
            return Ok(Emit::default());
        }
        use crate::artifacts::block3d::mutations as m;
        let mutation = match payload.field.as_str() {
            "name" => m::rename_representation(payload.id.clone(), payload.value.clone()),
            "meshUrl" | "mesh_url" => m::change_representation_mesh_url(payload.id.clone(), if payload.value.is_empty() { None } else { Some(payload.value.clone()) }),
            "lod" => m::change_representation_lod(payload.id.clone(), if payload.value.is_empty() { None } else { Some(payload.value.clone()) }),
            "description" => m::change_representation_description(payload.id.clone(), payload.value.clone()),
            _ => return Ok(Emit::default()),
        };
        Ok(Emit::mutations(vec![mutation]))
    }
}
