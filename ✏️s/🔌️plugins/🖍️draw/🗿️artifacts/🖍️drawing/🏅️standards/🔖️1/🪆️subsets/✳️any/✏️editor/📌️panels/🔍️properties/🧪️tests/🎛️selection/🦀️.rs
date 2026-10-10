//! 🧪️ Language-neutral inspector journeys through the actual UI projection.
use super::*;
use crate::schema::{create_drawing_shape_layer_rect, layer_base_mut};
use semio_framework_plugin::{artifact_app_laws::project_and_retire_fixture_tree, built_to_component_tree, ViewModel};

fn retire_inspector_view(view:ViewModel){
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let mut owner=ControlledRetirement::new(view).unwrap_or_else(|(_,original)|{let _original=std::mem::ManuallyDrop::new(original);panic!("original inspector view retirement was refused");});
    for _ in 0..100000 {
        if owner.terminal_is_empty(){return;}
        let copy=owner.next_copy_byte_demand().unwrap().max(4096);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        assert!(owner.step(grant).unwrap().progress().fits(grant));
    }
    panic!("original inspector view retained physical owners");
}

#[test]
fn inspector_selection_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let mut layer = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Rectangle")).to_string().into()).expect("nonempty authored identity"), "Rectangle");
    layer_base_mut(&mut layer).id = "shape.a".into();
    let document = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    for case in fixture["cases"].as_array().unwrap() {
        let ids = case["selection"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let tree = render(&document, &ids, &DrawingPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        let _: serde_json::Value = serde_json::from_str(&json).unwrap();
        for field in case["expectedFields"].as_array().unwrap() {
            assert!(json.contains(&format!("drawing-inspector.{}.input", field.as_str().unwrap())), "{}: {json}", case["name"]);
        }
        assert_eq!(json.contains("patchLayers"), !case["expectedFields"].as_array().unwrap().is_empty(), "{}: {json}", case["name"]);
    }
}

#[test]
fn inspector_stroke_controls_are_localized() {
    let layer = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Rectangle")).to_string().into()).expect("nonempty authored identity"), "Rectangle");
    let id = layer_base(&layer).id.clone();
    let document = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        let tree = render(&document, &[id.to_string_owner()], labels, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        for label in [labels.stroke_cap, labels.stroke_join, labels.stroke_dash, labels.cap_square, labels.join_bevel, labels.fill_rule, labels.fill_evenodd, labels.fill_nonzero] {
            assert!(json.contains(label.as_str()), "missing {}", label.as_str());
        }
    }
}

#[test]
fn inspector_path_nodes_publish_localized_edit_actions() {
    let mut layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Cubic { ctrl1: [0.0, 12.0], ctrl2: [12.0, 12.0], to: [12.0, 0.0] }, PathSegment::Line { to: [12.0,12.0] }, PathSegment::Move { to: [20.0,20.0] }, PathSegment::Line { to: [30.0,30.0] }].into());
    layer_base_mut(&mut layer).id = "path".into();
    let mut document = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        let mut json = String::new();
        for offset in 0..5 {
            let paged = ViewModel { tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: DRAWING_PLAY_BODY_PROPERTIES.into(), node_key: "drawing-inspector.nodes".into(), open: Some(true), offset, rows: 1 }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
            let tree = render(&document, &["path".into()], labels, &TreeWindows::for_body(&paged, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
            json.push_str(&project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap());
            retire_inspector_view(paged);
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
}

fn inspector_input_rows(node: &serde_json::Value, output: &mut Vec<serde_json::Value>) {
    let children = node["children"].as_array().unwrap();
    let inputs = children.iter().filter(|child| matches!(child["component"]["type"].as_str(), Some("input" | "select")))
        .map(|child| child["key"].as_str().unwrap().strip_prefix("drawing-inspector.").unwrap()).collect::<Vec<_>>();
    if !inputs.is_empty() {
        output.push(serde_json::json!({"id":node["key"].as_str().unwrap().strip_prefix("drawing-inspector.").unwrap(),"inputs":inputs}));
    }
    for child in children { inspector_input_rows(child, output); }
}

#[test]
fn node_coordinates_are_grouped_into_readable_pairs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️node-rows/🔣️.json")).unwrap();
    let segments: Vec<PathSegment> = serde_json::from_value(fixture["segments"].clone()).unwrap();
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        for disabled in [false, true] {
            let mut actual = Vec::new();
            for (index, segment) in segments.iter().enumerate() {
                let row = node_row("path", index, segment, None, disabled, labels).unwrap();
                let json = project_and_retire_fixture_tree(built_to_component_tree(row)).unwrap();
                inspector_input_rows(&serde_json::from_str(&json).unwrap(), &mut actual);
            }
            assert_eq!(serde_json::Value::Array(actual), fixture["rows"]);
        }
    }
}

