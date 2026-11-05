//! 🔁️ Persistent array projections preserve sequence order and require genuine native receiving.
use super::*;
use semio_framework_value_intrinsic::{FromValue,ToValue,DslValue,ValueShape};

#[test]
fn persistent_array_wire_matches_the_independent_vector_oracle(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for expected in fixture["cases"].as_array().unwrap(){
        let rows:Vec<i64>=serde_json::from_value(expected.clone()).unwrap();let _bank=super::tests::wallet::case(rows.len().max(1));
        let root=if rows.is_empty(){PersistentList::default()}else{super::tests::list(rows.len())};
        assert_eq!(root.to_value(),rows.to_value());assert_eq!(root.value_shape_at_path(&[]).unwrap(),ValueShape::Array {len:rows.len()});
        for (index,value) in rows.iter().enumerate(){assert_eq!(&root[index],value);assert_eq!(root.value_at_path(&[&index.to_string()]).unwrap(),value.to_value());}
        assert!(root.value_at_path(&[&rows.len().to_string()]).is_err());
        assert!(<PersistentList<i64> as FromValue>::from_value(rows.to_value()).is_err());
        let mut source=super::tests::original(root);super::tests::close_source(&mut source);
    }
    for invalid in fixture["invalid"].as_array().unwrap(){let value=match invalid{serde_json::Value::Null=>DslValue::Null,serde_json::Value::Object(_)=>DslValue::Object(vec![]),serde_json::Value::Bool(value)=>value.to_value(),serde_json::Value::Number(value)=>value.as_i64().unwrap().to_value(),serde_json::Value::String(value)=>value.to_value(),_=>panic!("invalid scalar fixture")};assert!(<PersistentList<i64> as FromValue>::from_value(value).is_err());}
}
