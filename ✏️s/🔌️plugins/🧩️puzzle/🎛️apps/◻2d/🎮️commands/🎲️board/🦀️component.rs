//! 🎲️ Puzzle 2d play app commands — `applyBoardEvents`: the single entry point through which the
//! client canvas replays a batch of board events (camera, select, drags, brush placements, edge and
//! node create/delete) back into the document and the view config.

use crate::apps::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::apps::puzzle2d::panels::{document, inspection};
use crate::apps::puzzle2d::{apply_brush_place_payload, delete_selection_from_fixture, patch_inspector_nodes, set_runtime_camera, Puzzle2dActionCtx, Puzzle2dScene};
use semio_framework::kernel::UiDirtyScope;
use serde_json::{json, Value};

//#region 🔖️Constants
/// 🐢️ `UiDirtyScope.windowBodies`/`.panelBodies` are matched against `AppDefinition.windowKinds[].bodyKey`
/// on the shell side (`buildUiRefreshRequest`'s `uiRefreshWantsWindow`), so these must be the body-key
/// constants (`puzzle2d.play.overview`, …) — *not* the pane/kind-id constants (`PUZZLE2D_PANES`,
/// `2d-overview`, …), which are a different id space used to key utilities/engagements/measures.
pub const PUZZLE2D_WINDOW_BODY_KEYS: [&str; 3] = [overview::BODY_KEY, detail::BODY_KEY, selection::BODY_KEY];
//#endregion 🔖️Constants

//#region 🔖️Scope
/// 🐢️ Classifies a batch of board events into the narrowest `UiDirtyScope` that covers all of them —
/// `applyBoardEvents` fires on every select/drag/zoom, so getting this right is most of the
/// perf-round-3 win. Unrecognized/empty event batches fall back to `Full` (safe default).
fn puzzle2d_board_events_scope(events: &[Value]) -> UiDirtyScope {
    if events.is_empty() {
        return UiDirtyScope::None;
    }
    let panes: Vec<String> = PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect();
    let mut window_bodies = false;
    let mut panel_layers = false;
    let mut panel_properties = false;
    let mut engagements = false;
    let mut measures = false;
    let mut recognized_all = true;
    for event in events {
        let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
            recognized_all = false;
            continue;
        };
        match name {
            "camera" => {
                window_bodies = true;
            }
            "select" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
                engagements = true;
            }
            "nodeMove" | "nodeDragEnd" => {
                window_bodies = true;
                panel_properties = true;
            }
            "brushPlace" | "edgeCreate" | "edgeDelete" | "nodeDelete" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
                engagements = true;
                measures = true;
            }
            "brushCandidates" => {
                window_bodies = true;
                engagements = true;
            }
            _ => recognized_all = false,
        }
    }
    if !recognized_all {
        return UiDirtyScope::Full;
    }
    let mut panel_bodies = Vec::new();
    if panel_layers {
        panel_bodies.push(document::PUZZLE2D_PLAY_BODY_LAYERS.to_string());
    }
    if panel_properties {
        panel_bodies.push(inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string());
    }
    UiDirtyScope::Partial { window_bodies: if window_bodies { panes } else { Vec::new() }, panel_bodies, utilities: false, tools: false, engagements, measures, labels: false }
}
//#endregion 🔖️Scope

//#region 🔖️Replay
pub fn apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dScene) {
    let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
        return;
    };
    for event in events {
        let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
            continue;
        };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
        match name {
            "camera" => {
                set_runtime_camera(&mut envelope.runtime, &payload);
            }
            "select" => {
                if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value(value.clone()).ok()) {
                    envelope.runtime.selected_ids = ids;
                }
            }
            "nodeDragEnd" => {
                if let Some(moves) = payload.get("moves").and_then(|value| value.as_array()) {
                    for entry in moves {
                        let Some(id) = entry.get("id").and_then(|value| value.as_str()) else {
                            continue;
                        };
                        if let Some(x) = entry.get("x").and_then(|value| value.as_f64()) {
                            patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", Some(&json!(x)), None);
                        }
                        if let Some(y) = entry.get("y").and_then(|value| value.as_f64()) {
                            patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", Some(&json!(y)), None);
                        }
                    }
                }
            }
            "nodeMove" => {
                let Some(id) = payload.get("id").and_then(|value| value.as_str()) else {
                    continue;
                };
                if let Some(x) = payload.get("x").and_then(|value| value.as_f64()) {
                    patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", Some(&json!(x)), None);
                }
                if let Some(y) = payload.get("y").and_then(|value| value.as_f64()) {
                    patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", Some(&json!(y)), None);
                }
            }
            "brushPlace" => {
                apply_brush_place_payload(&mut envelope.fixture, &payload);
            }
            "edgeCreate" => {
                if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                    edges.push(payload);
                }
            }
            "nodeDelete" => {
                if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                    envelope.runtime.selected_ids = vec![id.to_string()];
                    delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                    envelope.runtime.selected_ids.clear();
                }
            }
            "edgeDelete" => {
                if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                    if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                        edges.retain(|edge| edge.get("id").and_then(|value| value.as_str()) != Some(id));
                    }
                }
            }
            "brushCandidates" => {
                if let Some(candidates) = payload.get("candidates").and_then(|value| value.as_array()) {
                    envelope.runtime.brush_candidates = candidates.clone();
                }
                if let Some(source) = payload.get("sourceHandleId").and_then(|value| value.as_str()) {
                    envelope.runtime.brush_candidate_source_handle_id = source.to_string();
                }
                if let Some(index) = payload.get("index").and_then(|value| value.as_u64()) {
                    envelope.runtime.brush_candidate_index = index as usize;
                }
            }
            _ => {}
        }
    }
}

pub fn apply_board_events(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) else {
        return;
    };
    *ctx.ui_scope = serde_json::from_str::<Vec<Value>>(events_json).map_or(UiDirtyScope::Full, |events| puzzle2d_board_events_scope(&events));
    apply_board_events_from_json(events_json, ctx.scene);
    // 🪞️ `apply_host_events` (in the epilogue) trusts `host.selection` as the post-action source of
    // truth and overwrites `runtime.selected_ids` with it — mirror the new selection into the host now
    // (as every other selection-setting arm already does) or the just-applied `select`/
    // `brushCandidates` selection is silently reverted.
    ctx.host.borrow_mut().set_selection_ids(&ctx.scene.runtime.selected_ids);
}
//#endregion 🔖️Replay
