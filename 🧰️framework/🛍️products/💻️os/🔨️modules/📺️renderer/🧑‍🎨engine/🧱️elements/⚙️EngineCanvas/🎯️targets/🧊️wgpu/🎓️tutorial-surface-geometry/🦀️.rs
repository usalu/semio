//! 🎓️ Tutorial surface geometry.
//!
//! Tutorial geometry resolved from a surface: polyline points, graph node and wire anchors and the surface point
//! a tutorial step targets.

use super::engine_surface_registry::{EngineSurface, NodeGraphEngine, ENGINE_SURFACES};
use flow::dag::dag_world_to_screen;
use semio_framework_canvas as canvas;
use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq)]
struct TutorialSurfaceGeometry {
    point: [f32; 2],
    rect: Option<[f32; 4]>,
    polyline: Vec<[f32; 2]>,
    domain: Option<(f64, f64, bool)>,
}

pub fn tutorial_polyline_point(points: &[[f32; 2]], t: f64) -> Option<[f32; 2]> {
    let first = *points.first()?;
    if points.len() == 1 {
        return Some(first);
    }
    let lengths: Vec<f32> = points.windows(2).map(|pair| (pair[1][0] - pair[0][0]).hypot(pair[1][1] - pair[0][1])).collect();
    let total: f32 = lengths.iter().sum();
    if total <= f32::EPSILON {
        return Some(first);
    }
    let target = t.clamp(0.0, 1.0) as f32 * total;
    let mut consumed = 0.0;
    for (index, length) in lengths.iter().copied().enumerate() {
        if consumed + length >= target || index + 1 == lengths.len() {
            let segment_t = if length <= f32::EPSILON { 0.0 } else { (target - consumed) / length };
            return Some([points[index][0] + (points[index + 1][0] - points[index][0]) * segment_t, points[index][1] + (points[index + 1][1] - points[index][1]) * segment_t]);
        }
        consumed += length;
    }
    points.last().copied()
}

fn tutorial_geometry_from_json(raw: &str, requires_visible: bool) -> Option<TutorialSurfaceGeometry> {
    let value: Value = serde_json::from_str(raw).ok()?;
    if value.is_null() || requires_visible && value.get("visible").and_then(Value::as_bool) != Some(true) {
        return None;
    }
    let x = value.get("x")?.as_f64()? as f32;
    let y = value.get("y")?.as_f64()? as f32;
    let rect = value.get("rect").and_then(Value::as_array).filter(|values| values.len() == 4).and_then(|values| Some([values[0].as_f64()? as f32, values[1].as_f64()? as f32, values[2].as_f64()? as f32, values[3].as_f64()? as f32]));
    let polyline = value
        .get("polyline")
        .and_then(Value::as_array)
        .map(|points| {
            points
                .iter()
                .filter_map(|point| {
                    let pair = point.as_array()?;
                    Some([pair.first()?.as_f64()? as f32, pair.get(1)?.as_f64()? as f32])
                })
                .collect()
        })
        .unwrap_or_default();
    Some(TutorialSurfaceGeometry { point: [x, y], rect, polyline, domain: None })
}

pub fn tutorial_graph_world_to_screen(engine: &NodeGraphEngine, x: f64, y: f64) -> [f32; 2] {
    let point = match engine {
        NodeGraphEngine::Dag(host) => dag_world_to_screen(&host.dag, x, y),
        NodeGraphEngine::Flow(host) => dag_world_to_screen(&host.dag, x, y),
    };
    [point.0 as f32, point.1 as f32]
}

fn tutorial_graph_geometry(engine: &NodeGraphEngine, domain: &str, entity: &str) -> Option<TutorialSurfaceGeometry> {
    if domain == "slider" {
        let raw = match engine {
            NodeGraphEngine::Dag(host) => host.dag.slider_overlay_state_json().ok()?,
            NodeGraphEngine::Flow(host) => host.slider_overlay_state_json().ok()?,
        };
        let value: Value = serde_json::from_str(&raw).ok()?;
        let sliders = value.get("sliders")?.as_array()?;
        let slider = if entity == "*" { sliders.first() } else { sliders.iter().find(|slider| slider.get("widgetId").and_then(Value::as_str) == Some(entity)) }?;
        let x = slider.get("x")?.as_f64()?;
        let y = slider.get("y")?.as_f64()?;
        let point = tutorial_graph_world_to_screen(engine, x, y);
        let half_width = slider.get("w")?.as_f64()? * 0.5;
        let half_height = slider.get("h")?.as_f64()? * 0.5;
        let start = tutorial_graph_world_to_screen(engine, x - half_width, y - half_height);
        let end = tutorial_graph_world_to_screen(engine, x + half_width, y + half_height);
        return Some(TutorialSurfaceGeometry { point, rect: Some([start[0], start[1], end[0] - start[0], end[1] - start[1]]), polyline: Vec::new(), domain: Some((slider.get("min")?.as_f64()?, slider.get("max")?.as_f64()?, false)) });
    }
    let raw = match engine {
        NodeGraphEngine::Dag(host) => host.entity_screen_json(domain, entity),
        NodeGraphEngine::Flow(host) => host.entity_screen_json(domain, entity),
    };
    tutorial_geometry_from_json(&raw, true)
}

