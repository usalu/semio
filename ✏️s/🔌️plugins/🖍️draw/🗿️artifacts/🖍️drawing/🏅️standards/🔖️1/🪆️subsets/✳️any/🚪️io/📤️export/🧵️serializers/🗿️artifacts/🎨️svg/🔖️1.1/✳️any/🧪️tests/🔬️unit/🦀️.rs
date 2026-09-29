use super::*;
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{parse_svg_xml, svg_document_to_typed};

#[test]
fn svg_scene_fixture_preserves_paint_geometry_and_text() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let nodes: Vec<DrawingSceneNode> = dsl::json::from_json_str(&fixture["nodes"].to_string()).unwrap();
    let view_box: [f64;4] = serde_json::from_value(fixture["viewBox"].clone()).unwrap();
    let output = drawing_scene_to_svg(&nodes,view_box).unwrap();
    let root = svg_document_to_typed(&parse_svg_xml(&output).unwrap()).unwrap();
    let SvgElement::Svg { children,view_box:Some(view_box),.. } = root else { panic!("SVG root") };
    assert_eq!([view_box.min_x,view_box.min_y,view_box.width,view_box.height],[-20.0,-10.0,200.0,100.0]);
    assert!(matches!(&children[0],SvgElement::Defs { children,.. } if children.len() == 2));
    let SvgElement::Group { common,children } = &children[1] else { panic!("path wrapper") };
    assert_eq!(common.presentation.opacity.as_deref(),Some("0.7"));
    let SvgElement::Path { common,d } = &children[0] else { panic!("path") };
    assert_eq!(common.presentation.fill.as_deref(),Some("url(#draw-gradient-0)"));
    assert_eq!(common.presentation.stroke_opacity.as_deref(),Some("0.4"));
    assert!(matches!(&d[3],PathCommand::Arc { large_arc:true,sweep:false,.. }));
    assert!(output.contains("font-size=\"20\""));
    assert!(output.contains("A&lt;&amp;&gt;"));
    assert!(output.contains("Ü 🌍"));
    assert!(output.contains("y=\"44\""));
    assert!(output.contains("gradientUnits=\"userSpaceOnUse\""));
    assert!(output.contains("stroke-dasharray=\"3 2\""));
    assert!(output.contains("preserveAspectRatio=\"none\""));
}

#[test]
fn svg_refuses_invalid_view_box_and_transform() {
    assert!(drawing_scene_to_svg(&[],[0.0,0.0,0.0,10.0]).is_err());
    assert!(drawing_scene_to_svg(&[],[f64::NAN,0.0,10.0,10.0]).is_err());
}


#[test]
fn svg_refuses_nonfinite_geometry_and_paint() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for sample in fixture["invalidNumbers"].as_array().unwrap() {
        let mut nodes: Vec<DrawingSceneNode> = dsl::json::from_json_str(&fixture["nodes"].to_string()).unwrap();
        let value = sample["value"].as_str().unwrap().parse::<f64>().unwrap();
        let node = &mut nodes[sample["node"].as_u64().unwrap() as usize];
        match sample["field"].as_str().unwrap() {
            "opacity" => node.opacity = value,
            "curve" => node.segments = vec![PathSegment::Move { to:[value,0.0] }],
            "gradient" => node.fill = Some(FillStyle::Solid { color:[value,0.0,0.0,1.0] }),
            "stroke" => node.stroke.as_mut().unwrap().width = value,
            "text" => node.text.as_mut().unwrap().size = value,
            "image" => node.image.as_mut().unwrap().width = value,
            field => panic!("unknown numeric fixture field {field}"),
        }
        assert!(drawing_scene_to_svg(&nodes,[-20.0,-10.0,200.0,100.0]).unwrap_err().contains("finite"));
    }
}

