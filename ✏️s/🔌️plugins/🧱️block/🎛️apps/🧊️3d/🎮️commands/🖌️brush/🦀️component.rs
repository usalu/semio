//! 🖌️ Block 3D play app commands — the surface-brush utility: hover/leave preview, placement, and the
//! brush's own vortex-kind/radius/flip settings. All config-only except `PlaceVortex`, which also
//! emits document operations (creates a default vortex kind on first placement if none exists yet).

pub mod set_brush_vortex_kind {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setBrushVortexKind")]
    pub struct SetBrushVortexKind {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vortex_kind_id: Option<String>,
    }

    pub fn handle(payload: &SetBrushVortexKind, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetBrushVortexKind { vortex_kind_id: payload.vortex_kind_id.clone() }]))
    }
}

pub mod set_brush_radius {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setBrushRadius")]
    pub struct SetBrushRadius {
        pub radius: f64,
    }

    pub fn handle(payload: &SetBrushRadius, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetBrushRadius { radius: payload.radius }]))
    }
}

pub mod set_brush_flip {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setBrushFlip")]
    pub struct SetBrushFlip {
        pub flip: bool,
    }

    pub fn handle(payload: &SetBrushFlip, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetBrushFlip { flip: payload.flip }]))
    }
}

pub mod hover_surface {
    use crate::apps::block3d::config::{Block3dBrushPreview, Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    /// 🎯️ Manifest action id `worldSurfaceHover`, wire key `hoverSurface` — the two diverge (unlike
    /// every other row in this plugin), preserved verbatim from the pre-migration `#[dsl(key)]`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "hoverSurface")]
    pub struct HoverSurface {
        pub window_id: String,
        pub object_id: String,
        pub position: [f64; 3],
        pub normal: [f64; 3],
    }

    pub fn handle(payload: &HoverSurface, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetBrushPreview { preview: Some(Block3dBrushPreview { position: payload.position, direction: payload.normal }) }]))
    }
}

pub mod leave_surface {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    /// 🎯️ Manifest action id `worldSurfaceLeave`, wire key `leaveSurface`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "leaveSurface")]
    pub struct LeaveSurface {}

    pub fn handle(_payload: &LeaveSurface, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetBrushPreview { preview: None }]))
    }
}

pub mod place_vortex {
    use crate::apps::block3d::config::{block3d_window_view, Block3dConfig, Block3dConfigMutation};
    use crate::apps::block3d::world::{default_vortex_kind, instance_offset_for_representation, resolve_brush_vortex_kind_id};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::{Block3dDefinition, Block3dVortexTemplate};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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

    pub fn handle(payload: &PlaceVortex, doc: &DocumentView<'_, Block3dDefinition>, cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let view = block3d_window_view(cfg.projection, &payload.window_id);
        let offset = instance_offset_for_representation(doc.projection, &view, &payload.object_id);
        let local_position = [payload.position[0] - offset[0], payload.position[1] - offset[1], payload.position[2] - offset[2]];
        let direction = if cfg.projection.brush_flip { [-payload.normal[0], -payload.normal[1], -payload.normal[2]] } else { payload.normal };
        let vortex_kind_id = resolve_brush_vortex_kind_id(doc.projection, cfg.projection);
        let mut operations = Vec::new();
        if doc.projection.vortex_kinds.is_empty() {
            operations.push(Block3dMutation::SetVortexKind { index: 0, vortex_kind: default_vortex_kind() });
        }
        let id = crate::artifacts::block3d::engine::next_id(doc.projection.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
        operations.push(Block3dMutation::SetVortex {
            index: doc.projection.vortices.len(),
            vortex: Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: local_position, direction, radius: cfg.projection.brush_radius, label: None },
        });
        Ok(Emit { document_mutations: operations, config_mutations: vec![Block3dConfigMutation::SetBrushPreview { preview: None }], description: None, ..Default::default() })
    }
}
