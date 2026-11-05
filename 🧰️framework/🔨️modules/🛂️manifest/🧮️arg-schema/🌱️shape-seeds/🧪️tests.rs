//! 🧪️ Static shape seeds obey explicit defaults, requiredness and independently selected variant groups.
use super::*;
use crate::{ActionArgOption,LocalizedLabel};
fn text(id:&str)->ActionArgDef{ActionArgDef::text(id,LocalizedLabel::native(id,id)).required()}
#[test]
fn shape_seed_keeps_only_the_first_selected_variant_and_explicit_defaults(){
 let mut selector=ActionArgDef::select("/kind",LocalizedLabel::native("Kind","Art"),vec![ActionArgOption{value:"solid".into(),label:LocalizedLabel::native("Solid","Voll")},ActionArgOption{value:"tube".into(),label:LocalizedLabel::native("Tube","Rohr")}]).required();
 selector.default=Some(DslValue::String("tube".into()));
 let mut solid=text("/radius");solid.group=Some("solid".into());
 let mut tube=text("/wall");tube.group=Some("tube".into());tube.default=Some(DslValue::String("retained".into()));
 let optional=ActionArgDef::text("/optional",LocalizedLabel::native("Optional","Optional"));
 let input=ActionArgDef::object("/item",LocalizedLabel::native("Item","Eintrag"),vec![selector,text("/a~1b"),solid,tube,optional]).required();
 let expected=serde_json::json!({"kind":"tube","a/b":"","wall":"retained"});
 assert_eq!(serde_json::to_value(action_arg_shape_seed(&input)).unwrap(),expected);
 let seeds=action_arg_shape_seeds(&[input.clone()]);
 validate_action_arg_shape_seeds(&[input],&seeds).unwrap();
 assert_eq!(seeds.len(),6);assert_eq!(seeds[0].variant_selector,Some(0));assert!(seeds.iter().skip(1).all(|seed|seed.variant_selector.is_none()));assert_eq!(seeds[0].schema_path,vec![0]);assert_eq!(serde_json::to_value(&seeds[0].value).unwrap(),expected);
 assert_eq!(seeds[2].schema_path,vec![0,1]);assert_eq!(seeds[2].key.as_deref(),Some("a/b"));assert_eq!(seeds[0].children,vec![1,2,3,4,5]);
}
#[test]
fn shape_seed_respects_array_item_paths_numeric_rules_and_whole_input_defaults(){
 let schema=ArgSchema::Array{items:Box::new(ArgSchema::number(Some(2.25),None,None,true)),min_items:Some(2),max_items:None};
 let mut input=ActionArgDef::with_schema("/items",LocalizedLabel::native("Items","Einträge"),schema);
 assert_eq!(serde_json::to_value(arg_schema_shape_seed(&input.schema)).unwrap(),serde_json::json!([3,3]));
 input.default=Some(DslValue::Array(vec![DslValue::int(7)]));
 let seeds=action_arg_shape_seeds(&[input.clone()]);
 validate_action_arg_shape_seeds(&[input],&seeds).unwrap();assert_eq!(seeds.len(),2);assert_eq!(serde_json::to_value(&seeds[0].value).unwrap(),serde_json::json!([7]));
 assert_eq!(seeds[1].schema_path,vec![0,0]);assert_eq!(serde_json::to_value(&seeds[1].value).unwrap(),serde_json::json!(3));
}

#[test]
fn ordinary_layout_groups_are_not_variant_groups(){
 let selector=ActionArgDef::select("/mode",LocalizedLabel::native("Mode","Modus"),vec![ActionArgOption{value:"solid".into(),label:LocalizedLabel::native("Solid","Voll")}]).required();
 let mut grouped=text("/length");grouped.group=Some("dimensions".into());
 assert!(action_arg_variant_selector(&[selector.clone(),grouped.clone()]).is_none());
 let schema=ArgSchema::Object{fields:vec![selector,grouped]};
 assert!(action_arg_shape_seeds(&[ActionArgDef::with_schema("/record",LocalizedLabel::native("Record","Datensatz"),schema.clone())])[0].variant_selector.is_none());
 let seed=arg_schema_shape_seed(&schema);
 assert_eq!(serde_json::to_value(seed).unwrap(),serde_json::json!({"mode":"solid","length":""}));
}

#[test]
fn shape_seed_matches_language_neutral_scalar_array_and_number_cases(){
 let law:serde_json::Value=serde_json::from_str(include_str!("seed-cases.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let schema:ArgSchema=serde_json::from_value(row["schema"].clone()).unwrap();
  let value=serde_json::to_value(arg_schema_shape_seed(&schema)).unwrap();
  assert_eq!(value,row["expected"],"{}",row["name"]);
 }
}

#[test]
fn seed_emission_refuses_cycles_duplicate_edges_and_misaligned_keys_or_selectors(){
 let input=ActionArgDef::object("/item",LocalizedLabel::native("Item","Eintrag"),vec![text("/a"),text("/b")]);
 let original=action_arg_shape_seeds(&[input.clone()]);validate_action_arg_shape_seeds(&[input.clone()],&original).unwrap();
 for change in 0..6{
  let mut rows=original.clone();
  match change{0=>rows[0].children[0]=0,1=>rows[0].children[1]=1,2=>rows[0].children[0]=99,3=>rows[1].key=Some("wrong".into()),4=>rows[1].schema_path=vec![0,9],_=>rows[0].variant_selector=Some(1)}
  assert!(validate_action_arg_shape_seeds(&[input.clone()],&rows).is_err());
 }
}

#[test]
fn singleton_seeds_are_exactly_one_item_and_preserve_the_separate_explicit_array_default(){
 let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/singleton-cases.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let schema:ArgSchema=serde_json::from_value(row["schema"].clone()).unwrap();
  let mut input=ActionArgDef::with_schema("/items",LocalizedLabel::native("Items","Einträge"),schema);
  input.default=row.get("default").map(|value|serde_json::from_value(value.clone()).unwrap());
  let rows=action_arg_shape_seeds(&[input.clone()]);validate_action_arg_shape_seeds(&[input.clone()],&rows).unwrap();
  assert_eq!(serde_json::to_value(&rows[0].value).unwrap(),row["expectedSeed"]);
  assert_eq!(serde_json::to_value(&rows[0].singleton).unwrap(),row["singleton"]);
  if rows[0].singleton.is_some(){
   for replacement in [None,Some(DslValue::Array(vec![])),Some(DslValue::Array(vec![DslValue::Null,DslValue::Null]))]{
    let mut changed=rows.clone();changed[0].singleton=replacement;assert!(validate_action_arg_shape_seeds(&[input.clone()],&changed).is_err());
   }
  }else{
   let mut changed=rows;changed[0].singleton=Some(DslValue::Array(vec![DslValue::Null]));assert!(validate_action_arg_shape_seeds(&[input],&changed).is_err());
  }
 }
}
