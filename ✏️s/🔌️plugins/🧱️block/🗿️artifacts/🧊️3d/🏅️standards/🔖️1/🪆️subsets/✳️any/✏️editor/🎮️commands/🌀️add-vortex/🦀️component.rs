//! 🌀️ 🌀️ Block 3D play app commands command — `add-vortex`.

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "addVortex")]
pub struct AddVortex {}

pub async fn handle(_payload: &AddVortex, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let vortex_kinds = crate::artifacts::block3d::vortex_kinds_of(doc.snapshot);
    let Some(vortex_kind_id) = vortex_kinds.first().map(|kind| kind.id.clone()) else {
        return Ok(Emit::default());
    };
    let id = crate::artifacts::block3d::schema::next_id(doc.snapshot.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
    let vortex = Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 1.0], radius: 0.3, label: None };
    Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::create_vortex(vortex)]))
}
