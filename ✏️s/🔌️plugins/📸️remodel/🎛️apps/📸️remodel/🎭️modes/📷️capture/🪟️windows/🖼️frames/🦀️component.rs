//! 🖼️ Remodel play app — the Frames window: a Canvas2d view of the currently cursored frame, with any
//! ground control point observations planted on it.

use crate::apps::remodel::config::{RemodelConfig, RemodelFrameCursor};
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const REMODEL_PLAY_WINDOW_FRAMES: &str = "remodel-frames";
pub const REMODEL_PLAY_BODY_FRAMES: &str = "remodel.play.frames";
const REMODEL_PLAY_SURFACE_FRAMES: &str = "remodel.play.frames";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: REMODEL_PLAY_WINDOW_FRAMES.into(),
        label: LocalizedLabel::native("Frames", "Frames"),
        body_key: REMODEL_PLAY_BODY_FRAMES.into(),
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
/// 🖼️ The cursored frame image (as a data URL, decoded straight from the stored `ImageAsset`) plus every
/// GCP observation planted on it, as point markers. Keypoint circles/match lines/track polylines are a
/// documented gap: those live only in the reconstruction engine's in-progress runtime scratch and are
/// never distilled into durable document state, so there is nothing to render for them.
fn frames_layers_json(scene: &RemodelSnapshot, cursor: &RemodelFrameCursor) -> String {
    let mut layers: Vec<Value> = Vec::new();
    let Some(stream_id) = &cursor.stream_id else { return "[]".into() };
    let Some(stream) = scene.streams.iter().find(|stream| &stream.id == stream_id) else { return "[]".into() };
    if let Some(frame) = stream.frames.iter().find(|frame| frame.index == cursor.frame_index) {
        if let Some(asset) = crate::artifacts::remodel::remodel_asset(&scene.assets, &frame.asset_id) {
            layers.push(json!({
                "type": "image",
                "assetId": frame.asset_id,
                "dataUrl": format!("data:{};base64,{}", asset.mime, asset.data),
                "width": asset.width,
                "height": asset.height,
            }));
        }
    }
    let mut points: Vec<Value> = Vec::new();
    for gcp in &scene.gcps {
        for observation in &gcp.observations {
            if &observation.stream_id == stream_id && observation.frame_index == cursor.frame_index {
                points.push(json!({ "x": observation.pixel[0], "y": observation.pixel[1], "label": gcp.name }));
            }
        }
    }
    if !points.is_empty() {
        layers.push(json!({ "type": "points", "id": "remodel-gcp-observations", "points": points }));
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

pub fn render(scene: &RemodelSnapshot, config: &RemodelConfig) -> UiNode {
    let scene_2d = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: frames_layers_json(scene, &config.frame_cursor) };
    build_canvas_2d_scene(REMODEL_PLAY_SURFACE_FRAMES, crate::apps::remodel::REMODEL_PLAY_APP_ID, scene_2d)
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, render as render_body};
    use crate::artifacts::remodel::default_remodel_scene;

    #[test]
    fn an_unset_frame_cursor_renders_no_layers() {
        assert_eq!(frames_layers_json(&default_remodel_scene(), &RemodelFrameCursor::default()), "[]");
    }

    #[test]
    fn renders_a_canvas_2d_surface() {
        let mut app = app();
        assert!(render_body(&mut app, REMODEL_PLAY_BODY_FRAMES).contains(REMODEL_PLAY_SURFACE_FRAMES));
    }
}
//#endregion 🧪️Tests
