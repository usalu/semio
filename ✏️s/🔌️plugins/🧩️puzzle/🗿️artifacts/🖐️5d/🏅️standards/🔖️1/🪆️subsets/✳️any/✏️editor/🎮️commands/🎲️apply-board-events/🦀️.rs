//! 🎲️ `apply-board-events` command.

use crate::editor::puzzle5d::commands::add_brush_part::puzzle5d_place_brush_part;
use crate::editor::puzzle5d::config::Puzzle5dCamera2d;
use crate::editor::puzzle5d::{remove_parts, set_part_2d_position, Puzzle5dActionCtx, Puzzle5dFastener, Puzzle5dFreshIds};
use dsl::os_pack::json::{parse, Value};

fn payload_str<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

/// 🎲️ Replays a board's event batch (`eventsJson`) in order: camera, node moves, brush placements through the
/// shared brush placement, edge creates and deletes, node deletes. Selection events are framework-owned.
pub fn apply_board_events(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(events) = args.and_then(|value| value.get("eventsJson")).and_then(Value::as_str).and_then(|text| parse(text).ok()).and_then(|value| value.as_array().cloned()) else {
        return;
    };
    for event in events {
        let Some(name) = payload_str(&event, "name") else { continue };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
        match name {
            "camera" => {
                if let Ok(camera) = <Puzzle5dCamera2d as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&payload)) {
                    ctx.scene.runtime.camera2d = camera;
                }
            }
            "nodeDragEnd" => {
                for entry in payload.get("moves").and_then(Value::as_array).into_iter().flatten() {
                    if let Some(id) = payload_str(entry, "id") {
                        set_part_2d_position(&mut ctx.scene.document, id, entry.get("x").and_then(Value::as_f64), entry.get("y").and_then(Value::as_f64));
                    }
                }
            }
            "nodeMove" => {
                if let Some(id) = payload_str(&payload, "id") {
                    set_part_2d_position(&mut ctx.scene.document, id, payload.get("x").and_then(Value::as_f64), payload.get("y").and_then(Value::as_f64));
                }
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
}