#[test]
fn svg_preserves_shared_isolated_compositing_hierarchies() {
    fn visit(element:&SvgElement,groups:&mut Vec<String>,actual:&mut std::collections::BTreeMap<String,Vec<String>>) {
        match element {
            SvgElement::Svg {children,..}=>{for child in children {visit(child,groups,actual);}},
            SvgElement::Group {common,children}=>{
                let group=common.extra_attrs.iter().find(|attr|attr.name=="data-group-id");
                if let Some(group)=group {groups.push(group.value.clone());assert!(common.presentation.opacity.is_some());}
                if let Some(layer)=common.extra_attrs.iter().find(|attr|attr.name=="data-layer-id") {actual.insert(layer.value.clone(),groups.clone());}
                for child in children {visit(child,groups,actual);}
                if group.is_some() {groups.pop();}
            },
            _=>{},
        }
    }
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🧬️schema/🎬️scene/🧩️compositing/🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let nodes:Vec<DrawingSceneNode>=dsl::json::from_json_str(&case["nodes"].to_string()).unwrap();
        let output=drawing_scene_to_svg(&nodes,[0.0,0.0,24.0,16.0]).unwrap();
        let root=svg_document_to_typed(&parse_svg_xml(&output).unwrap()).unwrap();
        let mut actual=std::collections::BTreeMap::new();visit(&root,&mut Vec::new(),&mut actual);
        let expected=nodes.iter().map(|node|(node.id.clone(),node.groups.iter().map(|group|group.id.clone()).collect::<Vec<_>>())).collect::<std::collections::BTreeMap<_,_>>();
        assert_eq!(actual,expected,"{}",case["name"]);
    }
}

#[test]
fn drawing_projection_keeps_isolation_without_changing_leaf_opacity() {
    let first=crate::schema::create_drawing_shape_layer_rect("First");
    let second=crate::schema::create_drawing_shape_layer_rect("Second");
    let mut blend=crate::schema::create_drawing_group_layer("Blend");
    let crate::DrawingLayerNode::Group(body)=&mut blend else {unreachable!()};
    body.base.id="blend".into();body.base.blend_mode="screen".into();body.children=vec![second];
    let mut half=crate::schema::create_drawing_group_layer("Half");
    let crate::DrawingLayerNode::Group(body)=&mut half else {unreachable!()};
    body.base.id="half".into();body.base.opacity=0.5;body.base.transform.x=5.0;body.children=vec![first,blend];
    let mut outer=crate::schema::create_drawing_group_layer("Outer");
    let crate::DrawingLayerNode::Group(body)=&mut outer else {unreachable!()};
    body.base.transform.x=10.0;body.children=vec![half];
    let document=DrawingSnapshot {layers:vec![outer],..Default::default()};
    let nodes=flatten_drawing_document_to_scene_nodes(&document);
    assert_eq!(nodes.len(),2);
    assert_eq!(nodes[0].groups.iter().map(|g|g.id.as_str()).collect::<Vec<_>>(),vec!["half"]);
    assert_eq!(nodes[1].groups.iter().map(|g|g.id.as_str()).collect::<Vec<_>>(),vec!["half","blend"]);
    for node in &nodes {assert_eq!(node.opacity,1.0);assert_eq!(node.transform[4],15.0);assert_eq!(node.groups[0].opacity,0.5);}
}

#[test]
fn explicit_isolation_survives_unit_opacity_scene_projection() {
    let mut group=crate::schema::create_drawing_group_layer("Isolated");
    let crate::DrawingLayerNode::Group(body)=&mut group else {unreachable!()};
    body.isolation=true;body.base.id="isolated".into();body.children.push(crate::schema::create_drawing_shape_layer_rect("Child"));
    let mut document=DrawingSnapshot {layers:vec![group],..Default::default()};
    let nodes=flatten_drawing_document_to_scene_nodes(&document);
    assert_eq!(nodes[0].groups.len(),1);assert_eq!(nodes[0].groups[0].id,"isolated");assert_eq!(nodes[0].groups[0].opacity,1.0);
    let crate::DrawingLayerNode::Group(body)=&mut document.layers[0] else {unreachable!()};body.isolation=false;
    assert!(flatten_drawing_document_to_scene_nodes(&document)[0].groups.is_empty());
}
