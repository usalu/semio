//! 🧪️ Language-neutral node editing and source-preservation laws.
use super::*;

fn assert_geometry(actual: &serde_json::Value, expected: &serde_json::Value) {
    match (actual, expected) {
        (serde_json::Value::Number(a), serde_json::Value::Number(b)) => assert!((a.as_f64().unwrap()-b.as_f64().unwrap()).abs()<1e-10),
        (serde_json::Value::Array(a), serde_json::Value::Array(b)) => { assert_eq!(a.len(),b.len()); for (a,b) in a.iter().zip(b) { assert_geometry(a,b); } }
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => { assert_eq!(a.len(),b.len()); for (key,value) in a { assert_geometry(value,&b[key]); } }
        _ => assert_eq!(actual,expected),
    }
}

#[test]
fn path_node_editing_shared_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let source: Vec<PathSegment> = serde_json::from_value(case["before"].clone()).unwrap();
        let operation: PathEdit = serde_json::from_value(case["operation"].clone()).unwrap();
        let saved = source.clone();
        let result = edit_path(&source, &operation);
        assert_eq!(source, saved);
        if case["error"] == true { assert!(result.is_err(), "{}", case["name"]); continue; }
        let expected: Vec<PathSegment> = serde_json::from_value(case["after"].clone()).unwrap();
        let actual = result.unwrap();
        assert_geometry(&serde_json::to_value(&actual).unwrap(), &serde_json::to_value(&expected).unwrap());
        assert_eq!(edit_path(&edit_path(&actual, &PathEdit::Reverse).unwrap(), &PathEdit::Reverse).unwrap(), actual);
    }
}

#[test]
fn arc_segment_extrema_shared_cases() {
    let cases: serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🔄️arc-bounds/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let segment: PathSegment=serde_json::from_value(case["segment"].clone()).unwrap();
        let from=serde_json::from_value(case["from"].clone()).unwrap();
        let matrix=serde_json::from_value(case["matrix"].clone()).unwrap();
        let bounds=super::super::segment_bounds(&segment,from,from,matrix);
        assert_geometry(&serde_json::to_value(bounds).unwrap(),&case["bounds"]);
    }
}

#[test]
fn pointer_drag_uses_full_affine_ancestors_and_preserves_press_offset() {
    let fixtures:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖱️drag/🔣️.json")).unwrap();
    for row in fixtures.as_array().unwrap() {
        let segment=serde_json::from_value(row["segment"].clone()).unwrap();
        let point=serde_json::from_value(row["point"].clone()).unwrap();
        let result=drag_path_point(&segment,point,serde_json::from_value(row["matrix"].clone()).unwrap(),serde_json::from_value(row["start"].clone()).unwrap(),serde_json::from_value(row["end"].clone()).unwrap(),row["constrained"].as_bool().unwrap());
        if row.get("error").is_some(){assert!(result.is_none());continue;}
        let expected:[f64;2]=serde_json::from_value(row["to"].clone()).unwrap();
        let actual=result.unwrap();
        for axis in 0..2 {assert!((actual[axis]-expected[axis]).abs()<1e-10,"{}",row["name"]);}
    }
}

#[test]
fn gesture_position_preview_copies_only_adjacent_segments() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap().iter().filter(|row|row["name"].as_str().unwrap().starts_with("position-") && row["error"]!=true) {
        let source:Vec<PathSegment>=serde_json::from_value(row["before"].clone()).unwrap();
        let operation:PathEdit=serde_json::from_value(row["operation"].clone()).unwrap();
        let PathEdit::Position {index,point,to}=operation else {panic!("Expected position fixture")};
        let saved=source.clone();
        let (segment,next)=patch_path_point(&source[index],source.get(index+1),point,to).unwrap();
        let mut preview=source.clone();preview[index]=segment;
        if let Some(next)=next {preview[index+1]=next;}
        assert_geometry(&serde_json::to_value(&preview).unwrap(),&row["after"]);
        assert_eq!(preview,edit_path(&source,&operation).unwrap());
        assert_eq!(source,saved);
        assert!(patch_path_point(&source[index],source.get(index+1),point,[f64::INFINITY,0.0]).is_err());
    }
    assert!(patch_path_point(&PathSegment::Close,None,PathPoint::Anchor,[0.0,0.0]).is_err());
    assert!(patch_path_point(&PathSegment::Line {to:[0.0,0.0]},None,PathPoint::Control1,[0.0,0.0]).is_err());
}

#[test]
fn node_picking_names_nearest_point_with_anchor_priority() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️point-hit/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let segment=serde_json::from_value(row["segment"].clone()).unwrap();
        let matrix=serde_json::from_value(row["matrix"].clone()).unwrap();
        let world=serde_json::from_value(row["world"].clone()).unwrap();
        let result=path_point_hit(&segment,matrix,world,row["tolerance"].as_f64().unwrap());
        assert_eq!(serde_json::to_value(result.map(|(point,_)|point)).unwrap(),row["point"],"{}",row["name"]);
        assert!(path_point_hit(&segment,matrix,world,-1.0).is_none());
    }
}

#[test]
fn multi_point_translation_matches_shared_cases_atomically() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/↔️points/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let source:Vec<PathSegment>=serde_json::from_value(row["before"].clone()).unwrap();
        let saved=source.clone();
        let operation:PathEdit=serde_json::from_value(row["operation"].clone()).unwrap();
        let actual=edit_path(&source,&operation);
        assert_eq!(source,saved);
        if row["error"]==true {assert!(actual.is_err(),"{}",row["name"]);continue;}
        let actual=actual.unwrap();
        assert_geometry(&serde_json::to_value(&actual).unwrap(),&row["after"]);
        let mut reordered=row["operation"].clone();
        reordered["points"].as_array_mut().unwrap().reverse();
        assert_eq!(edit_path(&source,&serde_json::from_value(reordered).unwrap()).unwrap(),actual);
        let mut reversed=row["operation"].clone();
        for value in reversed["delta"].as_array_mut().unwrap() {*value=(-value.as_f64().unwrap()).into();}
        assert_eq!(edit_path(&actual,&serde_json::from_value(reversed).unwrap()).unwrap(),source);
        let value=dsl::ToValue::to_value(&operation);
        assert_eq!(<PathEdit as dsl::FromValue>::from_value(value).unwrap(),operation);
    }
}

#[test]
fn world_point_nudges_follow_the_complete_affine_basis() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🌍️translation/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let source:Vec<PathSegment>=serde_json::from_value(row["segments"].clone()).unwrap();
        let saved=source.clone();
        let points=serde_json::from_value::<Vec<PathPointRef>>(row["points"].clone()).unwrap();
        let actual=translate_world_path_points(&source,&points,serde_json::from_value(row["matrix"].clone()).unwrap(),serde_json::from_value(row["delta"].clone()).unwrap());
        if row["after"].is_null() {assert!(actual.is_err());} else {assert_eq!(actual.unwrap(),serde_json::from_value::<Vec<PathSegment>>(row["after"].clone()).unwrap(),"{}",row["name"]);}
        assert_eq!(source,saved);
    }
}
