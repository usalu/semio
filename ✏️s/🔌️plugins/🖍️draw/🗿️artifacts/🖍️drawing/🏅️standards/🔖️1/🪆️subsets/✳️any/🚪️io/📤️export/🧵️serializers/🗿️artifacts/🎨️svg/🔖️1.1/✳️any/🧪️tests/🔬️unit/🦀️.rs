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
