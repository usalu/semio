//! 🧬️ Generic owners finance native implementations while cold declarations remain unconstrained.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{retained_clone::RetainedClone,retirement::RetireOwned};
#[derive(semio_framework_value_derive::RetireOwned,semio_framework_value_derive::RetainedClone,serde::Serialize,serde::Deserialize)]
struct Node<K,V>{entry:Option<(K,V)>,left:Option<usize>,right:Option<usize>}
fn owns<T:RetainedClone+RetireOwned>(){}
#[test]
fn retained_generic_composite_fields_bound_owned_parameters_without_restricting_cold_declaration(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🧬️generic-owner/🔣️.json")).unwrap();
 let source=serde_json::json!({"entry":law["entry"],"left":law["left"],"right":law["right"]});let native:Node<String,Vec<Box<u32>>>=serde_json::from_value(source.clone()).unwrap();owns::<Node<String,Vec<Box<u32>>>>();assert_eq!(serde_json::to_value(&native).unwrap(),source);
 struct ColdOnly(std::rc::Rc<()>);let cold:Node<ColdOnly,()>=Node{entry:None,left:None,right:None};assert!(cold.entry.is_none());let marker=ColdOnly(std::rc::Rc::new(()));assert_eq!(std::rc::Rc::strong_count(&marker.0),1);
 let mut retirement=semio_framework_value::retirement::controlled::ControlledRetirement::new(native).ok().unwrap();let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:100};for _ in 0..1000{if retirement.terminal_is_empty(){break}retirement.step(grant).unwrap();}assert!(retirement.terminal_is_empty());
}

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
struct ExclusiveCell{close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<u8>>}
#[derive(semio_framework_value_derive::RetireOwned)]
struct SharedPlain{value:u8}
#[test]
fn send_only_owners_derive_exclusive_retirement_and_plain_owners_keep_shared_original_custody(){
 fn require_send<T:Send>(){}fn require_shared<T:Send+Sync>(){}
 require_send::<ExclusiveCell>();require_shared::<SharedPlain>();let _:fn()=require_send::<ExclusiveCell>;
 assert!(<ExclusiveCell as RetireOwned>::controlled_retirement_supported());
 assert!(ExclusiveCell{close:None}.close.is_none());
 assert!(<SharedPlain as RetireOwned>::shared_original_birth_bytes(&SharedPlain{value:1}).is_some());
 assert!(<ExclusiveCell as RetireOwned>::retirement_birth_bytes(&ExclusiveCell{close:None}).is_some());
}