#[test]
fn gradient_coordinates_and_stops_have_labeled_rows() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️fill-rows/🔣️.json")).unwrap();
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        for disabled in [false, true] {
            for case in fixture["cases"].as_array().unwrap() {
                let fill: FillStyle = serde_json::from_value(case["fill"].clone()).unwrap();
                let row = fill_controls("path", Some(&fill), disabled, labels).unwrap();
                let json = project_and_retire_fixture_tree(built_to_component_tree(row)).unwrap();
                let mut actual = Vec::new();
                inspector_input_rows(&serde_json::from_str(&json).unwrap(), &mut actual);
                assert_eq!(serde_json::Value::Array(actual), case["rows"]);
                let stop = crate::schema::fill::stops(&fill).first().unwrap();
                let row = stop_row("path", 0, stop, true, disabled, labels).unwrap();
                let json = project_and_retire_fixture_tree(built_to_component_tree(row)).unwrap();
                let mut actual = Vec::new();
                inspector_input_rows(&serde_json::from_str(&json).unwrap(), &mut actual);
                assert_eq!(serde_json::Value::Array(actual), fixture["stopRows"]);
            }
        }
    }
}

#[test]
fn inspector_exposes_localized_shape_conversion_only_for_editable_shapes() {
    let mut layer=create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");
    layer_base_mut(&mut layer).id="shape".into();
    let mut document=DrawingSnapshot { layers:vec![layer].into(),..Default::default() };
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
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
}

