//! 🧱️ 🧱️ Block 3D play app commands command — `patch-representation`.

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patchRepresentation")]
pub struct PatchRepresentation {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchRepresentation, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
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