pub fn tutorial_surface_geometry(surface: &EngineSurface, domain: &str, entity: &str) -> Option<TutorialSurfaceGeometry> {
    if let Some(engine) = surface.node_graph.as_ref() {
        if let Some(geometry) = tutorial_graph_geometry(engine, domain, entity) {
            return Some(geometry);
        }
    }
    if let Some(host) = surface.map_host.as_ref() {
        if let Some(geometry) = tutorial_geometry_from_json(&host.feature_screen_json(domain, entity), false) {
            return Some(geometry);
        }
    }
    None
}

/// 👻️ Resolves typed tutorial points through the attached engine surface that owns their live camera and geometry.
pub fn resolve_tutorial_surface_point(host_id: &str, point: &semio_framework::onboarding::IntroductionPoint) -> Option<(f32, f32)> {
    use semio_framework::onboarding::IntroductionPoint as P;
    let point_surface_id = match point {
        P::Scene { id, .. } | P::Canvas { id, .. } | P::Entity { id, .. } | P::Curve { id, .. } | P::Domain { id, .. } => id,
        _ => return None,
    };
    ENGINE_SURFACES.with(|cell| {
        let surfaces = cell.borrow();
        let surface = surfaces.get(host_id)?;
        if point_surface_id.as_str() != surface.surface_id.as_ref()?.as_str() {
            return None;
        }
        let resolved = (|| match point {
            P::Canvas { x, y, .. } => {
                if let Some(engine) = surface.node_graph.as_ref() {
                    let point = tutorial_graph_world_to_screen(engine, *x, *y);
                    return Some((point[0], point[1]));
                }
                if let Some(host) = surface.map_host.as_ref() {
                    let point = host.world_to_screen_point(*x, *y);
                    return Some((point.0 as f32, point.1 as f32));
                }
                if let Some(host) = surface.board_host.as_ref() {
                    let point = host.world_to_screen(canvas::Point::new(*x, *y));
                    return Some((point.x as f32, point.y as f32));
                }
                if let Some(host) = surface.editor.as_ref() {
                    let value: Value = serde_json::from_str(&host.world_to_screen_json(*x, *y)).ok()?;
                    return Some((value.get("x")?.as_f64()? as f32, value.get("y")?.as_f64()? as f32));
                }
                if let Some(host) = surface.raster_host.as_ref() {
                    let point = host.world_to_screen_point(*x, *y);
                    return Some((point.0 as f32, point.1 as f32));
                }
                None
            }
            P::Entity { domain, entity, offset, .. } => {
                let geometry = tutorial_surface_geometry(surface, domain, entity)?;
                let Some([x, y, width, height]) = geometry.rect else { return Some((geometry.point[0], geometry.point[1])) };
                let [offset_x, offset_y] = offset.unwrap_or([0.5, 0.5]);
                Some((x + width * offset_x as f32, y + height * offset_y as f32))
            }
            P::Curve { domain, entity, t, .. } => {
                let geometry = tutorial_surface_geometry(surface, domain, entity)?;
                tutorial_polyline_point(&geometry.polyline, *t).map(|point| (point[0], point[1]))
            }
            P::Domain { domain, entity, value, .. } => {
                let geometry = tutorial_surface_geometry(surface, domain, entity)?;
                let [x, y, width, height] = geometry.rect?;
                let (min, max, vertical) = geometry.domain?;
                let t = if max == min { 0.0 } else { ((*value).clamp(min, max) - min) / (max - min) } as f32;
                if vertical {
                    Some((x + width * 0.5, y + (1.0 - t) * height))
                } else {
                    Some((x + t * width, y + height * 0.5))
                }
            }
            P::Scene { .. } => None,
            _ => None,
        })();
        resolved.filter(|(x, y)| surface.node_graph.is_none() || x.is_finite() && y.is_finite() && *x >= 0.0 && *y >= 0.0 && *x <= surface.width as f32 && *y <= surface.height as f32)
    })
}
