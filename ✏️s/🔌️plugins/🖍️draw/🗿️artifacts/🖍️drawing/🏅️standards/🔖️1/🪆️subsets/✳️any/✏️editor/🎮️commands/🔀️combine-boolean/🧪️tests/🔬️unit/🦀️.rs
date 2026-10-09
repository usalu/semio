//! 🧪️ Boolean selection fixtures prove admission, parent preservation and inverse restoration.
use super::*;
fn document()->DrawingSnapshot {
    let mut a=crate::schema::create_drawing_shape_layer_rect("a");layer_base_mut(&mut a).id="a".into();
    let mut b=crate::schema::create_drawing_shape_layer_rect("b");layer_base_mut(&mut b).id="b".into();
    let mut text=crate::schema::create_layer_by_kind("text");layer_base_mut(&mut text).id="text".into();
    DrawingSnapshot {layers:vec![a,b,text].into(),..Default::default()}
}
#[test]
fn drawing_boolean_command_fixtures() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let before=document();
    for case in fixture["cases"].as_array().unwrap() {
        let ids=serde_json::from_value::<Vec<String>>(case["ids"].clone()).unwrap();
        let planned=plan(&before,&ids,case["operation"].as_str().unwrap());
        assert_eq!(planned.is_ok(),case["accepted"].as_bool().unwrap(),"{case}");
        if let Ok((mutations,selection))=planned {
            let mut after=before.clone();let mut inverses=Vec::new();
            for mutation in &mutations {inverses.push(crate::mutations::inverse_drawing_mutation(&after,mutation).unwrap());crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,mutation).unwrap();}
            let DrawingLayerNode::Boolean(body)=find_drawing_layer(&after,selection.as_str()).unwrap() else {panic!("Expected Boolean result")};
            assert_eq!(body.children.iter().map(|id|id.to_string_owner()).collect::<Vec<_>>(),ids);
            for inverse in inverses.into_iter().rev().flatten() {crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,&inverse).unwrap();}
            assert_eq!(before,after);
        }
    }
}
#[test]
fn drawing_boolean_command_preserves_parent_and_checks_ancestor_admission() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["parentCases"].as_array().unwrap() {
        let mut group=crate::schema::create_drawing_group_layer("parent");layer_base_mut(&mut group).id="parent".into();
        layer_base_mut(&mut group).locked=case["locked"].as_bool().unwrap();layer_base_mut(&mut group).visible=case["visible"].as_bool().unwrap();
        let DrawingLayerNode::Group(body)=&mut group else {unreachable!()};body.children=document().layers;
        let mut before=DrawingSnapshot {layers:vec![group].into(),..Default::default()};
        let planned=plan(&before,&["a".into(),"b".into()],"union");assert_eq!(planned.is_ok(),case["accepted"].as_bool().unwrap());
        if let Ok((mutations,id))=planned {
            for mutation in mutations {crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut before,&mutation).unwrap();}
            assert_eq!(find_drawing_layer_location(&before,id.as_str()).unwrap().parent_id,Some("parent".into()));
            let (mutations,next)=plan(&before,&["a".into(),"b".into()],"union").unwrap();assert_ne!(id,next);assert_eq!(mutations.len(),1);
        }
    }
}
