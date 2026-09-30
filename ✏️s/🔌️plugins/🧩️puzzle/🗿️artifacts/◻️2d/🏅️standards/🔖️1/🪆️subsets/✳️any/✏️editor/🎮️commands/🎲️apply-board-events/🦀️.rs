//! 🎲️ `apply-board-events` command.

use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{artifact, inspection};
use crate::editor::puzzle2d::{apply_brush_place_payload, delete_selection_from_host_snapshot, puzzle2d_push_target_region, puzzle2d_relocate_target_region, puzzle2d_selection_write, set_runtime_camera, Puzzle2dActionCtx, Puzzle2dScene, Puzzle2dSelectionRecord};
use semio_framework::kernel::UiDirtyScope;
use serde_json::{json, Value};

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
            // 🎬️ A finished drag or rotate arrives as ONE record: positions (and handle angles) change and a drop
            // may land new edges, so the panes, the inspector and the layers tree repaint.
            "gesture" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
            }
            // 🎯️ A painted region adds an outliner row; a moved or resized one only changes geometry,
            // so it repaints the panes and the inspector without churning the layers tree.
            "regionCreate" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
            }
            "regionResize" => {
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
            // 🖱️ Engine-local hover paints itself; the guest keeps no hover state to re-render for.
            "hover" | "preselectCancel" => {}
            _ => recognized_all = false,
        }
    }
    if !recognized_all {
        return UiDirtyScope::Full;
    }
    let mut panel_bodies = Vec::new();
    if panel_layers {
        panel_bodies.push(artifact::PUZZLE2D_PLAY_BODY_LAYERS.to_string());
    }
    if panel_properties {
        panel_bodies.push(inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string());
    }
    UiDirtyScope::Partial { window_bodies: if window_bodies { panes } else { Vec::new() }, panel_bodies, utilities: false, tools: false, engagements, measures, labels: false }
}
/// 🎲️ Folds one engine event batch into the scene and answers its gesture records. A `gesture` row never
/// touches the scene: it is a parametric input the select tool yields as ONE transaction (see
/// [`apply_board_events`]); every other document row still folds into the scratch fixture.
pub fn apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dScene) -> Vec<Puzzle2dSelectionRecord> {
    let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
        return Vec::new();
    };
    let mut records = Vec::new();
    for event in events {
        let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
            continue;
        };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
        match name {
            "camera" => {
                set_runtime_camera(&mut envelope.runtime, &payload);
            }
            // 🕹️ Selection is framework-owned: `apply_board_events` turns the engine's `select` rows into
            // `Emit::interaction_writes` (see [`board_selection_write`]); nothing to fold into the scene here.
            "select" => {}
            "gesture" => records.extend(Puzzle2dSelectionRecord::from_gesture(&payload)),
            // 🎯️ The board's area brush released: ONE rectangle, already grid-snapped by the engine,
            // through the SAME document push `addTargetRegion` ends in — so a canvas paint and a
            // dispatched verb mint the identical row, as a single history edit.
            "regionCreate" => {
                let read = |key: &str| payload.get(key).and_then(Value::as_f64).filter(|value| value.is_finite());
                let (Some(x), Some(y), Some(width), Some(height)) = (read("x"), read("y"), read("width"), read("height")) else {
                    continue;
                };
                if width <= 0.0 || height <= 0.0 {
                    continue;
                }
                puzzle2d_push_target_region(&mut envelope.fixture, x, y, width, height);
            }
            // 📐️ A region grip drag commits through `relocateTargetRegion`'s own reducer, which refuses a locked
            // region — the engine refuses the grab too, so a locked rectangle can never resize by either route. A
            // region BODY drag is a `drag` gesture record like any node drag.
            "regionResize" => {
                let Some(id) = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()) else {
                    continue;
                };
                let read = |key: &str| payload.get(key).and_then(Value::as_f64).filter(|value| value.is_finite());
                let (Some(x), Some(y), Some(width), Some(height)) = (read("x"), read("y"), read("width"), read("height")) else {
                    continue;
                };
                puzzle2d_relocate_target_region(&mut envelope.fixture, id, &json!({ "position": [x, y], "size": [width, height] }));
            }
            "brushPlace" => {
                apply_brush_place_payload(&mut envelope.fixture, &payload);
            }
            "edgeCreate" => {
                if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                    edges.push(payload);
                }
            }
            // 🗑️ One delete path for every granularity: the id may name a node, a handle, an edge or
            // a target region, so both collections are asked and whichever holds it drops it.
            "nodeDelete" => {
                if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                    delete_selection_from_host_snapshot(&mut envelope.fixture, &[id.to_string()]);
                    crate::editor::puzzle2d::delete_target_regions_from_fixture(&mut envelope.fixture, &[id.to_string()]);
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
                    envelope.runtime.brush_candidates = candidates.iter().map(dsl::DslValue::from).collect();
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
    records
}
/// 🐢️ `UiDirtyScope.windowBodies`/`.panelBodies` are matched against `AppDefinition.windowKinds[].bodyKey`
/// on the shell side (`buildUiRefreshRequest`'s `uiRefreshWantsWindow`), so these must be the body-key
/// constants (`puzzle2d.play.overview`, …) — *not* the pane/kind-id constants (`PUZZLE2D_PANES`,
/// `2d-overview`, …), which are a different id space used to key utilities/engagements/measures.
pub const PUZZLE2D_WINDOW_BODY_KEYS: [&str; 3] = [overview::BODY_KEY, detail::BODY_KEY, selection::BODY_KEY];

/// 🕹️ The LAST `select` row of a batch is the engine's whole selection set (`ids`, replace semantics),
/// so it becomes one framework selection write — later rows supersede earlier ones inside a batch.
fn board_selection_write(events: &[Value], fixture: &Value) -> Option<semio_framework_plugin::InteractionWrite> {
    let ids: Vec<String> = events
        .iter()
        .rev()
        .find(|event| event.get("name").and_then(Value::as_str) == Some("select"))?
        .get("payload")
        .and_then(|payload| payload.get("ids"))
        .and_then(Value::as_array)
        .map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default();
    Some(puzzle2d_selection_write(fixture, &ids))
}

/// 🎲️ One board flush: folds its rows into the scene, writes its last `select` as the framework selection, and
/// commits its gesture records through the select tool — a drag or rotate is ONE transaction whose parametric
/// leaf (and the connections its drop lands) is what history keeps. A record whose every target is locked is
/// refused at the tool with one sentence; the selection write of the same batch still lands.
pub fn apply_board_events(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) else {
        return;
    };
    let events = serde_json::from_str::<Vec<Value>>(events_json).ok();
    let scope = events.as_deref().map_or(UiDirtyScope::Full, puzzle2d_board_events_scope);
    *ctx.ui_scope = scope.clone();
    let records = apply_board_events_from_json(events_json, ctx.scene);
    if let Some(write) = events.as_deref().and_then(|events| board_selection_write(events, &ctx.scene.fixture)) {
        ctx.interaction_writes.push(write);
    }
    ctx.commit_selection(overview::utilities::select::UTILITY_ID, records);
    if !ctx.interaction_writes.is_empty() && matches!(*ctx.ui_scope, UiDirtyScope::None) {
        *ctx.ui_scope = scope;
    }
}
