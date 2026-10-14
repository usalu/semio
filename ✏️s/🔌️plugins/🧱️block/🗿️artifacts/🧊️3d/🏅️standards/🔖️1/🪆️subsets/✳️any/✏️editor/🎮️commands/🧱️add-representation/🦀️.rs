//! 🧱️ 🧱️ Block 3D play app commands command — `add-representation`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_s_plugin_block::BlockRepresentation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "addRepresentation")]
pub struct AddRepresentation {}

pub fn handle(_payload: &AddRepresentation, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let id = semio_s_artifact_block_3d_schema::next_id(doc.snapshot.representations.iter().map(|representation| representation.id.as_str()), "representation-");
    let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
    Ok(Emit::mutations(vec![semio_s_artifact_block_3d_mut_representations::create_representation(representation)]))
}
