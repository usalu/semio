//! 🖌️ Block 3D play app — world window option: the surface-brush's vortex-kind/radius/flip group.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::Block3dConfig;
use crate::editor::block3d::terminology::Block3dLabels;
use crate::editor::block3d::world::resolve_brush_vortex_kind_id;
use crate::editor::block3d::BLOCK3D_UTILITY_SURFACE_BRUSH;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

pub fn measure(definition: &Block3dSnapshot, config: &Block3dConfig, labels: &Block3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "block3d-brush-options".into(),
        label: labels.brush.as_str().to_string(),
        default_open: Some(true),
        active_utility_id: Some(BLOCK3D_UTILITY_SURFACE_BRUSH.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Select {
                id: "block3d-brush-kind".into(),
                label: Some(labels.vortex_kinds.as_str().to_string()),
                value: resolve_brush_vortex_kind_id(definition, config),
                items: crate::artifacts::block3d::vortex_kinds_of(definition).iter().map(|kind| MeasureSelectItem { id: kind.id.clone(), value: kind.id.clone(), label: kind.label.clone() }).collect(),
                on_change: crate::editor::block3d::block3d_window_action("setBrushVortexKind", None),
            },
            WindowMeasure::Slider {
                id: "block3d-brush-radius".into(),
                label: Some(labels.brush_radius.as_str().to_string()),
                value: config.brush_radius,
                min: 0.05,
                max: 2.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: crate::editor::block3d::block3d_window_action("setBrushRadius", None),
            },
            WindowMeasure::Toggle {
                id: "block3d-brush-flip".into(),
                icon_id: "flip-vertical".into(),
                label: Some(labels.flip_normal.as_str().to_string()),
                pressed: config.brush_flip,
                text: None,
                on_change: crate::editor::block3d::block3d_window_action("setBrushFlip", Some(dsl::DslValue::object([("flip".to_string(), dsl::DslValue::Bool(!config.brush_flip))]))),
            },
        ],
    }
}
