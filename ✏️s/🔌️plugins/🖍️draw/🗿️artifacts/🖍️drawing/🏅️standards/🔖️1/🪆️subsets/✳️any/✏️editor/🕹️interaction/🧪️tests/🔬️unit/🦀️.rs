use super::*;

fn layer(value: &serde_json::Value) -> DrawingLayerNode {
    let kind = value["kind"].as_str().unwrap();
    let id = value["base"]["id"].as_str().unwrap();
    let mut node = crate::schema::create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit(((if kind == "shape" { "shape:rect" } else { kind })).to_string().into()).expect("nonempty authored identity"), if kind == "shape" { "shape:rect" } else { kind });
    let base = crate::schema::layer_base_mut(&mut node);
    base.id = id.into();
    base.visible = value["base"]["visible"].as_bool().unwrap_or(true);
    base.locked = value["base"]["locked"].as_bool().unwrap_or(false);
    match &mut node {
        DrawingLayerNode::Group(group) => group.children = value["children"].as_array().unwrap().iter().map(layer).collect(),
        DrawingLayerNode::Path(path) => path.segments = value.get("segments").map(|segments|serde_json::from_value(segments.clone()).unwrap()).unwrap_or_default(),
        DrawingLayerNode::Boolean(boolean) => boolean.children = value["children"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().into()).collect(),
        _ => {}
    }
    node
}

#[test]
fn document_topology_matches_language_neutral_fixture() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let snapshot = DrawingSnapshot { layers: case["layers"].as_array().unwrap().iter().map(layer).collect(), ..Default::default() };
        let topology = drawing_interaction_topology(&snapshot);
        let actual = topology.ordered.iter().map(|node| {
            let mut value = serde_json::json!({ "id": node.id, "granularity": node.granularity });
            if let Some(parent) = &node.parent { value["parent"] = parent.clone().into(); }
            value
        }).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(actual), case["ordered"], "{}", case["name"]);
    }
}

#[test]
fn point_topology_obeys_ancestor_restrictions_and_actual_handles() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🎯️points/🧫️fixtures/🌳️topology/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let snapshot=DrawingSnapshot {layers:row["layers"].as_array().unwrap().iter().map(layer).collect(),..Default::default()};
        let actual=drawing_point_topology(&snapshot).ordered.into_iter().map(|node| {
            let point=points::parse_point_id(&node.id).unwrap();
            assert_eq!(node.granularity,"point");assert!(node.parent.is_none());
            serde_json::json!([point.layer_id,point.index,points::point_name(point.point)])
        }).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(actual),row["points"]);
    }
}
