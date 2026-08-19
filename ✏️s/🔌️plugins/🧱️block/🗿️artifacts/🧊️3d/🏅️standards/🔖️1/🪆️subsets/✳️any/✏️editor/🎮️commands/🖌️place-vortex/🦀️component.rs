//! 🖌️ 🖌️ Block 3D play app commands command — `place-vortex`.

use crate::editor::block3d::config::{block3d_window_view, Block3dConfig, Block3dConfigMutation};
use crate::editor::block3d::world::{default_vortex_kind, instance_offset_for_representation, resolve_brush_vortex_kind_id};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🎯️ Manifest action id `worldSurfacePlace`, wire key `placeVortex`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "placeVortex")]
pub struct PlaceVortex {
    pub window_id: String,
    pub object_id: String,
    pub position: [f64; 3],
    pub normal: [f64; 3],
}

pub async fn handle(payload: &PlaceVortex, doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let view = block3d_window_view(cfg.snapshot, &payload.window_id);
    let offset = instance_offset_for_representation(doc.snapshot, &view, &payload.object_id);
    let local_position = [payload.position[0] - offset[0], payload.position[1] - offset[1], payload.position[2] - offset[2]];
    let direction = if cfg.snapshot.brush_flip { [-payload.normal[0], -payload.normal[1], -payload.normal[2]] } else { payload.normal };
    let vortex_kind_id = resolve_brush_vortex_kind_id(doc.snapshot, cfg.snapshot);
    let mut operations = Vec::new();
    if crate::artifacts::block3d::vortex_kinds_of(doc.snapshot).is_empty() {
        operations.push(crate::artifacts::block3d::mutations::create_vortex_kind(default_vortex_kind()));
    }
    let id = crate::artifacts::block3d::schema::next_id(doc.snapshot.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
    operations.push(crate::artifacts::block3d::mutations::create_vortex(Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: local_position, direction, radius: cfg.snapshot.brush_radius, label: None }));
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Block3dConfigMutation::SetBrushPreview { preview: None }], description: None, ..Default::default() })
}
