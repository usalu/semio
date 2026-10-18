//! 🖱️ Hover, rect and pick targets of the DAG graph: the minimap pointer hover, world to screen points, the hovered node, entity screen rects and the pick targets at a screen point. See [`super`].

use super::*;

pub fn sync_minimap_pointer_hover(viewport: &mut DagViewportState, graph: &mut DagGraphState, world_x: f64, world_y: f64) {
    if !matches!(draw_lod_for_frame(viewport, graph), DagDrawLod::Minimap) {
        return;
    }
    if !matches!(graph.engine.interaction, InteractionMode::Idle) {
        return;
    }
    graph.engine.hover = graph.resolve_minimap_hover_node_id(world_x, world_y);
}

pub fn world_to_screen_point(viewport_state: &DagViewportState, graph: &DagGraphState, x: f64, y: f64) -> (f64, f64) {
    use canvas::camera::{world_to_screen, Camera, Viewport};
    let camera = Camera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width, height: viewport_state.height, dpr: viewport_state.dpr };
    let point = world_to_screen(&camera, &viewport, canvas::Point::new(x, y));
    (point.x, point.y)
}

/// 🖱️ Hovered snapshot widget id for node body hover, or parent widget when a channel handle is hovered at detail LOD.
pub fn hovered_node_id(viewport: &DagViewportState, graph: &DagGraphState) -> Option<String> {
    hovered_node_id_ref(viewport, graph).map(str::to_owned)
}

pub fn hovered_node_id_ref<'a>(viewport: &'a DagViewportState, graph: &'a DagGraphState) -> Option<&'a str> {
    let hover = graph.engine.hover?;
    if graph.node_id_map.contains_key(&hover) {
        return graph.widget_id_for_node_id_ref(hover);
    }
    if draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        if let Some(handle) = graph.engine.handles.get(&hover) {
            return graph.widget_id_for_node_id_ref(handle.node_id);
        }
    }
    None
}

/// 🗺️ The screen rect [`Self::entity_screen_json`] reports for one entity, or `None` when it is
/// off screen — the typed read the census needs before it can scan inside it.
pub fn entity_screen_rect(viewport: &DagViewportState, graph: &DagGraphState, domain: &str, id: &str) -> Option<[f64; 4]> {
    let value = semio_framework_pack_json::parse(&entity_screen_json(viewport, graph, domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    if value.get("visible").and_then(Value::as_bool) != Some(true) {
        return None;
    }
    let rect = value.get("rect")?.as_array()?;
    if rect.len() < 4 {
        return None;
    }
    let read = |index: usize| rect.get(index).and_then(Value::as_f64);
    Some([read(0)?, read(1)?, read(2)?, read(3)?])
}

/// 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
pub fn dag_pick_targets_at_screen_json(viewport: &DagViewportState, graph: &DagGraphState, sx: f64, sy: f64) -> String {
    #[derive(ToValue, FromValue)]
    struct Row {
        domain: String,
        id: String,
        generality: u32,
        #[value(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }
    let world = screen_to_world_point(viewport, graph, sx, sy);
    let targets = graph.engine.hit_test_pick_targets(world);
    let rows: Vec<Row> = targets
        .into_iter()
        .filter_map(|target| {
            let row = match target.domain.as_str() {
                "node" => graph.widget_id_for_node_id(target.id).map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                "edge" => graph.edge_id_map.get(&target.id).cloned().map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                "handle" => graph.decode_channel_ref(target.id).map(|channel| Row {
                    domain: "handle".into(),
                    id: format!("{}@{}", channel.widget_id, channel.port),
                    generality: target.generality,
                    label: Some(format!("{} · {}", channel.widget_id, channel.port)),
                }),
                _ => None,
            };
            row
        })
        .collect();
    semio_framework_pack_json::to_json_string(&rows)
}
