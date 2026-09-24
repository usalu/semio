//! 🔮️ Third-party DWG oracle: `acadrust` (MPL-2.0, pure Rust, test-only) reads and writes DWG with a
//! codebase independent of this subset's. Three directions, no network at test time (the crate is a
//! build-time dev-dependency; the fixture is committed):
//! - the committed AutoCAD-written AC1024 drawing, read by both readers;
//! - this subset's AC1024 writer, read by acadrust: every entity kind the drawing model carries;
//! - acadrust's writer at every version this subset reads (AC1018, AC1021, AC1024, AC1027, AC1032),
//!   read by this subset.
//! Compared: each model-space entity as kind + layer + colour + coordinates rounded to 1e-6, as a
//! sorted multiset. LibreDWG 0.13 (`dwgread`) cross-checks the same files by hand (see the slice report).
use super::*;
use acadrust::entities::EntityType;
use acadrust::types::{DxfVersion, Vector2, Vector3};

const ARCHITECTURAL: &[u8] = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn round(value: f64) -> i64 {
    (value * 1e6).round() as i64
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rounded(points: &[[f64; 3]]) -> Vec<[i64; 3]> {
    points.iter().map(|point| point.map(round)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ours(drawing: &DwgDrawing) -> Vec<String> {
    let mut out: Vec<String> = drawing
        .entities
        .iter()
        .map(|entity| {
            let shape = match &entity.geometry {
                DwgGeometry::Line { start, end } => format!("LINE {:?}", rounded(&[*start, *end])),
                DwgGeometry::Point { at } => format!("POINT {:?}", rounded(&[*at])),
                DwgGeometry::Circle { center, radius, .. } => format!("CIRCLE {:?} {}", rounded(&[*center]), round(*radius)),
                DwgGeometry::Arc { center, radius, start_angle, end_angle, .. } => format!("ARC {:?} {:?}", rounded(&[*center]), [*radius, *start_angle, *end_angle].map(round)),
                DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, .. } => format!("ELLIPSE {:?} {:?}", rounded(&[*center, *major_axis]), [*ratio, *start_param, *end_param].map(round)),
                DwgGeometry::LwPolyline { closed, vertices, bulges, .. } => format!("LWPOLYLINE {closed} {:?} {:?}", vertices.iter().map(|v| v.map(round)).collect::<Vec<_>>(), bulges.iter().map(|b| round(*b)).collect::<Vec<_>>()),
                DwgGeometry::Spline { degree, control_points, knots, .. } => format!("SPLINE {degree} {:?} {:?}", rounded(control_points), knots.iter().map(|k| round(*k)).collect::<Vec<_>>()),
                DwgGeometry::Text { at, height, content, .. } => format!("TEXT {:?} {} {content}", [at[0], at[1]].map(round), round(*height)),
                DwgGeometry::Face3d { corners } => format!("3DFACE {:?}", rounded(corners)),
                DwgGeometry::Polyline3d { closed, vertices } => format!("POLYLINE3D {closed} {:?}", rounded(vertices)),
                DwgGeometry::PolyfaceMesh { vertices, faces } => format!("PFACE {:?} {faces:?}", rounded(vertices)),
            };
            let color = match entity.color {
                DwgColor::ByLayer => "bylayer".to_string(),
                DwgColor::ByBlock => "byblock".to_string(),
                DwgColor::Index(index) => index.to_string(),
            };
            format!("{} {color} {shape}", drawing.layers[entity.layer].name)
        })
        .collect();
    out.sort();
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vector(value: &Vector3) -> [f64; 3] {
    [value.x, value.y, value.z]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn theirs(document: &acadrust::CadDocument) -> Vec<String> {
    let mut out: Vec<String> = document
        .entities()
        .filter_map(|entity| {
            let shape = match entity {
                EntityType::Line(line) => format!("LINE {:?}", rounded(&[vector(&line.start), vector(&line.end)])),
                EntityType::Point(point) => format!("POINT {:?}", rounded(&[vector(&point.location)])),
                EntityType::Circle(circle) => format!("CIRCLE {:?} {}", rounded(&[vector(&circle.center)]), round(circle.radius)),
                EntityType::Arc(arc) => format!("ARC {:?} {:?}", rounded(&[vector(&arc.center)]), [arc.radius, arc.start_angle, arc.end_angle].map(round)),
                EntityType::Ellipse(ellipse) => format!("ELLIPSE {:?} {:?}", rounded(&[vector(&ellipse.center), vector(&ellipse.major_axis)]), [ellipse.minor_axis_ratio, ellipse.start_parameter, ellipse.end_parameter].map(round)),
                EntityType::LwPolyline(polyline) => format!(
                    "LWPOLYLINE {} {:?} {:?}",
                    polyline.is_closed,
                    polyline.vertices.iter().map(|v| [v.location.x, v.location.y].map(round)).collect::<Vec<_>>(),
                    polyline.vertices.iter().map(|v| round(v.bulge)).collect::<Vec<_>>()
                ),
                EntityType::Spline(spline) => format!("SPLINE {} {:?} {:?}", spline.degree, rounded(&spline.control_points.iter().map(vector).collect::<Vec<_>>()), spline.knots.iter().map(|k| round(*k)).collect::<Vec<_>>()),
                EntityType::Text(text) => format!("TEXT {:?} {} {}", [text.insertion_point.x, text.insertion_point.y].map(round), round(text.height), text.value),
                EntityType::Face3D(face) => format!("3DFACE {:?}", rounded(&[vector(&face.first_corner), vector(&face.second_corner), vector(&face.third_corner), vector(&face.fourth_corner)])),
                EntityType::Polyline3D(polyline) => format!("POLYLINE3D {} {:?}", polyline.flags.closed, rounded(&polyline.vertices.iter().map(|v| vector(&v.position)).collect::<Vec<_>>())),
                EntityType::PolyfaceMesh(mesh) => format!(
                    "PFACE {:?} {:?}",
                    rounded(&mesh.vertices.iter().map(|v| vector(&v.location)).collect::<Vec<_>>()),
                    mesh.faces.iter().map(|f| [f.index1, f.index2, f.index3, f.index4].map(i32::from)).collect::<Vec<_>>()
                ),
                _ => return None,
            };
            let color = match entity.common().color {
                acadrust::types::Color::ByLayer => "bylayer".to_string(),
                acadrust::types::Color::ByBlock => "byblock".to_string(),
                other => other.index().map(|index| index.to_string()).unwrap_or_else(|| format!("{other:?}")),
            };
            Some(format!("{} {color} {shape}", entity.common().layer))
        })
        .collect();
    out.sort();
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn acadrust_read(bytes: &[u8]) -> acadrust::CadDocument {
    acadrust::DwgReader::from_stream(std::io::Cursor::new(bytes.to_vec())).read().expect("acadrust reads the DWG")
}

/// 🧩️ One of every entity kind the drawing model carries, on two layers and in three colour modes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn every_entity_kind() -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    let zero = drawing.ensure_layer("0");
    let walls = drawing.ensure_layer("Walls");
    let entity = |layer, color, geometry| DwgEntity { layer, color, geometry };
    drawing.entities = vec![
        entity(zero, DwgColor::ByLayer, DwgGeometry::Line { start: [1.0, 2.0, 0.0], end: [30.5, -4.25, 0.0] }),
        entity(walls, DwgColor::Index(3), DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [0.0, 5.0]], bulges: vec![0.0, 0.5, 0.0, 0.0] }),
        entity(zero, DwgColor::ByBlock, DwgGeometry::Circle { center: [7.0, 8.0, 0.0], radius: 2.5, normal: [0.0, 0.0, 1.0] }),
        entity(walls, DwgColor::ByLayer, DwgGeometry::Arc { center: [-3.0, 4.0, 0.0], radius: 1.5, start_angle: 0.25, end_angle: 2.0, normal: [0.0, 0.0, 1.0] }),
        entity(zero, DwgColor::Index(1), DwgGeometry::Text { at: [2.0, 3.0, 0.0], height: 1.0, rotation: 0.0, content: "Plan".into() }),
        entity(zero, DwgColor::ByLayer, DwgGeometry::Point { at: [4.0, 4.0, 1.0] }),
        entity(zero, DwgColor::ByLayer, DwgGeometry::Ellipse { center: [1.0, 1.0, 0.0], major_axis: [3.0, 0.0, 0.0], ratio: 0.5, start_param: 0.0, end_param: std::f64::consts::TAU, normal: [0.0, 0.0, 1.0] }),
        entity(zero, DwgColor::ByLayer, DwgGeometry::Spline { degree: 3, control_points: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [3.0, 2.0, 0.0], [4.0, 0.0, 0.0]], knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], weights: vec![] }),
        entity(walls, DwgColor::ByLayer, DwgGeometry::Face3d { corners: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]] }),
        entity(zero, DwgColor::Index(5), DwgGeometry::Polyline3d { closed: true, vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 2.0], [1.0, 1.0, 3.0]] }),
        entity(walls, DwgColor::ByLayer, DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4], [1, 3, 4, 4]] }),
    ];
    drawing
}

