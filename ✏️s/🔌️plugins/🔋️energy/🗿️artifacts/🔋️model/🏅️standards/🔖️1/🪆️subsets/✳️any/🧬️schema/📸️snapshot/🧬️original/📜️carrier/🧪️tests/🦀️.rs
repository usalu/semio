//! 🧪️ The synchronous carriers reproduce the authored native Pack schema and round-trip the canonical document value.
use super::*;

fn named(name:&str)->EnergyModelSnapshot{EnergyModelSnapshot::from_model(&crate::model::Model{name:name.into(),..Default::default()}).expect("an engine model is an admissible native document")}

#[test]
fn derived_owned_spec_hashes_to_the_authored_native_schema_hash(){
 assert_eq!(pack::record::schema_hash(&EnergyModelDocument::__dsl_spec()),crate::original::pack::SCHEMA_HASH);
}

#[test]
fn text_carrier_round_trips_to_an_equal_snapshot_and_a_print_fixpoint(){
 let snapshot=named("carrier");
 let text=snapshot.print_dsl();
 let parsed=EnergyModelSnapshot::parse_dsl(&text).expect("canonical text parses");
 assert_eq!(parsed,snapshot);
 assert_eq!(parsed.print_dsl(),text);
 assert!(text.starts_with("semio energy.model.dsl v1"));
}

#[test]
fn pack_carrier_round_trips_to_an_equal_snapshot_and_stable_bytes(){
 let snapshot=named("carrier");
 let bytes=snapshot.encode_pack();
 let decoded=EnergyModelSnapshot::decode_pack(&bytes).expect("canonical pack decodes");
 assert_eq!(decoded,snapshot);
 assert_eq!(decoded.encode_pack(),bytes);
}

#[test]
fn default_is_the_canonical_empty_engine_model(){
 assert_eq!(EnergyModelSnapshot::default(),EnergyModelSnapshot::from_model(&crate::model::Model::default()).unwrap());
}

#[test]
fn document_value_agrees_with_an_independent_json_reader(){
 let json=semio_framework_pack_json::to_json_string(&EnergyModelSnapshot::document_value(&crate::model::Model{name:"oracle".into(),..Default::default()}));
 let parsed:serde_json::Value=serde_json::from_str(&json).expect("canonical json");
 assert_eq!(parsed["schema"],"energy.model");
 assert_eq!(parsed["model"]["name"],"oracle");
 assert!(parsed["referencedModel"].is_null()&&parsed["weatherLink"].is_null());
 let projected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&named("oracle"))).expect("canonical json");
 assert_eq!(projected,parsed);
}

#[test]
fn a_wire_value_outside_the_schema_is_refused_without_a_leaked_owner(){
 assert!(EnergyModelSnapshot::from_document(DslValue::Null).is_err());
}
