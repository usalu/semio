//! 🔘️ 🔘️ Block 3D play app commands command — `add-vortex-kind`.

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "addVortexKind")]
pub struct AddVortexKind {}

pub async fn handle(_payload: &AddVortexKind, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let vortex_kinds = crate::artifacts::block3d::vortex_kinds_of(doc.snapshot);
    let id = crate::artifacts::block3d::schema::next_id(vortex_kinds.iter().map(|kind| kind.id.as_str()), "vortex-kind-");
    let vortex_kind = Block3dVortexKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_cable_kind: "cable.link".into() };
    Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::create_vortex_kind(vortex_kind)]))
}
