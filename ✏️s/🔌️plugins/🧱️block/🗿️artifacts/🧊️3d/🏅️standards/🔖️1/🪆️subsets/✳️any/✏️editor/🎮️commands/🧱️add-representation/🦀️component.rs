//! 🧱️ 🧱️ Block 3D play app commands command — `add-representation`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::BlockRepresentation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "addRepresentation")]
pub struct AddRepresentation {}

pub async fn handle(_payload: &AddRepresentation, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let id = crate::artifacts::block3d::schema::next_id(doc.snapshot.representations.iter().map(|representation| representation.id.as_str()), "representation-");
    let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
    Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::create_representation(representation)]))
}
