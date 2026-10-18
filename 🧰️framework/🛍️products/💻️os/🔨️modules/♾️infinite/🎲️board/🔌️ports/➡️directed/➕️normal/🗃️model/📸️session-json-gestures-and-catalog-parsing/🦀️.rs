//! 📸️ Session JSON, gestures and catalog parsing of the board model: the brush preview snapshot from session JSON, gesture release events, drag records, pick targets, the draw level of detail label and the catalog parsers. See [`super`].

use super::*;

pub fn brush_preview_snapshot_from_session_json(node: &serde_json::Value, edge: &serde_json::Value, source_handle_id: &str) -> Option<BrushPreviewSnapshot> {
    let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
    let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
    let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
    let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
        Some("rectangle") => NodeShape::Rectangle,
        _ => NodeShape::Circle,
    };
    let (radius, width, height) = match shape {
        NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
        NodeShape::Rectangle => {
            let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
            let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
            (0.0, w, h)
        }
    };
    let target_handle_index = edge.get("targetHandleIndex").and_then(|v| v.as_u64()).map(|v| v as usize)?;
    let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
    if let Some(arr) = node.get("handles").and_then(|x| x.as_array()) {
        for row in arr {
            let ho = row.as_object()?;
            let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
            let angle = ho.get("angle").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
            let radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
            handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
        }
    }
    if handles.is_empty() {
        return None;
    }
    let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
    Some(BrushPreviewSnapshot { source_handle_id: source_handle_id.to_string(), node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, handles, target_handle_index, icon_kind })
}

/// 🏁️ The rows one released gesture publishes: its staged `select` (tagged with the gesture id when a
/// record follows) and the record itself — `(None, None)` for a release that changed nothing.
pub fn gesture_release_events(gesture: &GestureStage, record: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> [Option<Result<BoardOwnedEvent, BoardEventFault>>; 2] {
    let tag = record.is_some().then_some(gesture.id.as_str());
    [gesture.select.as_ref().map(|(ids, merge)| BoardOwnedEvent::staged_select(ids, merge.as_deref(), tag)), record]
}

/// 🎬️ The drag record of `start_positions` moved by `delta`, `None` for a release that never moved.
pub fn drag_gesture_record(gesture: &GestureStage, start_positions: &BTreeMap<String, (f64, f64)>, delta: Vec2, proximity_pair: Option<&(String, String)>) -> Option<Result<BoardOwnedEvent, BoardEventFault>> {
    if (delta.x == 0.0 && delta.y == 0.0) || start_positions.is_empty() {
        return None;
    }
    Some(BoardOwnedEvent::gesture(&gesture.id, start_positions.keys().map(String::as_str), BoardGestureMotion::Drag { dx: delta.x, dy: delta.y }, proximity_pair.map(std::slice::from_ref).unwrap_or_default()))
}

pub fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
    let mut v: Vec<_> = set.iter().cloned().collect();
    v.sort();
    v
}

pub fn push_pick_target(out: &mut Vec<BoardPickTargetJson>, domain: &str, id: String, generality: u32, label: Option<String>) {
    if out.iter().any(|row| row.domain == domain && row.id == id) {
        return;
    }
    out.push(BoardPickTargetJson { domain: domain.to_string(), id, generality, label });
}

pub fn board_draw_lod_label(lod: BoardDrawLod) -> &'static str {
    lod.label()
}

pub fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, NormalPortError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "general" => Ok(CompatSpecificity::General),
        "node" => Ok(CompatSpecificity::Node),
        "edge" => Ok(CompatSpecificity::Edge),
        "handle" | "vortex" => Ok(CompatSpecificity::Handle),
        "wire" => Ok(CompatSpecificity::Wire),
        _ => Err(NormalPortError::InvalidCompatSpecificity(raw.to_string())),
    }
}

pub fn reject_kind_catalog_row_legacy_label(row: &serde_json::Map<String, serde_json::Value>, slice: &'static str) -> Result<(), NormalPortError> {
    if row.contains_key("label") {
        return Err(NormalPortError::LegacyLabelField(slice));
    }
    Ok(())
}

pub fn parse_catalog_tip_slot(value: Option<&str>) -> Option<String> {
    let s = value?.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") {
        Some(String::new())
    } else {
        Some(s.to_string())
    }
}
