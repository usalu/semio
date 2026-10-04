//! 🎲️ `apply-board-events` command.

use crate::editor::puzzle5d::commands::add_brush_part::puzzle5d_place_brush_part;
use crate::editor::puzzle5d::config::Puzzle5dCamera2d;
use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::{Puzzle5dSelectionMotion, Puzzle5dSelectionRecord};
use crate::editor::puzzle5d::{remove_parts, Puzzle5dActionCtx, Puzzle5dFastener, Puzzle5dFreshIds};
use semio_framework_pack_json::{parse, Value};

fn payload_str<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

/// 🎲️ Replays a board's event batch (`eventsJson`) in order: camera, brush placements through the shared brush
/// placement, edge creates and deletes, node deletes. Selection events are framework-owned. Every drag gesture
/// record is one `drag-selection2d` leaf of ONE transform-tool transaction over the committed document.
pub fn apply_board_events(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(events) = args.and_then(|value| value.get("eventsJson")).and_then(Value::as_str).and_then(|text| parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).and_then(|value| value.as_array().cloned()) else {
        return;
    };
    let mut drags = Vec::new();
    for event in events {
        let Some(name) = payload_str(&event, "name") else { continue };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
        match name {
            "camera" => {
                if let Ok(camera) = <Puzzle5dCamera2d as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&payload)) {
                    ctx.scene.runtime.camera2d = camera;
                }
            }
            // 🎬️ A board drag is ONE `drag` gesture record: its targets (each once) move by its offset, flat pose only.
            "gesture" if payload_str(&payload, "kind") == Some("drag") => {
                let (Some(dx), Some(dy)) = (payload.get("dx").and_then(Value::as_f64), payload.get("dy").and_then(Value::as_f64)) else { continue };
                let targets = payload.get("targets").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                drags.push(Puzzle5dSelectionRecord::new(targets, Puzzle5dSelectionMotion::Board { dx, dy }));
            }
            "brushPlace" => {
                let part_kind = payload_str(&payload, "nodeKind").unwrap_or("Part").to_string();
                let at = [payload.get("x").and_then(Value::as_f64), payload.get("y").and_then(Value::as_f64)];
                puzzle5d_place_brush_part(ctx, &part_kind, payload_str(&payload, "sourceHandleId"), at, payload_str(&payload, "nodeId"), payload_str(&payload, "edgeId"));
            }
            "edgeCreate" => {
                let source = payload_str(&payload, "source").unwrap_or("").to_string();
                let target = payload_str(&payload, "target").unwrap_or("").to_string();
                let document = &mut ctx.scene.document;
                if !source.is_empty() && !target.is_empty() && !document.fasteners.iter().any(|entry| entry.source == source && entry.target == target || entry.source == target && entry.target == source) {
                    let id = payload_str(&payload, "id").map_or_else(|| Puzzle5dFreshIds::from_document(document).next_fastener(), str::to_string);
                    let fastener_kind = payload_str(&payload, "edgeKind").map(str::to_string);
                    document.fasteners.push(Puzzle5dFastener { id, source, target, fastener_kind, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
                }
            }
            "nodeDelete" => {
                if let Some(id) = payload_str(&payload, "id") {
                    remove_parts(&mut ctx.scene.document, &[id.to_string()]);
                }
            }
            "edgeDelete" => {
                if let Some(id) = payload_str(&payload, "id") {
                    ctx.scene.document.fasteners.retain(|fastener| fastener.id != id);
                }
            }
            _ => {}
        }
    }
    let base = ctx.snapshot.typed_arc();
    if drags.iter().any(|record| record.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id && part.part_2d.locked == Some(true)))) {
        ctx.notice(|labels| labels.selection_locked.as_str());
    }
    if drags.iter().any(|record| record.applies_to(&base)) {
        ctx.commit_selection("applyBoardEvents", drags);
    }
}
