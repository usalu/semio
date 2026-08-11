//! 📥️ `dxf` (r12) → `s.stdio.semio/v1/drawing` — real, honest ENTITY-to-PATH translation (dxf is
//! entity-shaped, `drawing` is path-shaped — see the master plan's own framing). Each real DXF
//! entity kind becomes an EXACT `DrawNode::Path` (circle/arc via true elliptical arcs, not
//! polygon approximation), grouped one `DrawLayer` per real on-disk DXF layer NAME (declared in
//! `TABLES/LAYER` or not — this leaf buckets by whatever `layer` string an entity actually
//! carries, so undeclared-but-used layers still keep their geometry).
//!
//! Honest lossy points (documented, never fabricated):
//! - `z` is dropped (drawing is 2D-only, matching the cad↔dxf bridge's own dimensionality note).
//! - `BLOCKS`/`INSERT` (block instancing) has no equivalent in `DrawNode`'s flat recursive scene
//!   graph (no instance-reference node kind) — both are dropped. Use the cad↔dxf bridge instead
//!   when block/insert fidelity matters (this is an architectural choice, not an oversight — cad
//!   is entity+block shaped, drawing is scene-graph shaped, per the master plan).
//! - `TEXT`'s DXF `height`/rotation have no field on `DrawNode::Text` (value/at/style only) — both
//!   dropped.
//! - `SOLID`'s 4-point quad becomes a closed 4-line `Path` — exact, not approximated.
//! - Every OTHER unmodeled `DxfEntity::Other` kind (3DFACE, POINT, DIMENSION, …) is dropped — no
//!   raw-retention path node kind exists on `DrawNode`.

use crate::artifacts::dxf::{DxfSnapshot, schema::snapshot::DxfEntity};
use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

//#region 🔖️Geometry
fn ellipse_path(cx: f64, cy: f64, r: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x: cx + r, y: cy } },
        PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx - r, y: cy } },
        PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx + r, y: cy } },
        PathSegment::Close,
    ]
}

fn arc_path(cx: f64, cy: f64, r: f64, start_deg: f64, end_deg: f64) -> Vec<PathSegment> {
    let (sr, er) = (start_deg.to_radians(), end_deg.to_radians());
    let start = SemioPoint2 { x: cx + r * sr.cos(), y: cy + r * sr.sin() };
    let end = SemioPoint2 { x: cx + r * er.cos(), y: cy + r * er.sin() };
    let sweep_deg = (end_deg - start_deg).rem_euclid(360.0);
    let large_arc = sweep_deg > 180.0;
    vec![PathSegment::MoveTo { to: start }, PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc, sweep: true, to: end }]
}
//#endregion 🔖️Geometry

//#region 🔖️EntityMap
fn draw_node_from_entity(e: &DxfEntity) -> Option<DrawNode> {
    match e {
        DxfEntity::Line { start, end, .. } => Some(DrawNode::Path {
            segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: start[0], y: start[1] } }, PathSegment::LineTo { to: SemioPoint2 { x: end[0], y: end[1] } }],
            style: None,
        }),
        DxfEntity::Circle { center, radius, .. } => Some(DrawNode::Path { segments: ellipse_path(center[0], center[1], *radius), style: None }),
        DxfEntity::Arc { center, radius, start_angle, end_angle, .. } => Some(DrawNode::Path { segments: arc_path(center[0], center[1], *radius, *start_angle, *end_angle), style: None }),
        DxfEntity::Polyline { vertices, closed, .. } => {
            let mut segments: Vec<PathSegment> = vertices.iter().enumerate().map(|(i, v)| {
                let to = SemioPoint2 { x: v.x, y: v.y };
                if i == 0 { PathSegment::MoveTo { to } } else { PathSegment::LineTo { to } }
            }).collect();
            if *closed { segments.push(PathSegment::Close); }
            Some(DrawNode::Path { segments, style: None })
        }
        DxfEntity::Text { position, value, .. } => Some(DrawNode::Text { value: value.clone(), at: SemioPoint2 { x: position[0], y: position[1] }, style: None }),
        DxfEntity::Solid { points, .. } => Some(DrawNode::Path {
            segments: vec![
                PathSegment::MoveTo { to: SemioPoint2 { x: points[0][0], y: points[0][1] } },
                PathSegment::LineTo { to: SemioPoint2 { x: points[1][0], y: points[1][1] } },
                PathSegment::LineTo { to: SemioPoint2 { x: points[2][0], y: points[2][1] } },
                PathSegment::LineTo { to: SemioPoint2 { x: points[3][0], y: points[3][1] } },
                PathSegment::Close,
            ],
            style: None,
        }),
        DxfEntity::Insert { .. } | DxfEntity::Other { .. } => None,
    }
}

fn entity_layer(e: &DxfEntity) -> Option<&str> {
    match e {
        DxfEntity::Line { layer, .. } | DxfEntity::Circle { layer, .. } | DxfEntity::Arc { layer, .. } | DxfEntity::Polyline { layer, .. } | DxfEntity::Text { layer, .. } | DxfEntity::Solid { layer, .. } | DxfEntity::Insert { layer, .. } => Some(layer.as_str()),
        DxfEntity::Other { .. } => None,
    }
}
//#endregion 🔖️EntityMap

//#region 🔖️Deserializer
pub struct SemioDrawingFromDxf;

impl ArtifactDeserializer for SemioDrawingFromDxf {
    type From = DxfSnapshot;
    type Into = SemioDrawingSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut order: Vec<String> = Vec::new();
        let mut buckets: std::collections::HashMap<String, Vec<DrawNode>> = std::collections::HashMap::new();
        for e in &from.entities {
            let Some(node) = draw_node_from_entity(e) else { continue };
            let layer = entity_layer(e).unwrap_or("0").to_string();
            if !buckets.contains_key(&layer) {
                order.push(layer.clone());
            }
            buckets.entry(layer).or_default().push(node);
        }
        let layers = order
            .into_iter()
            .map(|name| {
                let children = buckets.remove(&name).unwrap_or_default();
                DrawLayer { id: name.clone(), name, visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }
            })
            .collect();
        Ok(SemioDrawingSnapshot { schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(), canvas: DrawCanvas::default(), styles: Vec::new(), layers })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dxf() -> DxfSnapshot {
        DxfSnapshot {
            entities: vec![
                DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Circle { center: [2.0, 2.0, 0.0], radius: 1.0, layer: "walls".into(), unknown_group_codes: vec![] },
                DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![] },
            ],
            ..DxfSnapshot::default()
        }
    }

    #[test]
    fn buckets_entities_by_layer_and_drops_unmodeled() {
        let drawing = SemioDrawingFromDxf::deserialize(&sample_dxf()).expect("deserialize");
        assert_eq!(drawing.layers.len(), 2);
        assert_eq!(drawing.layers[0].id, "0");
        assert_eq!(drawing.layers[1].id, "walls");
        match &drawing.layers[0].root {
            DrawNode::Group { children, .. } => {
                assert_eq!(children.len(), 1);
                assert!(matches!(children[0], DrawNode::Path { .. }));
            }
            other => panic!("expected Group, got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
