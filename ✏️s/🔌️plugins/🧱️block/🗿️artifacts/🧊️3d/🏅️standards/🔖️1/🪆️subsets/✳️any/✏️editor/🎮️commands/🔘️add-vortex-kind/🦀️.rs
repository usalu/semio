//! 🔘️ 🔘️ Block 3D play app commands command — `add-vortex-kind`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::{Block3dSnapshot, Block3dVortexKind};
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "addVortexKind")]
pub struct AddVortexKind {}

pub fn handle(_payload: &AddVortexKind, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let vortex_kinds = semio_s_artifact_block_3d_model::vortex_kinds_of(doc.snapshot);
    let id = semio_s_artifact_block_3d_schema::next_id(vortex_kinds.iter().map(|kind| kind.id.as_str()), "vortex-kind-");
    let vortex_kind = Block3dVortexKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_cable_kind: "cable.link".into() };
    Ok(Emit::mutations(vec![semio_s_artifact_block_3d_mut_vortex_kinds::create_vortex_kind(vortex_kind)]))
}
