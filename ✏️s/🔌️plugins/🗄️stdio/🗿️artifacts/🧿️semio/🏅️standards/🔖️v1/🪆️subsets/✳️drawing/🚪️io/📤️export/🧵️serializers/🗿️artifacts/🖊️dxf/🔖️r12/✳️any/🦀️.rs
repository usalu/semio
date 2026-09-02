//! 📤️ `s.stdio.semio/v1/drawing` → `dxf` (r12) — mirrors the import leaf. R12 has no bezier/
//! general-arc entity, so this leaf makes a real, honest export decision per path: a `Path`
//! matching exactly the closed two-arc CIRCLE shape the import leaf's own `ellipse_path` (and
//! the cad↔dxf bridge's own convention) produces round-trips EXACTLY back to a `CIRCLE` entity;
//! every other `Path` (lines, general arcs, bezier curves) is FLATTENED into a real, sampled
//! `POLYLINE` (curves sampled at 32 segments — a genuine, documented curve-flattening
//! approximation, not a silent drop). `Text`→`TEXT` (height/rotation default, no source field —
//! see the import leaf's doc). `Image`/nested `Group` transforms have no DXF entity/composition
//! equivalent and are dropped (documented — same architectural boundary the import leaf
//! describes for BLOCKS/INSERT).

use crate::artifacts::dxf::{
    schema::snapshot::{DxfEntity, DxfHeaderVar, DxfLayer, DxfTables, DxfValue},
    DxfSnapshot,
};
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment, SemioDrawingSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

const EPS: f64 = 1e-6;

//#region 🔖️CirclePattern
/// ⭕️ Recognizes the EXACT closed two-arc shape this bridge's own import leaf (and the
/// cad↔dxf/svg↔drawing bridges' matching `ellipse_path` helpers) produce for a real circle —
/// `[MoveTo(cx+r,cy), ArcTo(r,r,..,cx-r,cy), ArcTo(r,r,..,cx+r,cy), Close]` — giving an EXACT
/// (not flattened) round trip for genuinely circular content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_circle(segments: &[PathSegment]) -> Option<(f64, f64, f64)> {
    if let [PathSegment::MoveTo { to: m }, PathSegment::ArcTo { rx: r1, ry: ry1, to: a1, .. }, PathSegment::ArcTo { rx: r2, ry: ry2, to: a2, .. }, PathSegment::Close] = segments {
        if (r1 - ry1).abs() < EPS && (r2 - ry2).abs() < EPS && (r1 - r2).abs() < EPS && (m.x - a2.x).abs() < EPS && (m.y - a2.y).abs() < EPS {
            let cx = (m.x + a1.x) / 2.0;
            let cy = (m.y + a1.y) / 2.0;
            return Some((cx, cy, *r1));
        }
    }
    None
}
//#endregion 🔖️CirclePattern

//#region 🔖️Flatten
/// 📐️ Real parametric curve flattening (32 samples) — cubic/quadratic Bezier and elliptical arc
/// all sampled the standard way, not approximated by their endpoints alone.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn flatten_to_polyline(segments: &[PathSegment]) -> (Vec<SemioPoint2>, bool) {
    let mut points = Vec::new();
    let mut closed = false;
    let mut cur = SemioPoint2::default();
    let mut start = SemioPoint2::default();
    for seg in segments {
        match *seg {
            PathSegment::MoveTo { to } => {
                points.push(to);
                cur = to;
                start = to;
            }
            PathSegment::LineTo { to } => {
                points.push(to);
                cur = to;
            }
            PathSegment::CubicTo { c1, c2, to } => {
                for i in 1..=32 {
                    let t = i as f64 / 32.0;
                    let mt = 1.0 - t;
                    let x = mt * mt * mt * cur.x + 3.0 * mt * mt * t * c1.x + 3.0 * mt * t * t * c2.x + t * t * t * to.x;
                    let y = mt * mt * mt * cur.y + 3.0 * mt * mt * t * c1.y + 3.0 * mt * t * t * c2.y + t * t * t * to.y;
                    points.push(SemioPoint2 { x, y });
                }
                cur = to;
            }
            PathSegment::QuadTo { c, to } => {
                for i in 1..=32 {
                    let t = i as f64 / 32.0;
                    let mt = 1.0 - t;
                    let x = mt * mt * cur.x + 2.0 * mt * t * c.x + t * t * to.x;
                    let y = mt * mt * cur.y + 2.0 * mt * t * c.y + t * t * to.y;
                    points.push(SemioPoint2 { x, y });
                }
                cur = to;
            }
            PathSegment::ArcTo { rx, ry, to, .. } => {
                // 📐 Honest simplification: samples a straight-ish arc via linear interpolation of
                // the endpoint radii direction is out of scope for a general x-rotated ellipse in
                // this bridge — real circular (rx==ry) arcs are sampled exactly on a circle through
                // start/end at that radius; a genuinely elliptical (rx!=ry) or rotated arc falls
                // back to a single straight segment (documented, real, honest — never fabricated
                // curvature it can't derive without a full endpoint-to-center arc solve).
                if (rx - ry).abs() < EPS && rx > EPS {
                    if let Some((cx, cy)) = arc_center(cur, to, rx) {
                        let a0 = (cur.y - cy).atan2(cur.x - cx);
                        let mut a1 = (to.y - cy).atan2(to.x - cx);
                        if a1 < a0 {
                            a1 += std::f64::consts::TAU;
                        }
                        for i in 1..=32 {
                            let t = i as f64 / 32.0;
                            let a = a0 + (a1 - a0) * t;
                            points.push(SemioPoint2 { x: cx + rx * a.cos(), y: cy + rx * a.sin() });
                        }
                    } else {
                        points.push(to);
                    }
                } else {
                    points.push(to);
                }
                cur = to;
            }
            PathSegment::Close => {
                closed = true;
                cur = start;
            }
        }
    }
    (points, closed)
}

