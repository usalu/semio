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
        let mut document=DrawingSnapshot { layers:vec![crate::schema::create_drawing_shape_layer_rect("Behind"),layer,crate::schema::create_drawing_shape_layer_rect("Ahead")].into(),..Default::default() };
        let order=document.layers.iter().map(|layer|layer_base(layer).id.clone()).collect::<Vec<_>>();
        let mutations=plan(&document,&["converted".into()],"toPath").unwrap();
        assert_eq!(mutations.len(),2);
        for mutation in mutations { crate::mutations::apply_drawing_mutation(&mut document,&mutation).unwrap(); }
        assert_eq!(document.layers.iter().map(|layer|layer_base(layer).id.clone()).collect::<Vec<_>>(),order);
        let DrawingLayerNode::Path(path)=&document.layers[1] else { panic!("Expected path") };
        assert_eq!(path.base,original);
        assert_eq!(path.segments,serde_json::from_value::<Vec<crate::PathSegment>>(case["segments"].clone()).unwrap().into());
        assert!(plan(&document,&["converted".into()],"toPath").unwrap().is_empty());
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
    let original=DrawingSnapshot { layers:vec![a,group].into(),..Default::default() };
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
                let actual = crate::schema::drawing_layer_world_bounds(layer).unwrap();
                let expected: (f64,f64,f64,f64) = serde_json::from_value(expected.clone()).unwrap();
                assert_eq!(actual, expected, "{case}");
            }
        }
    }
}

#[test]
fn repeated_duplication_preserves_unique_descendant_ids() {
    let child = crate::schema::create_drawing_shape_layer_rect("Child");
    let mut group = crate::schema::create_drawing_group_layer("Group");
    if let DrawingLayerNode::Group(body) = &mut group { body.children.push(child); }
    let id = layer_base(&group).id.clone();
    let mut document = DrawingSnapshot { layers: vec![group].into(), ..Default::default() };
    for _ in 0..2 {
        for mutation in plan(&document, &[id.clone()], "duplicate").unwrap() { crate::mutations::apply_drawing_mutation(&mut document, &mutation).unwrap(); }
    }
    let ids = document.layers.iter().flat_map(|layer| match layer { DrawingLayerNode::Group(group) => vec![group.base.id.clone(), layer_base(&group.children[0]).id.clone()], _ => unreachable!() }).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 6);
}

#[test]
fn arrangement_preserves_ancestor_coordinates_and_atomicity() {
    fn node(value:&serde_json::Value)->DrawingLayerNode {
        let id=value["id"].as_str().unwrap();
        let mut layer=if let Some(children)=value["children"].as_array() {
            let mut layer=crate::schema::create_drawing_group_layer(id);
            let DrawingLayerNode::Group(group)=&mut layer else {unreachable!()};
            group.children=children.iter().map(node).collect();layer
        }else {
            let mut layer=crate::schema::create_drawing_shape_layer_rect(id);
            let DrawingLayerNode::Shape(shape)=&mut layer else {unreachable!()};
            let [x,y,width,height]:[f64;4]=serde_json::from_value(value["rect"].clone()).unwrap();
            shape.rect=Some(crate::DrawingRect {x,y,width,height});layer
        };
        let base=layer_base_mut(&mut layer);base.id=id.into();
        base.locked=value["locked"].as_bool().unwrap_or(false);
        let t=&value["transform"];
        base.transform=crate::DrawingTransform {x:t["x"].as_f64().unwrap_or(0.0),y:t["y"].as_f64().unwrap_or(0.0),scale_x:t["scaleX"].as_f64().unwrap_or(1.0),scale_y:t["scaleY"].as_f64().unwrap_or(1.0),rotation:t["rotation"].as_f64().unwrap_or(0.0),shear:t["shear"].as_f64().unwrap_or(0.0)};
        layer
    }
    fn visit(layers:&[DrawingLayerNode],parent:[f64;6],actual:&mut std::collections::BTreeMap<String,[f64;4]>) {
        for layer in layers {
            let base=layer_base(layer);
            if let DrawingLayerNode::Group(group)=layer {
                visit(&group.children,crate::schema::geometry::multiply(parent,crate::schema::drawing_transform_to_matrix(&base.transform)),actual);
            }else {
                let (x,y,width,height)=crate::schema::drawing_layer_bounds_with_parent(layer,parent).unwrap();
                actual.insert(base.id.clone(),[x,y,width,height]);
            }
        }
    }
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🌍️arrangement/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let document=DrawingSnapshot {layers:case["nodes"].as_array().unwrap().iter().map(node).collect(),..Default::default()};
        let saved=document.clone();
        let ids=serde_json::from_value::<Vec<String>>(case["ids"].clone()).unwrap();
        let result=plan(&document,&ids,case["operation"].as_str().unwrap());
        assert_eq!(document,saved);
        if case["after"].is_null() {assert!(result.is_err(),"{}",case["name"]);continue;}
        let mutations=result.unwrap();
        let mut output=document.clone();
        for mutation in mutations {crate::mutations::apply_drawing_mutation(&mut output,&mutation).unwrap();}
        let mut actual=std::collections::BTreeMap::new();visit(&output.layers,[1.0,0.0,0.0,1.0,0.0,0.0],&mut actual);
        let expected:std::collections::BTreeMap<String,[f64;4]>=serde_json::from_value(case["after"].clone()).unwrap();
        assert_eq!(actual.len(),expected.len());
        for (id,bounds) in expected {for (a,b) in actual[&id].iter().zip(bounds) {assert!((a-b).abs()<1e-9,"{} {id}: {a} != {b}",case["name"]);}}
    }
}

