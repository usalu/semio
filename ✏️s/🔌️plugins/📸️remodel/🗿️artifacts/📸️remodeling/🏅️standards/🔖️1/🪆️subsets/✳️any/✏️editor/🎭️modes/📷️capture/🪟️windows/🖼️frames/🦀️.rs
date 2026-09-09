//! 🖼️ Remodeling play app — the Frames window: a Canvas2d view of the currently cursored frame, with any
//! ground control point observations planted on it.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingFrameCursor};
use crate::RemodelingSnapshot;
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowOptions};
// 🧬️ Two `SurfaceKind` enums coexist: `WindowKindDefinition` carries the retained `ui_wgpu` one
// (re-exported by the SDK root), while `scene_surface` takes the semantic contract's — same spelling,
// different types, so both are imported explicitly.
use pack::JsonValue;
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const REMODELING_PLAY_WINDOW_FRAMES: &str = "remodeling-frames";
pub const REMODELING_PLAY_BODY_FRAMES: &str = "remodeling.play.frames";
const REMODELING_PLAY_SURFACE_FRAMES: &str = "remodeling.play.frames";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: REMODELING_PLAY_WINDOW_FRAMES.into(),
        label: LocalizedLabel::native("Frames", "Frames"),
        body_key: REMODELING_PLAY_BODY_FRAMES.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::None },
        actions: Vec::new(),
        utilities: ["select", "gcpPlace"].iter().map(|id| UtilityRef::from(*id)).collect(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🎯️ Half-side of a ground-control-point marker, in frame pixels. The host draws a `circle` layer
/// from its bounds rectangle, so the marker is a square box centred on the observed pixel.
const GCP_MARKER_RADIUS_PX: f64 = 6.0;

/// 🖼️ The cursored frame image plus every GCP observation planted on it, in the exact record shape
/// `JsonLayersCanvasSession` (`🧰️framework/…/📐️Canvas2dHost/🟦️.tsx`) reads: `kind` (never `type`),
/// `dataUrl` for the image, and an explicit `x`/`y`/`width`/`height` bounds rectangle per layer —
/// a record with no bounds falls through to the host's "print the label in the corner" branch and
/// draws nothing recognizable. The frame is laid out centred on the canvas origin (the host's own
/// camera transform centres the viewport there), and observation markers share that origin so their
/// document pixel coordinates land on the pixels they annotate. Keypoint circles / match lines /
/// track polylines are a documented gap: those live only in the reconstruction engine's in-progress
/// runtime scratch and are never distilled into durable document state.
fn frames_layers_json(scene: &RemodelingSnapshot, cursor: &RemodelingFrameCursor) -> String {
    let mut layers: Vec<JsonValue> = Vec::new();
    let Some(stream_id) = &cursor.stream_id else { return "[]".into() };
    let Some(stream) = scene.streams.iter().find(|stream| &stream.id == stream_id) else { return "[]".into() };
    let mut origin = (0.0_f64, 0.0_f64);
    if let Some(frame) = stream.frames.iter().find(|frame| frame.index == cursor.frame_index) {
        if let Some(asset) = crate::remodeling_asset(scene, &frame.asset_id) {
            let width = f64::from(asset.width);
            let height = f64::from(asset.height);
            origin = (-width / 2.0, -height / 2.0);
            layers.push(pack::json_object([
                ("kind".to_string(), JsonValue::from("image")),
                ("id".to_string(), JsonValue::from(frame.asset_id.as_str())),
                ("name".to_string(), JsonValue::from(frame.asset_id.as_str())),
                ("dataUrl".to_string(), JsonValue::from(format!("data:{};base64,{}", asset.mime, asset.data))),
                ("x".to_string(), JsonValue::from(origin.0)),
                ("y".to_string(), JsonValue::from(origin.1)),
                ("width".to_string(), JsonValue::from(width)),
                ("height".to_string(), JsonValue::from(height)),
            ]));
        }
    }
    for gcp in &scene.gcps {
        for observation in &gcp.observations {
            if &observation.stream_id == stream_id && observation.frame_index == cursor.frame_index {
                layers.push(pack::json_object([
                    ("kind".to_string(), JsonValue::from("circle")),
                    ("id".to_string(), JsonValue::from(format!("gcp-observation-{}-{}", gcp.id, observation.frame_index))),
                    ("name".to_string(), JsonValue::from(gcp.name.as_str())),
                    ("role".to_string(), JsonValue::from("handle")),
                    ("x".to_string(), JsonValue::from(origin.0 + f64::from(observation.pixel[0]) - GCP_MARKER_RADIUS_PX)),
                    ("y".to_string(), JsonValue::from(origin.1 + f64::from(observation.pixel[1]) - GCP_MARKER_RADIUS_PX)),
                    ("width".to_string(), JsonValue::from(GCP_MARKER_RADIUS_PX * 2.0)),
                    ("height".to_string(), JsonValue::from(GCP_MARKER_RADIUS_PX * 2.0)),
                ]));
            }
        }
    }
    pack::json_to_string(&pack::json_array(layers))
}

pub fn render(scene: &RemodelingSnapshot, config: &RemodelingConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let scene_2d = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: frames_layers_json(scene, &config.frame_cursor), snapshot: None };
    semio_framework_plugin::scene_surface(REMODELING_PLAY_SURFACE_FRAMES, ContractSurfaceKind::Canvas2d, &scene_2d)
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