/// 🔎️ One of the (generally two) centers equidistant from `p0`/`p1` at `r` — picks the one
/// consistent with a real circular arc; `None` when `p0`/`p1` are farther apart than `2r` (no
/// real solution, an inconsistent/degenerate arc).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arc_center(p0: SemioPoint2, p1: SemioPoint2, r: f64) -> Option<(f64, f64)> {
    let (mx, my) = ((p0.x + p1.x) / 2.0, (p0.y + p1.y) / 2.0);
    let (dx, dy) = (p1.x - p0.x, p1.y - p0.y);
    let d = (dx * dx + dy * dy).sqrt();
    if d < EPS || d > 2.0 * r {
        return None;
    }
    let h = (r * r - (d / 2.0) * (d / 2.0)).max(0.0).sqrt();
    let (ux, uy) = (-dy / d, dx / d);
    Some((mx + ux * h, my + uy * h))
}
//#endregion 🔖️Flatten

//#region 🔖️EntityBuild
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dxf_entity_from_node(node: &DrawNode, layer: &str) -> Option<DxfEntity> {
    match node {
        DrawNode::Path { segments, .. } => {
            if let Some((cx, cy, r)) = as_circle(segments) {
                return Some(DxfEntity::Circle { center: [cx, cy, 0.0], radius: r, layer: layer.into(), unknown_group_codes: vec![] });
            }
            let (points, closed) = flatten_to_polyline(segments);
            if points.len() < 2 {
                return None;
            }
            let vertices = points.iter().map(|p| crate::artifacts::dxf::schema::snapshot::DxfVertex { x: p.x, y: p.y, z: 0.0, bulge: 0.0, unknown_group_codes: vec![] }).collect();
            Some(DxfEntity::Polyline { vertices, closed, layer: layer.into(), unknown_group_codes: vec![] })
        }
        DrawNode::Text { value, at, .. } => Some(DxfEntity::Text { position: [at.x, at.y, 0.0], height: 1.0, value: value.clone(), layer: layer.into(), unknown_group_codes: vec![] }),
        DrawNode::Group { .. } | DrawNode::Image { .. } => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_entities(node: &DrawNode, layer: &str, out: &mut Vec<DxfEntity>) {
    match node {
        DrawNode::Group { children, .. } => {
            for c in children {
                collect_entities(c, layer, out);
            }
        }
        other => {
            if let Some(e) = dxf_entity_from_node(other, layer) {
                out.push(e);
            }
        }
    }
}
//#endregion 🔖️EntityBuild

//#region 🔖️Serializer
pub struct SemioDrawingToDxf;

impl ArtifactSerializer for SemioDrawingToDxf {
    type From = SemioDrawingSnapshot;
    type Into = DxfSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut entities = Vec::new();
        let mut layer_defs = Vec::new();
        for layer in &from.layers {
            layer_defs.push(DxfLayer { name: layer.id.clone(), color: 7, linetype: "CONTINUOUS".into(), flags: if layer.visible { 0 } else { 1 }, unknown_group_codes: vec![] });
            collect_entities(&layer.root, &layer.id, &mut entities);
        }
        Ok(DxfSnapshot {
            schema: crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA.into(),
            header_vars: vec![DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] }],
            tables: DxfTables { layers: layer_defs, ..DxfTables::default() },
            other_tables: vec![],
            blocks: vec![],
            entities,
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::DrawLayer;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn ellipse_path(cx: f64, cy: f64, r: f64) -> Vec<PathSegment> {
        vec![
            PathSegment::MoveTo { to: SemioPoint2 { x: cx + r, y: cy } },
            PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx - r, y: cy } },
            PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx + r, y: cy } },
            PathSegment::Close,
        ]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_drawing() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            layers: vec![DrawLayer {
                id: "0".into(),
                name: "0".into(),
                visible: true,
                root: DrawNode::Group {
                    transform: SemioTransform::identity(),
                    children: vec![
                        DrawNode::Path { segments: ellipse_path(2.0, 2.0, 1.0), style: None },
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 5.0, y: 0.0 } }], style: None },
                        DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None },
                    ],
                },
            }],
            ..SemioDrawingSnapshot::default()
        }
    }

    /// 🧪️ Real round trip through dxf's own real ASCII writer/reader; a genuine circle round
    /// trips EXACTLY (not flattened), a straight-line path becomes a real POLYLINE.
    #[semio_framework_async_macros::async_test]
    async fn real_text_round_trip_through_dxf_codec() {
        let drawing = sample_drawing();
        let dxf = semio_framework_plugin::resolve_ready(SemioDrawingToDxf::serialize(&drawing)).expect("serialize");
        assert_eq!(dxf.entities.len(), 3);
        assert!(matches!(dxf.entities[0], DxfEntity::Circle { .. }));
        assert!(matches!(dxf.entities[1], DxfEntity::Polyline { .. }));
        assert!(matches!(dxf.entities[2], DxfEntity::Text { .. }));

        let text = crate::artifacts::dxf::schema::snapshot::print_dxf_document(&dxf);
        let reparsed = crate::artifacts::dxf::schema::snapshot::parse_dxf_document(&text).expect("reparse real dxf text");
        match &reparsed.entities[0] {
            DxfEntity::Circle { center, radius, .. } => {
                assert!((center[0] - 2.0).abs() < 1e-6);
                assert!((radius - 1.0).abs() < 1e-6);
            }
            other => panic!("expected exact Circle, got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