#[test]
fn layer_stack_steps_match_shared_order_in_root_and_group() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🗂️stack/🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {for nested in [false,true] {
        let layers=case["order"].as_array().unwrap().iter().map(|id| {
            let mut layer=crate::schema::create_drawing_shape_layer_rect(id.as_str().unwrap());
            layer_base_mut(&mut layer).id=id.as_str().unwrap().into();layer
        }).collect::<Vec<_>>();
        let layers=if nested {
            let mut layer=crate::schema::create_drawing_group_layer("Parent");
            let DrawingLayerNode::Group(group)=&mut layer else {unreachable!()};
            group.base.id="parent".into();group.children=layers.into();vec![layer]
        }else {layers};
        let document=DrawingSnapshot {layers,..Default::default()};
        let saved=document.clone();
        let ids:Vec<String>=serde_json::from_value(case["ids"].clone()).unwrap();
        let mutations=plan(&document,&ids,case["operation"].as_str().unwrap()).unwrap();
        assert_eq!(document,saved);
        if case["order"]==case["after"] {assert!(mutations.is_empty(),"{}",case["name"]);}
        let mut output=document.clone();
        for mutation in mutations {crate::mutations::apply_drawing_mutation(&mut output,&mutation).unwrap();}
        let layers=if nested {let DrawingLayerNode::Group(group)=&output.layers[0] else {unreachable!()}; &group.children}else {&output.layers};
        assert_eq!(layers.iter().map(|layer|layer_base(layer).id.as_str()).collect::<Vec<_>>(),case["after"].as_array().unwrap().iter().map(|id|id.as_str().unwrap()).collect::<Vec<_>>(),"{}",case["name"]);
    }}
}

#[test]
fn ungroup_matches_shared_structure_transform_and_selection_cases() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧩️ungroup/🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let document=DrawingSnapshot {layers:serde_json::from_value(case["before"].clone()).unwrap(),..Default::default()};
        let saved=document.clone();
        let ids:Vec<String>=serde_json::from_value(case["ids"].clone()).unwrap();
        let result=ungroup::plan(&document,&ids);
        assert_eq!(document,saved);
        if case["error"]==true {assert!(result.is_err(),"{}",case["name"]);continue;}
        let (mutations,selection)=result.unwrap();
        assert_eq!(selection,serde_json::from_value::<Vec<String>>(case["selection"].clone()).unwrap());
        let mut output=document.clone();
        for mutation in mutations {crate::mutations::apply_drawing_mutation(&mut output,&mutation).unwrap();}
        assert_eq!(output.layers.iter().map(|layer|layer_base(layer).id.as_str()).collect::<Vec<_>>(),case["order"].as_array().unwrap().iter().map(|id|id.as_str().unwrap()).collect::<Vec<_>>());
        for (id,expected) in case["matrices"].as_object().unwrap() {
            let base=layer_base(find_drawing_layer(&output,id).unwrap());
            let matrix=crate::schema::drawing_transform_to_matrix(&base.transform);
            let expected:[f64;6]=serde_json::from_value(expected.clone()).unwrap();
            for (a,b) in matrix.iter().zip(expected) {assert!((a-b).abs()<1e-9,"{} {id}: {a} != {b}",case["name"]);}
            assert_eq!(base.visible,!case["hidden"].as_array().unwrap().iter().any(|value|value==id));
        }
        if case["name"]=="reflected-sheared-group" {assert!(layer_base(find_drawing_layer(&output,"a").unwrap()).locked);}
    }
}

#[test]
fn ungroup_refuses_explicit_isolation_even_at_normal_blend_and_unit_opacity() {
    let mut layer=crate::schema::create_drawing_group_layer("Isolated");
    let crate::DrawingLayerNode::Group(group)=&mut layer else {unreachable!()};group.isolation=true;group.base.id="group".into();
    let document=crate::DrawingSnapshot {layers:vec![layer].into(),..Default::default()};
    assert!(super::plan(&document,&["group".into()],"ungroup").is_err());
}
