use super::*;
#[test]
fn original_expression_names_agree_with_independent_json_oracle(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🖨️printing/📐️names.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let name=row["name"].as_str().unwrap();let actual=print_expr(&ExprValue::Var(name.into()));assert_eq!(actual,row["printed"].as_str().unwrap());assert_eq!(actual,serde_json::to_string(name).unwrap());}
}
