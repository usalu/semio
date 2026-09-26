//! 🧪️ Selection operations must produce the language-neutral document outcomes.
use super::*;

#[test]
fn shape_conversion_fixtures_preserve_identity_style_transform_and_order() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛤️to-path/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let mut value=serde_json::to_value(crate::schema::create_drawing_shape_layer_rect("Original")).unwrap();
        let kind=case["kind"].as_str().unwrap();
        value["shapeKind"]=case["kind"].clone();
        value[kind]=case["geometry"].clone();
        let mut layer:DrawingLayerNode=serde_json::from_value(value).unwrap();
        layer_base_mut(&mut layer).id="converted".into();
        layer_base_mut(&mut layer).transform.x=23.0;
        layer_base_mut(&mut layer).transform.scale_x=2.0;
        layer_base_mut(&mut layer).opacity=0.4;
        let original=layer_base(&layer).clone();
        let mut document=DrawingSnapshot { layers:vec![crate::schema::create_drawing_shape_layer_rect("Behind"),layer,crate::schema::create_drawing_shape_layer_rect("Ahead")],..Default::default() };
        let order=document.layers.iter().map(|layer|layer_base(layer).id.clone()).collect::<Vec<_>>();
        let mutations=plan(&document,&["converted".into()],"toPath").unwrap();
        assert_eq!(mutations.len(),2);
        for mutation in mutations { crate::mutations::apply_drawing_mutation(&mut document,&mutation).unwrap(); }
        assert_eq!(document.layers.iter().map(|layer|layer_base(layer).id.clone()).collect::<Vec<_>>(),order);
        let DrawingLayerNode::Path(path)=&document.layers[1] else { panic!("Expected path") };
        assert_eq!(path.base,original);
        assert_eq!(path.segments,serde_json::from_value::<Vec<crate::PathSegment>>(case["segments"].clone()).unwrap());
        assert!(plan(&document,&["converted".into()],"toPath").unwrap().is_empty());
        eprintln!("[DEBUG] {kind} conversion retained identity, appearance, transform and stack position");
    }
}

#[test]
fn conversion_preserves_distinct_parents_and_rejects_locked_or_unsupported_selections() {
    let mut a=crate::schema::create_drawing_shape_layer_rect("First");
    layer_base_mut(&mut a).id="a".into();
    let mut b=crate::schema::create_drawing_shape_layer_rect("Second");
    layer_base_mut(&mut b).id="b".into();
    let mut group=crate::schema::create_drawing_group_layer("Nested");
    layer_base_mut(&mut group).id="group".into();
    let DrawingLayerNode::Group(body)=&mut group else { unreachable!() };
    body.children.push(b);
    let original=DrawingSnapshot { layers:vec![a,group],..Default::default() };
    let ids=vec!["a".into(),"b".into()];
    let mut converted=original.clone();
    for mutation in plan(&original,&ids,"toPath").unwrap() { crate::mutations::apply_drawing_mutation(&mut converted,&mutation).unwrap(); }
    for id in &ids {
        assert!(matches!(find_drawing_layer(&converted,id),Some(DrawingLayerNode::Path(_))));
        assert_eq!(find_drawing_layer_location(&original,id).unwrap().parent_id,find_drawing_layer_location(&converted,id).unwrap().parent_id);
    }
    assert!(plan(&original,&["a".into(),"group".into()],"toPath").is_err());
    let mut locked=original.clone();
    layer_base_mut(&mut locked.layers[1]).locked=true;
    assert!(plan(&locked,&ids,"toPath").is_err());
}
#[test]
fn selection_operation_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let layers = fixture["layers"].as_array().unwrap().iter().map(|value| {
        let mut layer = crate::schema::create_drawing_shape_layer_rect(value["id"].as_str().unwrap());
        crate::schema::layer_base_mut(&mut layer).id = value["id"].as_str().unwrap().into();
        if let DrawingLayerNode::Shape(shape) = &mut layer { shape.rect = Some(crate::DrawingRect { x: value["x"].as_f64().unwrap(), y: value["y"].as_f64().unwrap(), width: value["width"].as_f64().unwrap(), height: value["height"].as_f64().unwrap() }); }
        layer
    }).collect();
    let document = DrawingSnapshot { layers, ..Default::default() };
    for case in fixture["cases"].as_array().unwrap() {
        let ids = serde_json::from_value::<Vec<String>>(case["ids"].clone()).unwrap();
        let operations = plan(&document, &ids, case["operation"].as_str().unwrap()).unwrap();
        let mut output = document.clone();
        for operation in operations { crate::mutations::apply_drawing_mutation(&mut output, &operation).unwrap(); }
        if let Some(count) = case["rootCount"].as_u64() { assert_eq!(output.layers.len(), count as usize, "{case}"); }
        if let Some(order) = case["order"].as_array() { assert_eq!(output.layers.iter().map(|layer| layer_base(layer).id.as_str()).collect::<Vec<_>>(), order.iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>(), "{case}"); }
        if let Some(bounds) = case["bounds"].as_array() {
            for (layer, expected) in output.layers.iter().zip(bounds) {
                let actual = drawing_layer_world_bounds(layer).unwrap();
                let expected: (f64,f64,f64,f64) = serde_json::from_value(expected.clone()).unwrap();
                assert_eq!(actual, expected, "{case}");
            }
        }
        eprintln!("[DEBUG] Drawing selection operation {} reached expected document", case["operation"]);
    }
}

#[test]
fn repeated_duplication_preserves_unique_descendant_ids() {
    let child = crate::schema::create_drawing_shape_layer_rect("Child");
    let mut group = crate::schema::create_drawing_group_layer("Group");
    if let DrawingLayerNode::Group(body) = &mut group { body.children.push(child); }
    let id = layer_base(&group).id.clone();
    let mut document = DrawingSnapshot { layers: vec![group], ..Default::default() };
    for _ in 0..2 {
        for mutation in plan(&document, &[id.clone()], "duplicate").unwrap() { crate::mutations::apply_drawing_mutation(&mut document, &mutation).unwrap(); }
    }
    let ids = document.layers.iter().flat_map(|layer| match layer { DrawingLayerNode::Group(group) => vec![group.base.id.clone(), layer_base(&group.children[0]).id.clone()], _ => unreachable!() }).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 6);
}
