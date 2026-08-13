//! 🗂️ 🗂️ Block 3D play app commands command — `select-vortex`.

use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "selectVortex")]
pub struct SelectVortex {
    pub full_id: String,
    pub merge: bool,
}

pub fn handle(payload: &SelectVortex, _doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let local = payload.full_id.split_once(':').map_or(payload.full_id.as_str(), |(_, tail)| tail);
    let id = format!("vortex:{local}");
    let mut ids = if payload.merge { cfg.snapshot.selected_ids.clone() } else { Vec::new() };
    if !ids.contains(&id) {
        ids.push(id);
    }
    Ok(Emit::config(vec![Block3dConfigMutation::SetSelection { ids }]))
}
