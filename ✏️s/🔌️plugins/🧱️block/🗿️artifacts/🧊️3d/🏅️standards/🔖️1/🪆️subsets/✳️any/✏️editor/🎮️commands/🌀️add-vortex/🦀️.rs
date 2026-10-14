//! 🌀️ 🌀️ Block 3D play app commands command — `add-vortex`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::{Block3dSnapshot, Block3dVortexTemplate};
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "addVortex")]
pub struct AddVortex {}

pub fn handle(_payload: &AddVortex, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let vortex_kinds = semio_s_artifact_block_3d_model::vortex_kinds_of(doc.snapshot);
    let Some(vortex_kind_id) = vortex_kinds.first().map(|kind| kind.id.clone()) else {
        return Ok(Emit::default());
    };
    let id = semio_s_artifact_block_3d_schema::next_id(doc.snapshot.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
    let vortex = Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 1.0], radius: 0.3, label: None };
    Ok(Emit::mutations(vec![semio_s_artifact_block_3d_mut_vortices::create_vortex(vortex)]))
}