#[test]
fn both_readers_agree_on_the_committed_autocad_drawing() {
    let (a, b) = (ours(&dwg_from_bytes(ARCHITECTURAL).expect("this subset reads the DWG")), theirs(&acadrust_read(ARCHITECTURAL)));
    assert_eq!(a.len(), 68, "the AutoCAD drawing's model space: lines, lwpolylines, arcs");
    assert_eq!(a, b);
}

#[test]
fn acadrust_reads_what_this_subset_writes() {
    let drawing = every_entity_kind();
    let bytes = dwg_to_bytes(&drawing).expect("this subset writes the DWG");
    assert_eq!(&bytes[..6], b"AC1024");
    let document = acadrust_read(&bytes);
    assert_eq!(document.version, DxfVersion::AC1024);
    assert_eq!(theirs(&document), ours(&drawing));
}

#[test]
fn this_subset_reads_what_acadrust_writes() {
    use acadrust::entities::{Arc, Circle, Ellipse, Face3D, Line, LwPolyline, Point, PolyfaceMesh, Polyline3D, Spline, Text};
    for version in [DxfVersion::AC1018, DxfVersion::AC1021, DxfVersion::AC1024, DxfVersion::AC1027, DxfVersion::AC1032] {
        let mut document = acadrust::CadDocument::new();
        document.version = version;
        let v3 = |x, y, z| Vector3 { x, y, z };
        document.add_entity(EntityType::Line(Line::from_points(v3(1.0, 2.0, 0.0), v3(30.5, -4.25, 0.0)))).expect("line");
        let mut polyline = LwPolyline::from_points(vec![Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 10.0, y: 0.0 }, Vector2 { x: 10.0, y: 5.0 }]);
        polyline.is_closed = true;
        polyline.vertices[1].bulge = 0.5;
        document.add_entity(EntityType::LwPolyline(polyline)).expect("polyline");
        let mut circle = Circle::new();
        circle.center = v3(7.0, 8.0, 0.0);
        circle.radius = 2.5;
        document.add_entity(EntityType::Circle(circle)).expect("circle");
        let mut arc = Arc::new();
        arc.center = v3(-3.0, 4.0, 0.0);
        arc.radius = 1.5;
        arc.start_angle = 0.25;
        arc.end_angle = 2.0;
        document.add_entity(EntityType::Arc(arc)).expect("arc");
        let mut text = Text::new();
        text.insertion_point = v3(2.0, 3.0, 0.0);
        text.height = 1.0;
        text.value = "Plan".into();
        document.add_entity(EntityType::Text(text)).expect("text");
        document.add_entity(EntityType::Point(Point::from_coords(4.0, 4.0, 1.0))).expect("point");
        let mut ellipse = Ellipse::from_center_axes(v3(1.0, 1.0, 0.0), v3(3.0, 0.0, 0.0), 0.5);
        ellipse.start_parameter = 0.0;
        ellipse.end_parameter = std::f64::consts::TAU;
        document.add_entity(EntityType::Ellipse(ellipse)).expect("ellipse");
        let mut spline = Spline::from_control_points(3, vec![v3(0.0, 0.0, 0.0), v3(1.0, 2.0, 0.0), v3(3.0, 2.0, 0.0), v3(4.0, 0.0, 0.0)]);
        spline.knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        document.add_entity(EntityType::Spline(spline)).expect("spline");
        document.add_entity(EntityType::Face3D(Face3D::new(v3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), v3(1.0, 1.0, 1.0), v3(0.0, 1.0, 1.0)))).expect("3dface");
        let mut polyline3d = Polyline3D::from_points(vec![v3(0.0, 0.0, 0.0), v3(1.0, 0.0, 2.0), v3(1.0, 1.0, 3.0)]);
        polyline3d.flags.closed = true;
        document.add_entity(EntityType::Polyline3D(polyline3d)).expect("polyline3d");
        let mut mesh = PolyfaceMesh::new();
        for [x, y, z] in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]] {
            mesh.add_vertex_xyz(x, y, z);
        }
        mesh.add_quad(1, 2, 3, 4);
        document.add_entity(EntityType::PolyfaceMesh(mesh)).expect("pface");
        let bytes = acadrust::DwgWriter::write_to_vec(&document).expect("acadrust writes the DWG");
        let expected = theirs(&acadrust_read(&bytes));
        assert_eq!(expected.len(), 11, "{version:?}: acadrust reads back its own eleven entities");
        assert_eq!(ours(&dwg_from_bytes(&bytes).unwrap_or_else(|error| panic!("{version:?}: {error}"))), expected, "{version:?}");
    }
}

