//! 🧪️ Native scalar outcomes share the exact closed Source and GraphQL oracle corpus.
use super::{parse_int64_literal,parse_int64_variable,serialize_int64,Int64Error};
use crate::{DslValue,Number};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn graphql_int64_neutral_complete_signed_words(){
 let fixture=fixture();let cases=fixture["valid"].as_array().unwrap();assert_eq!(cases.len(),9);
 for case in cases{
  let text=case["variable"].as_str().unwrap();let expected=case["output"].as_str().unwrap().parse::<i64>().unwrap();
  assert_eq!(parse_int64_variable(&DslValue::String(text.into())),Ok(expected));
  for key in ["ast","stringAst"]{assert_eq!(parse_int64_literal(case[key]["kind"].as_str().unwrap(),case[key]["value"].as_str()),Ok(expected));}
  assert_eq!(serialize_int64(expected),case["output"].as_str().unwrap());
 }
}
#[test]
fn graphql_int64_neutral_canonical_and_category_refusals(){
 let fixture=fixture();
 for text in fixture["invalidDecimal"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()){
  assert!(parse_int64_variable(&DslValue::String(text.into())).is_err());
  assert!(parse_int64_literal("IntValue",Some(text)).is_err());
 }
 for value in fixture["invalidVariableKinds"].as_array().unwrap(){
  let value=match value{serde_json::Value::Null=>DslValue::Null,serde_json::Value::Bool(v)=>DslValue::Bool(*v),serde_json::Value::Number(v)=>DslValue::Number(v.as_i64().map(Number::Int).unwrap_or_else(||Number::Float(v.as_f64().unwrap()))),serde_json::Value::Array(_)=>DslValue::Array(Vec::new()),serde_json::Value::Object(_)=>DslValue::Object(Vec::new()),serde_json::Value::String(_)=>panic!("nontext authority differs")};
  assert_eq!(parse_int64_variable(&value),Err(Int64Error::Type));
 }
 for kind in fixture["invalidAstKinds"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()){assert_eq!(parse_int64_literal(kind,Some("1")),Err(Int64Error::Literal));}
 assert_eq!(parse_int64_literal("IntValue",None),Err(Int64Error::Type));
 assert!(parse_int64_variable(&DslValue::String("1".repeat(8194))).is_err());
}
