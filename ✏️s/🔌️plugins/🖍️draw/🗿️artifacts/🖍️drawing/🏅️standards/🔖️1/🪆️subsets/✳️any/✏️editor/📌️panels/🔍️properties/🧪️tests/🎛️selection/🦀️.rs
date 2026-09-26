//! 🧪️ Language-neutral inspector journeys through the actual UI projection.
use super::*;
use crate::schema::{create_drawing_shape_layer_rect, layer_base_mut};
use semio_framework_plugin::{artifact_app_laws::project_and_retire_fixture_tree, built_to_component_tree, ViewModel};

#[test]
fn inspector_selection_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let mut layer = create_drawing_shape_layer_rect("Rectangle");
    layer_base_mut(&mut layer).id = "shape.a".into();
    let document = DrawingSnapshot { layers: vec![layer], ..Default::default() };
    for case in fixture["cases"].as_array().unwrap() {
        let ids = case["selection"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let view = ViewModel::default();
        let tree = render(&document, &ids, &DrawingPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        let _: serde_json::Value = serde_json::from_str(&json).unwrap();
        for field in case["expectedFields"].as_array().unwrap() {
            assert!(json.contains(&format!("drawing-inspector.{}.input", field.as_str().unwrap())), "{}: {json}", case["name"]);
        }
        assert_eq!(json.contains("patchLayers"), !case["expectedFields"].as_array().unwrap().is_empty(), "{}: {json}", case["name"]);
        eprintln!("[DEBUG] inspector journey {} projected successfully", case["name"]);
    }
}

#[test]
fn inspector_stroke_controls_are_localized() {
    let layer = create_drawing_shape_layer_rect("Rectangle");
    let id = layer_base(&layer).id.clone();
    let document = DrawingSnapshot { layers: vec![layer], ..Default::default() };
    let view = ViewModel::default();
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        let tree = render(&document, &[id.clone()], labels, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        for label in [labels.stroke_cap, labels.stroke_join, labels.stroke_dash, labels.cap_square, labels.join_bevel] {
            assert!(json.contains(label.as_str()), "missing {}", label.as_str());
        }
    }
    eprintln!("[DEBUG] stroke inspector projected English and German controls");
}

#[test]
fn inspector_path_nodes_publish_localized_edit_actions() {
    let mut layer = crate::schema::create_drawing_path_layer("Curve", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Cubic { ctrl1: [0.0, 12.0], ctrl2: [12.0, 12.0], to: [12.0, 0.0] }, PathSegment::Line { to: [12.0,12.0] }, PathSegment::Move { to: [20.0,20.0] }, PathSegment::Line { to: [30.0,30.0] }]);
    layer_base_mut(&mut layer).id = "path".into();
    let mut document = DrawingSnapshot { layers: vec![layer], ..Default::default() };
    let view = ViewModel::default();
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        let mut json = String::new();
        for offset in 0..5 {
            let paged = ViewModel { tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: DRAWING_PLAY_BODY_PROPERTIES.into(), node_key: "drawing-inspector.nodes".into(), open: Some(true), offset, rows: 1 }], ..Default::default() };
            let tree = render(&document, &["path".into()], labels, &TreeWindows::for_body(&paged, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
            json.push_str(&project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap());
        }
        for id in ["node.0.anchor.x", "node.1.control1.x", "node.1.control2.y", "node.1.split", "node.0.reverse"] { assert!(json.contains(id), "missing {id}"); }
        assert!(json.contains("editPath"));
        assert!(json.contains(labels.nodes.as_str()));
        for id in ["node.1.convert.line","node.2.convert.cubic"] { assert!(json.contains(id),"missing {id}"); }
        assert!(json.contains(labels.curve_segment.as_str()));
        assert!(json.contains(labels.straighten_segment.as_str()));
        assert!(json.contains("node.2.join"));
        assert!(json.contains(labels.join_contour.as_str()));
    }
    layer_base_mut(&mut document.layers[0]).locked = true;
    let tree = render(&document, &["path".into()], &DrawingPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
    let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
    assert!(!json.contains("node.1.split"));
    assert!(!json.contains("node.1.convert"));
    assert!(!json.contains("node.2.join"));
    eprintln!("[DEBUG] localized path-node inspection and lock projection completed");
}

#[test]
fn inspector_exposes_localized_shape_conversion_only_for_editable_shapes() {
    let mut layer=create_drawing_shape_layer_rect("Shape");
    layer_base_mut(&mut layer).id="shape".into();
    let mut document=DrawingSnapshot { layers:vec![layer],..Default::default() };
    let view=ViewModel::default();
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let tree=render(&document,&["shape".into()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        assert!(json.contains(labels.convert_to_path.as_str()));
        assert!(json.contains("toPath"));
    }
    layer_base_mut(&mut document.layers[0]).locked=true;
    let tree=render(&document,&["shape".into()],&DrawingPlayLabels::NATIVE_EN,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
    let json=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
    assert!(!json.contains("toPath"));
    eprintln!("[DEBUG] shape conversion is localized and respects the inspector lock state");
}

#[test]
fn gradient_inspector_projects_type_coordinates_and_stops() {
    let mut layer = create_drawing_shape_layer_rect("Gradient");
    layer_base_mut(&mut layer).attributes.fill = crate::schema::fill::edit_fill(None,&crate::schema::fill::FillEdit::Type { value: crate::schema::fill::FillType::LinearGradient }).unwrap();
    let id = layer_base(&layer).id.clone();
    let mut document = DrawingSnapshot { layers: vec![layer], ..Default::default() };
    let view = ViewModel::default();
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let tree = render(&document,&[id.clone()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        for control in ["fill.type","fill.x1","fill.x2","fill.stop.0.color","fill.stop.1.alpha","fill.stop.1.offset","fill.add"] { assert!(json.contains(control),"missing {control}"); }
        assert!(json.contains(labels.gradient_stops.as_str()));
        assert!(json.contains("editFill"));
    }
    layer_base_mut(&mut document.layers[0]).locked = true;
    let tree = render(&document,&[id],&DrawingPlayLabels::NATIVE_EN,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
    let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
    assert!(!json.contains("fill.add"));
    eprintln!("[DEBUG] gradient inspector projected localized coordinates, stops, and locked actions");
}
