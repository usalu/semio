//! 🛬️ Fixtures retain one authentic decoder and fund native failure owners through original closure.
use super::*;
use semio_framework_value::{DslValue,ToValue,persistent::PersistentList};

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn policy()->RetainedCloneGrant{let p=fixture();RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap() as usize}}
fn native(value:&serde_json::Value)->DslValue{match value{serde_json::Value::Null=>DslValue::Null,serde_json::Value::Bool(value)=>value.to_value(),serde_json::Value::Number(value)=>if let Some(value)=value.as_i64(){value.to_value()}else{value.as_f64().unwrap().to_value()},serde_json::Value::String(value)=>value.to_value(),serde_json::Value::Array(rows)=>DslValue::Array(rows.iter().map(native).collect()),serde_json::Value::Object(rows)=>DslValue::Object(rows.iter().map(|(key,value)|(key.clone(),native(value))).collect())}}

#[test]
fn original_native_fixture_array_receiving_matches_serde_rows(){
    let fixture=fixture();for row in fixture["valid"].as_array().unwrap(){let mut caller=OriginalWorkFixture::with_source(OperationId(995),Generation(17),policy(),||Some(1),u64::MAX,1<<24);let expected:Vec<i64>=serde_json::from_value(row.clone()).unwrap();let list:PersistentList<i64>=caller.try_receive_native(native(row),policy()).unwrap();assert_eq!(list.iter().copied().collect::<Vec<_>>(),expected);assert_eq!(list.to_value(),expected.to_value());caller.retire_owned(list,policy());caller.finish();}
}

#[test]
fn original_native_fixture_refuses_invalid_rows_and_closes_partial_owners(){
    let fixture=fixture();for row in fixture["invalid"].as_array().unwrap(){let mut caller=OriginalWorkFixture::with_source(OperationId(996),Generation(17),policy(),||Some(1),u64::MAX,1<<24);let result=caller.try_receive_native::<DslValue,PersistentList<i64>>(native(row),policy());let error=match result{Ok(value)=>{caller.retire_owned(value,policy());panic!("invalid native fixture was accepted: {row}")},Err(error)=>error};assert!(error.primary_receiving().is_some());caller.retire_owned(error,policy());let list:PersistentList<i64>=caller.receive_native(vec![7i64].to_value(),policy());assert_eq!(list.get(0),Some(&7));caller.retire_owned(list,policy());caller.finish();}
}