#[test]
fn a_written_document_reads_back_entity_for_entity_and_re_encodes_byte_for_byte() {
    let drawing = every_entity_kind();
    let bytes = dwg_to_bytes(&drawing).expect("this subset writes the DWG");
    assert_eq!(ours(&dwg_from_bytes(&bytes).expect("drawing reader")), ours(&drawing));
    let snapshot = crate::schema::snapshot::decode_dwg(&bytes).expect("lossless AC1024 decoder");
    let written = crate::DwgSnapshot::from_drawing(&drawing).expect("new document");
    for (read, expected) in snapshot.drawing.objects.iter().zip(&written.drawing.objects) {
        assert_eq!(read, expected);
    }
    assert_eq!(snapshot.drawing.objects.len(), written.drawing.objects.len());
    assert_eq!(snapshot.header, written.header);
    assert_eq!(snapshot.classes, written.classes);
    assert_eq!(snapshot.summary, written.summary);
    assert_eq!(snapshot.application, written.application);
    assert_eq!(snapshot.auxiliary_header, written.auxiliary_header);
    assert_eq!(snapshot.preview, written.preview);
    assert_eq!(snapshot.application_history, written.application_history);
    assert_eq!(crate::DwgSnapshot { drawing: written.drawing.clone(), ..snapshot.clone() }, written);
    assert_eq!(crate::schema::snapshot::encode_dwg(&snapshot).expect("re-encode"), bytes);
}
