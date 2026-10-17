//! 🧪️ The Semio drawing codec against the layout slot: lowering and lifting agree with the typed standard, and external formats are real files.

#[path = "🌉️media/🦀️.rs"]
mod media;
#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

use super::*;
use semio_framework_io_ports::EmbeddedChildSlotV1;

fn bind() {
    bind_layout_background_drawing().expect("the Semio drawing codec binds");
}

fn point(x: f64, y: f64) -> SemioPoint2 {
    SemioPoint2 { x, y }
}

fn drawing() -> SemioDrawingSnapshot {
    let mut turned = SemioTransform::identity();
    turned.translation = SemioPoint3 { x: 4.0, y: 5.0, z: 0.0 };
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 120.0, height: 80.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![DrawStyle { name: "wall".into(), fill: Some(SemioRgba { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }), stroke: Some(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), stroke_width: Some(2.0), opacity: Some(0.5) }],
        layers: vec![
            DrawLayer {
                id: "plan".into(),
                name: "Plan".into(),
                visible: true,
                root: DrawNode::Group {
                    transform: turned,
                    children: vec![
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: point(0.0, 0.0) }, PathSegment::LineTo { to: point(10.0, 0.0) }, PathSegment::CubicTo { c1: point(11.0, 1.0), c2: point(12.0, 2.0), to: point(13.0, 3.0) }, PathSegment::QuadTo { c: point(14.0, 4.0), to: point(15.0, 5.0) }, PathSegment::ArcTo { rx: 1.0, ry: 2.0, x_rotation: 30.0, large_arc: true, sweep: false, to: point(16.0, 6.0) }, PathSegment::Close], style: Some("wall".into()) },
                        DrawNode::Text { value: "Kitchen".into(), at: point(2.0, 3.0), style: None },
                        DrawNode::Image { at: point(1.0, 1.0), width: 4.0, height: 5.0, mime: "image/png".into(), bytes: vec![0, 127, 255] },
                    ],
                },
            },
            DrawLayer { id: "hidden".into(), name: "Hidden".into(), visible: false, root: DrawNode::Text { value: "Spare".into(), at: point(0.0, 0.0), style: Some("wall".into()) } },
        ],
    }
}

fn rect_drawing() -> SemioDrawingSnapshot {
    let rect = vec![PathSegment::MoveTo { to: point(0.0, 0.0) }, PathSegment::LineTo { to: point(210.0, 0.0) }, PathSegment::LineTo { to: point(210.0, 297.0) }, PathSegment::LineTo { to: point(0.0, 297.0) }, PathSegment::Close];
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 210.0, height: 297.0, background: None },
        styles: Vec::new(),
        layers: vec![DrawLayer { id: "page".into(), name: "Page".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Path { segments: rect, style: None }] } }],
    }
}

#[test]
fn binding_is_idempotent_and_the_slot_hands_out_the_semio_codec() {
    bind();
    bind();
    let codec = semio_s_artifact_layout_layout_model::drawing_codec().expect("bound");
    assert_eq!(codec.import_formats(), &["svg", "dxf", "dwg"]);
    assert_eq!(codec.export_formats(), &["svg", "dwg", "dxf", "png"]);
    let foreign = EmbeddedChildSlotV1::new("test.foreign");
    assert!(foreign.bind(&SEMIO_DRAWING_CODEC).is_ok());
    assert!(foreign.bind(&SEMIO_DRAWING_CODEC).is_ok());
}

#[test]
fn lowering_names_every_field_and_lifting_returns_the_same_content() {
    bind();
    let content = drawing().to_value();
    let scene = SEMIO_DRAWING_CODEC.lower(&content).expect("lower");
    assert_eq!(scene.schema, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA);
    assert_eq!(scene.canvas.width, 120.0);
    assert_eq!(scene.styles[0].name, "wall");
    assert_eq!(scene.styles[0].stroke_width, Some(2.0));
    assert_eq!(scene.styles[0].opacity, Some(0.5));
    assert_eq!(scene.layers.len(), 2);
    assert!(!scene.layers[1].visible);
    assert_eq!(scene.texts(), vec!["Kitchen", "Spare"]);
    assert_eq!(SEMIO_DRAWING_CODEC.lift(&scene).expect("lift"), content);
}

