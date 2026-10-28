//! 🧪️ Direct and mutual ownership cycles remain supported only when every reachable owner is controlled.
use semio_framework_value::{retirement::{RetireOwned,RetirementCursor},RetainedCloneGrant};
#[allow(dead_code)]
#[derive(semio_framework_value::RetireOwned)]
enum Direct{Leaf(String),Next(Box<Direct>)}
#[allow(dead_code)]
#[derive(semio_framework_value::RetireOwned)]
struct Left{right:Option<Box<Right>>,text:String}
#[allow(dead_code)]
#[derive(semio_framework_value::RetireOwned)]
struct Right{left:Vec<Left>}
#[allow(dead_code)]
#[derive(semio_framework_value::RetireOwned)]
struct DeniedLeft{right:Vec<DeniedRight>}
#[allow(dead_code)]
#[derive(semio_framework_value::RetireOwned)]
struct DeniedRight{left:Option<Box<DeniedLeft>>,unsupported:Unsupported}
struct Unsupported;
impl RetireOwned for Unsupported{fn retirement(self)->Box<dyn RetirementCursor>{panic!("unsupported source must remain owned")}fn controlled_retirement_supported()->bool{false}}
#[test]
fn recursive_capability_checks_are_allocation_free_and_keep_unsupported_siblings(){let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||match row["root"].as_str().unwrap(){"direct"=>Direct::controlled_retirement_supported(),"left"=>Left::controlled_retirement_supported(),"denied-left"=>DeniedLeft::controlled_retirement_supported(),_=>unreachable!()});assert_eq!(actual,row["supported"].as_bool().unwrap());assert_eq!(heap,Default::default());}let owner=Left{right:Some(Box::new(Right{left:vec![Left{right:None,text:"retained child".into()}]})),text:"original root".into()};let pointer=owner.text.as_ptr();let mut original=Some(owner);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:4096};let mut close=semio_framework_value::retained_clone::RetainedCloneClose::default();for turn in 0..16384{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{if let Some(step)=close.begin_granted(&mut original,grant).unwrap(){step}else{close.step_granted(grant).unwrap()}});let p=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if original.is_none()&&close.is_empty(){return}assert!(turn<16383,"recursive original retained its cursor");if let Some(owner)=original.as_ref(){assert_eq!(owner.text.as_ptr(),pointer)}}panic!("recursive original did not close")}