#[test]
fn gradient_inspector_projects_type_coordinates_and_stops() {
    let mut layer = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Gradient")).to_string().into()).expect("nonempty authored identity"), "Gradient");
    layer_base_mut(&mut layer).attributes.fill = crate::schema::fill::edit_fill(None,&crate::schema::fill::FillEdit::Type { value: crate::schema::fill::FillType::LinearGradient }).unwrap();
    let id = layer_base(&layer).id.clone();
    let mut document = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let tree = render(&document,&[id.to_string_owner()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        for control in ["fill.type","fill.x1","fill.x2","fill.stop.0.color","fill.stop.1.alpha","fill.stop.1.offset","fill.add"] { assert!(json.contains(control),"missing {control}"); }
        assert!(json.contains(labels.gradient_stops.as_str()));
        assert!(json.contains("editFill"));
    }
    layer_base_mut(&mut document.layers[0]).locked = true;
    let tree = render(&document,&[id.to_string_owner()],&DrawingPlayLabels::NATIVE_EN,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
    let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
    assert!(!json.contains("fill.add"));
}

#[test]
fn inspector_controls_bind_the_events_the_host_dispatches() {
    fn check(node: &serde_json::Value, events: &serde_json::Value, count: &mut usize) {
        if let Some(expected) = node["component"]["type"].as_str().and_then(|kind| events.get(kind)) {
            let bindings = node["bindings"].as_array().unwrap();
            assert!(bindings.iter().any(|binding| &binding["trigger"] == expected), "{} needs {}: {}", node["key"], expected, node);
            assert!(node["accessibility"]["label"].as_str().is_some_and(|label| !label.is_empty()), "{} has no accessible name", node["key"]);
            *count += 1;
        }
        for child in node["children"].as_array().unwrap() { check(child, events, count); }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let mut layer = crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", vec![PathSegment::Move { to: [0.0,0.0] },PathSegment::Cubic { ctrl1: [0.0,12.0],ctrl2: [12.0,12.0],to: [12.0,0.0] }].into());
    layer_base_mut(&mut layer).attributes.fill = crate::schema::fill::edit_fill(None,&crate::schema::fill::FillEdit::Type { value: crate::schema::fill::FillType::LinearGradient }).unwrap();
    let id = layer_base(&layer).id.clone();
    let document = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let view = ViewModel { tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: DRAWING_PLAY_BODY_PROPERTIES.into(), node_key: "drawing-inspector.nodes".into(), open: Some(true), offset: 0, rows: 2 }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let tree = render(&document,&[id.to_string_owner()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let projection = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        let json = serde_json::from_str(&projection).unwrap();
        let mut count = 0;
        check(&json,&fixture["controlEvents"],&mut count);
        assert!(count >= 30,"missing inspector controls: {count}");
    }
    retire_inspector_view(view);
}

#[test]
fn text_inspector_is_multiline_localized_and_commits_on_blur() {
    let layer = crate::schema::create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Text")).to_string().into()).expect("nonempty authored identity"), "Text");
    let id = layer_base(&layer).id.clone();
    let document = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        let tree = render(&document, &[id.to_string_owner()], labels, &TreeWindows::for_body(&view, DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        for value in [labels.text_content.as_str(), labels.text_size.as_str(), "textContent", "textSize", "longText", "blur", "patchLayers"] { assert!(json.contains(value), "missing {value}: {json}"); }
    }
}

#[test]
fn fill_rule_inspector_preserves_choice_mixed_state_and_lock() {
    fn control(node:&serde_json::Value)->Option<&serde_json::Value> {
        if node["key"]=="drawing-inspector.fillRule.input" {return Some(node);}
        node["children"].as_array()?.iter().find_map(control)
    }
    fn disabled(node:&BuiltNode)->Option<bool> {
        if node.key.as_str()=="drawing-inspector.fillRule.input" {return Some(node.disabled);}
        node.children.iter().find_map(disabled)
    }
    let mut first=create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
    let mut second=create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
    layer_base_mut(&mut first).id="first".into();
    layer_base_mut(&mut second).id="second".into();
    let mut document=DrawingSnapshot {layers:vec![first,second].into(),..Default::default()};
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        for (rule,locked,value) in [(crate::FillRule::Evenodd,false,"evenodd"),(crate::FillRule::Nonzero,false,""),(crate::FillRule::Nonzero,true,"")] {
            layer_base_mut(&mut document.layers[1]).attributes.fill_rule=rule;
            layer_base_mut(&mut document.layers[1]).locked=locked;
            let tree=render(&document,&["first".into(),"second".into()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
            let actual_disabled=disabled(&tree);
            let projection=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
            let node:serde_json::Value=serde_json::from_str(&projection).unwrap();
            let input=control(&node).expect("fill-rule select");
            assert_eq!(input["component"]["type"],"select");
            assert_eq!(input["component"]["value"],value);
            assert_eq!(actual_disabled,Some(locked));
            assert_eq!(input["accessibility"]["label"],labels.fill_rule.as_str());
            assert_eq!(input["component"]["items"].as_array().unwrap().len(),2);
            if value.is_empty() {assert_eq!(input["component"]["placeholder"],labels.mixed.as_str());}
        }
    }
}

#[test]
fn layer_stack_controls_are_localized_and_dispatch_semantic_operations() {
    let mut layer=create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Layer")).to_string().into()).expect("nonempty authored identity"), "Layer");
    layer_base_mut(&mut layer).id="layer".into();
    let document=DrawingSnapshot {layers:vec![layer].into(),..Default::default()};
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let tree=render(&document,&["layer".into()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        for text in [labels.bring_forward.as_str(),labels.send_backward.as_str(),"bringForward","sendBackward","editSelection"] {assert!(json.contains(text),"missing {text}: {json}");}
    }
}

#[test]
fn ungroup_control_appears_for_editable_groups_in_both_languages() {
    let mut group=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    layer_base_mut(&mut group).id="group".into();
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {for locked in [false,true] {
        layer_base_mut(&mut group).locked=locked;
        let document=DrawingSnapshot {layers:vec![group.clone()].into(),..Default::default()};
        let tree=render(&document,&["group".into()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
        assert_eq!(json.contains(labels.ungroup.as_str()),!locked);
        assert_eq!(json.contains("drawing-inspector.ungroup"),!locked);
    }}
}

#[test]
fn group_isolation_inspector_exposes_localized_common_mixed_and_locked_states() {
    fn control(node:&serde_json::Value)->Option<&serde_json::Value> {
        if node["key"]=="drawing-inspector.isolation.input" {return Some(node);}
        node["children"].as_array()?.iter().find_map(control)
    }
    fn disabled(node:&BuiltNode)->Option<bool> {
        if node.key.as_str()=="drawing-inspector.isolation.input" {return Some(node.disabled);}
        node.children.iter().find_map(disabled)
    }
    let mut first=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
    let mut second=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
    layer_base_mut(&mut first).id="first".into();layer_base_mut(&mut second).id="second".into();
    let mut document=DrawingSnapshot {layers:vec![first,second].into(),..Default::default()};
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        for (first_value,second_value,locked) in [(false,false,false),(true,true,false),(true,false,false),(true,false,true)] {
            let DrawingLayerNode::Group(first)=&mut document.layers[0] else {unreachable!()};first.isolation=first_value;
            let DrawingLayerNode::Group(second)=&mut document.layers[1] else {unreachable!()};second.isolation=second_value;second.base.locked=locked;
            let tree=render(&document,&["first".into(),"second".into()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
            let actual_disabled=disabled(&tree);
            let projection=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
            let node:serde_json::Value=serde_json::from_str(&projection).unwrap();let input=control(&node).unwrap();
            assert_eq!(actual_disabled,Some(locked));assert_eq!(input["accessibility"]["label"],labels.isolation.as_str());
            if first_value==second_value {assert_eq!(input["component"]["type"],"toggle");assert_eq!(input["component"]["on"],first_value);}
            else {assert_eq!(input["component"]["type"],"select");assert_eq!(input["component"]["value"],"");assert_eq!(input["component"]["placeholder"],labels.mixed.as_str());assert_eq!(input["component"]["items"].as_array().unwrap().len(),2);}
        }
    }
}

#[test]
fn blend_inspector_exposes_every_mode_and_preserves_mixed_and_locked_states() {
    fn control(node: &serde_json::Value) -> Option<&serde_json::Value> {
        if node["key"] == "drawing-inspector.blendMode.input" { return Some(node); }
        node["children"].as_array()?.iter().find_map(control)
    }
    fn disabled(node: &BuiltNode) -> Option<bool> {
        if node.key.as_str() == "drawing-inspector.blendMode.input" { return Some(node.disabled); }
        node.children.iter().find_map(disabled)
    }
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🧬️mutations/🧫️fixtures/🎛️field-patch/🔣️.json")).unwrap();
    let modes = cases.as_array().unwrap().iter().filter(|case| case["patch"]["field"] == "blendMode" && case["accepted"] == true).map(|case| case["patch"]["value"].as_str().unwrap()).collect::<Vec<_>>();
    assert_eq!(modes.len(), 16);
    let view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for labels in [&DrawingPlayLabels::NATIVE_EN, &DrawingPlayLabels::NATIVE_DE] {
        for mode in &modes { for (mixed, locked) in [(false,false),(true,false),(true,true)] {
            let mut first = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
            let mut second = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
            layer_base_mut(&mut first).id = "first".into();
            layer_base_mut(&mut second).id = "second".into();
            layer_base_mut(&mut first).blend_mode = (*mode).into();
            layer_base_mut(&mut second).blend_mode = if mixed { if *mode == "normal" {"multiply"} else {"normal"} } else {*mode}.into();
            layer_base_mut(&mut second).locked = locked;
            let document = DrawingSnapshot { layers: vec![first,second].into(), ..Default::default() };
            let tree = render(&document, &["first".into(),"second".into()], labels, &TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
            assert_eq!(disabled(&tree), Some(locked));
            let projection = project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
            let json: serde_json::Value = serde_json::from_str(&projection).unwrap();
            let input = control(&json).unwrap();
            assert_eq!(input["component"]["value"], if mixed {""} else {*mode});
            assert_eq!(input["accessibility"]["label"], labels.blend_mode.as_str());
            let items = input["component"]["items"].as_array().unwrap();
            assert_eq!(items.len(), 16);
            for label in [labels.blend_normal, labels.blend_multiply, labels.blend_screen, labels.blend_overlay, labels.blend_darken, labels.blend_lighten, labels.blend_color_dodge, labels.blend_color_burn, labels.blend_hard_light, labels.blend_soft_light, labels.blend_difference, labels.blend_exclusion, labels.blend_hue, labels.blend_saturation, labels.blend_color, labels.blend_luminosity] { assert!(input.to_string().contains(label.as_str()), "missing {}", label.as_str()); }
            for candidate in &modes { assert!(items.iter().any(|item| item["value"] == *candidate), "missing {candidate}: {items:?}"); }
            if mixed { assert_eq!(input["component"]["placeholder"], labels.mixed.as_str()); }
        }}
    }
}

#[test]
fn authored_geometry_controls_match_shared_inspector_contract() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎛️geometry/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let layer=crate::schema::create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit(((case["kind"].as_str().unwrap())).to_string().into()).expect("nonempty authored identity"), case["kind"].as_str().unwrap());
        let id=crate::schema::layer_id(&layer).to_string_owner();
        let document=DrawingSnapshot{layers:vec![layer].into(),..Default::default()};
        for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
            let mut json=String::new();
            for offset in 0..36 {
                let view=ViewModel{tree_windows:vec![semio_framework_plugin::TreeWindowRequest{body_key:DRAWING_PLAY_BODY_PROPERTIES.into(),node_key:ROOT.into(),open:Some(true),offset,rows:1}],..ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native)};
                let tree=render(&document,&[id.clone()],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
                json.push_str(&project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap());
                retire_inspector_view(view);
            }
            for field in case["fields"].as_array().unwrap(){assert!(json.contains(&format!("drawing-inspector.{}.input",field.as_str().unwrap())));}
            if case["kind"]=="shape:polygon" {for field in fixture["polygonCoordinates"].as_array().unwrap(){assert!(json.contains(field.as_str().unwrap()));}}
        }
    }
}

#[test]
fn node_mode_and_simplification_controls_dispatch_semantic_edits() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎛️geometry/🔣️.json")).unwrap();
    let segment=PathSegment::Move{to:[0.0,0.0]};
    for labels in [&DrawingPlayLabels::NATIVE_EN,&DrawingPlayLabels::NATIVE_DE] {
        let row=node_row("path",0,&segment,None,false,labels).unwrap();
        let json=project_and_retire_fixture_tree(built_to_component_tree(row)).unwrap();
        for mode in fixture["nodeModes"].as_array().unwrap(){assert!(json.contains(&format!("mode.{}",mode.as_str().unwrap())));}
        for label in [labels.node_corner,labels.node_smooth,labels.node_symmetric]{assert!(json.contains(label.as_str()));}
        let layer=crate::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), "Path",vec![segment.clone()].into());let id=crate::schema::layer_id(&layer).to_string_owner();
        let document=DrawingSnapshot{layers:vec![layer].into(),..Default::default()};
        let view=ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);
        let tree=render(&document,&[id],labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
        let json=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();assert!(json.contains("path.simplify"));assert!(json.contains(labels.simplify_tolerance.as_str()));
    }
}

/// 🧭️ Neutral eligibility cases reach the actual bilingual inspector with inherited locks and original groups.
#[test]
fn inspector_actions_respect_selection_eligibility() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎬️actions/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let selection=&row["selection"];
        let count=selection["count"].as_u64().unwrap() as usize;
        let locked=selection["locked"].as_bool().unwrap();
        let same_parent=selection["sameParent"].as_bool().unwrap();
        let convertible=selection["convertibleShapes"].as_bool().unwrap();
        let ungroupable=selection["ungroupableGroups"].as_bool().unwrap();
        let mut layers=Vec::new();
        let mut ids=Vec::new();
        for index in 0..count {
            let mut layer=if convertible {create_drawing_shape_layer_rect("Shape")} else if ungroupable || row["name"]=="one unresolved composited group" {crate::schema::create_drawing_group_layer("Group")} else {crate::schema::create_layer_by_kind("text")};
            let id=format!("selected-{index}");
            layer_base_mut(&mut layer).id=id.as_str().into();
            if row["name"]=="one unresolved composited group" {layer_base_mut(&mut layer).opacity=0.5;}
            ids.push(id);
            layers.push(layer);
        }
        if locked {
            let mut parent=crate::schema::create_drawing_group_layer("Locked parent");
            layer_base_mut(&mut parent).id="locked-parent".into();
            layer_base_mut(&mut parent).locked=true;
            if let DrawingLayerNode::Group(group)=&mut parent {group.children=layers.into();}
            layers=vec![parent];
        }else if !same_parent {
            layers=layers.into_iter().enumerate().map(|(index,layer)|{
                let mut parent=crate::schema::create_drawing_group_layer("Parent");
                layer_base_mut(&mut parent).id=format!("parent-{index}").into();
                if let DrawingLayerNode::Group(group)=&mut parent {group.children=vec![layer].into();}
                parent
            }).collect();
        }
        let structure=row["structure"].as_str().unwrap_or("");
        match structure {
            "ancestorChild"|"ancestorChildSibling"=>{
                let mut group=crate::schema::create_drawing_group_layer("Selected ancestor");
                layer_base_mut(&mut group).id="selected-0".into();
                let child=layers.remove(1);
                if let DrawingLayerNode::Group(body)=&mut group {body.children=vec![child].into();}
                layers[0]=group;
            }
            "missing"=>ids.push("missing-layer".into()),
            "hidden"=>layer_base_mut(&mut layers[0]).visible=false,
            "inheritedHidden"=>{
                let mut parent=crate::schema::create_drawing_group_layer("Hidden parent");
                layer_base_mut(&mut parent).id="hidden-parent".into();
                layer_base_mut(&mut parent).visible=false;
                if let DrawingLayerNode::Group(group)=&mut parent {group.children=layers.into();}
                layers=vec![parent];
            }
            "isolated"|"blended"=>{
                let mut group=crate::schema::create_drawing_group_layer("Composited group");
                layer_base_mut(&mut group).id="selected-0".into();
                if let DrawingLayerNode::Group(body)=&mut group {
                    body.isolation=structure=="isolated";
                    if structure=="blended" {body.base.blend_mode="multiply".into();}
                }
                layers=vec![group];
            }
            "duplicateRows"=>{
                ids=vec![crate::schema::drawing_play_layers_tree_row_id(&layers[0]);2];
            }
            "shapePath"=>{
                let shape=&layers[1];
                layers[1]=DrawingLayerNode::Path(crate::DrawingPathBody {base:layer_base(shape).clone(),segments:crate::schema::layer_to_path_segments(shape).into()});
            }
            _=>{}
        }
        let document=DrawingSnapshot {layers:layers.into(),..Default::default()};
        let resolved=crate::schema::selected_drawing_layers(&document,&ids);
        let eligibility=selection_actions::SelectionActionEligibility::from_selection(&document,&resolved,&ids);
        assert_eq!(eligibility.count,count,"{}: resolved count",row["name"]);
        assert_eq!(eligibility.locked,locked,"{}: inherited lock",row["name"]);
        assert_eq!(eligibility.same_parent,same_parent,"{}: same parent",row["name"]);
        assert_eq!(eligibility.complete,selection["complete"].as_bool().unwrap(),"{}: original requested ids",row["name"]);
        assert_eq!(eligibility.arrangement_count,selection["arrangementCount"].as_u64().unwrap() as usize,"{}: ancestor selection boundary",row["name"]);
        assert_eq!(eligibility.arrangeable,selection["arrangeable"].as_bool().unwrap(),"{}: inherited visibility",row["name"]);
        if !structure.is_empty() {
            for operation in ["alignLeft","distributeHorizontal","ungroup"] {
                let expected=eligibility.allows(operation);
                let actual=crate::editor::drawing::commands::edit_selection::plan(&document,&ids,operation).is_ok();
                assert_eq!(actual,expected,"{}: actual planner {operation}",row["name"]);
            }
        }
        let before=document.clone();
        for (locale,labels) in [(semio_framework_ui_locale::Locale::En,&DrawingPlayLabels::NATIVE_EN),(semio_framework_ui_locale::Locale::De,&DrawingPlayLabels::NATIVE_DE)] {
            let view=ViewModel::new(locale,semio_framework_ui_locale::Terminology::Native);
            let tree=render(&document,&ids,labels,&TreeWindows::for_body(&view,DRAWING_PLAY_BODY_PROPERTIES)).unwrap();
            let json=project_and_retire_fixture_tree(built_to_component_tree(tree)).unwrap();
            for action in fixture["actions"].as_array().unwrap() {
                let operation=action.as_str().unwrap();
                let expected=row["expected"].as_array().unwrap().iter().any(|value|value==action);
                assert_eq!(json.contains(&format!("\"drawing-inspector.{operation}\"")),expected,"{}: {operation}: {json}",row["name"]);
            }
            assert_eq!(document,before,"inspector projection cannot publish a document edit");
        }
    }
    eprintln!("[DEBUG] Actual EN/DE inspector action projection matched twenty-one neutral selection cases including inherited locks and unresolved group compositing");
}