#[test]
fn an_empty_scene_schema_lifts_with_the_standards_own_schema_id() {
    let scene = VectorScene::default();
    let content = SEMIO_DRAWING_CODEC.lift(&scene).expect("lift");
    let snapshot = <SemioDrawingSnapshot as FromValue>::from_value(content).expect("a drawing");
    assert_eq!(snapshot.schema, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA);
}

#[test]
fn validation_admits_a_drawing_and_refuses_a_foreign_value() {
    let mut accepted = |_| true;
    let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut accepted);
    assert!(SEMIO_DRAWING_CODEC.validate(&drawing().to_value(), &mut control).is_ok());
    let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut accepted);
    assert!(SEMIO_DRAWING_CODEC.validate(&DslValue::String("not a drawing".into()), &mut control).is_err());
    let mut control = NativeDecodeControl::new(1, &mut accepted);
    assert!(SEMIO_DRAWING_CODEC.validate(&drawing().to_value(), &mut control).is_err(), "a decode budget of one byte refuses the drawing");
}

#[test]
fn the_sqlite_component_is_the_standards_own_ddl() {
    bind();
    assert_eq!(SEMIO_DRAWING_CODEC.sqlite_schema(), <SemioDrawingSnapshot as pack::snapshot::ArtifactSqliteSnapshot>::SQLITE_SCHEMA);
    assert!(SEMIO_DRAWING_CODEC.sqlite_owns_table("semio_drawing_document"));
    assert!(!SEMIO_DRAWING_CODEC.sqlite_owns_table("layout_document"));
    let composed = <semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot as pack::snapshot::ArtifactSqliteSnapshot>::sqlite_schema();
    assert!(composed.starts_with(<semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot as pack::snapshot::ArtifactSqliteSnapshot>::SQLITE_SCHEMA));
    assert!(composed.ends_with(SEMIO_DRAWING_CODEC.sqlite_schema()));
}

#[test]
fn exports_are_real_files_in_every_declared_format() {
    let content = rect_drawing().to_value();
    let svg = String::from_utf8(SEMIO_DRAWING_CODEC.export("svg", &content).expect("svg")).expect("svg text");
    assert!(svg.starts_with("<svg"), "{svg}");
    assert!(svg.contains("<path"), "{svg}");
    let dxf = String::from_utf8(SEMIO_DRAWING_CODEC.export("dxf", &content).expect("dxf")).expect("dxf text");
    assert!(dxf.contains("POLYLINE"), "{dxf}");
    assert!(!SEMIO_DRAWING_CODEC.export("dwg", &content).expect("dwg").is_empty());
    let png = SEMIO_DRAWING_CODEC.export("png", &content).expect("png");
    assert_eq!(&png[..8], &[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]);
}

#[test]
fn an_exported_svg_and_dxf_read_back_as_drawings() {
    let content = rect_drawing().to_value();
    for format in ["svg", "dxf"] {
        let bytes = SEMIO_DRAWING_CODEC.export(format, &content).expect("export");
        let imported = SEMIO_DRAWING_CODEC.import(format, &bytes).expect("import");
        let scene = SEMIO_DRAWING_CODEC.lower(&imported).expect("the imported content is a drawing");
        assert!(!scene.layers.is_empty(), "{format}");
    }
}

#[test]
fn unknown_formats_are_refused_in_both_directions() {
    let content = drawing().to_value();
    assert!(SEMIO_DRAWING_CODEC.export("pdf", &content).is_err());
    assert!(SEMIO_DRAWING_CODEC.import("pdf", b"%PDF").is_err());
}

#[test]
fn a_dwg_with_rectangular_boundaries_frames_one_page_per_rectangle() {
    let mut dwg = DwgDrawing::default();
    dwg.entities.push(semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgEntity {
        layer: 0,
        color: semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::ByLayer,
        geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [210.0, 0.0], [210.0, 297.0], [0.0, 297.0]], bulges: vec![0.0; 4] },
    });
    let imported = dwg_drawing_to_semio_drawing(&dwg);
    assert_eq!(imported.layers.len(), 1);
    assert_eq!(imported.layers[0].id, "imported");
    let DrawNode::Group { children, .. } = &imported.layers[0].root else { panic!("one group") };
    assert_eq!(children.len(), 1);
}
