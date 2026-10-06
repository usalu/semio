//! 📍️ Block 3D play app command — `place-vortex`: one surface click through the framework's one-step tool
//! (`Emit::tool_once`), ONE tool transaction of everything the click creates (design §22.32 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;
use crate::{Block3dSnapshot, Block3dVortexTemplate};
use crate::editor::block3d::config::{block3d_window_view, Block3dConfig, Block3dConfigMutation};
use crate::editor::block3d::world::{default_vortex_kind, instance_offset_for_representation, resolve_brush_vortex_kind_id};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪪️ The tool verb every place transaction is stamped with: `block3d-play#worldSurfacePlace`.
pub const PLACE_VORTEX_VERB: &str = "worldSurfacePlace";

/// 🎯️ Manifest action id `worldSurfacePlace`, wire key `placeVortex`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "placeVortex")]
pub struct PlaceVortex {
    pub window_id: String,
    pub object_id: String,
    pub position: [f64; 3],
    pub normal: [f64; 3],
}

pub fn handle(payload: &PlaceVortex, doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let view = block3d_window_view(cfg.snapshot, &payload.window_id);
    let offset = instance_offset_for_representation(doc.snapshot, &view, &payload.object_id);
    let local_position = [payload.position[0] - offset[0], payload.position[1] - offset[1], payload.position[2] - offset[2]];
    let direction = if cfg.snapshot.brush_flip { [-payload.normal[0], -payload.normal[1], -payload.normal[2]] } else { payload.normal };
    let vortex_kind_id = resolve_brush_vortex_kind_id(doc.snapshot, cfg.snapshot);
    let mut operations = Vec::new();
    if crate::vortex_kinds_of(doc.snapshot).is_empty() {
        operations.push(crate::standards::v1::subsets::any::schema::mutations::create_vortex_kind(default_vortex_kind()));
    }
    let id = crate::standards::v1::subsets::any::schema::next_id(doc.snapshot.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
    operations.push(crate::standards::v1::subsets::any::schema::mutations::create_vortex(Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: local_position, direction, radius: cfg.snapshot.brush_radius, label: None }));
    let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    Ok(Emit::tool_once(crate::editor::block3d::BLOCK3D_PLAY_APP_ID, PLACE_VORTEX_VERB, authoring_seed, operations))
}
