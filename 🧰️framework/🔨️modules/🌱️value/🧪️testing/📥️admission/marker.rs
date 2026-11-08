use semio_framework_value_testing_admission::{Wallet,capture,close_owned};

#[test]
fn canonical_staged_capture_posts_actual_work_markers_and_physical_births(){
 let law:serde_json::Value=serde_json::from_str(include_str!("marker.json")).unwrap();let mut state=(Wallet::from_profile(&law["stock"]),Vec::new(),0usize);
 let source=capture(law["value"].as_u64().unwrap(),&mut state,|state|state.0.loan(1,4096),|state|state.0.work_remaining(),|state,p,work,heap,g|{state.0.post(p,u128::from(work),g,heap);state.1.push(work);state.2+=usize::from(heap.0>0)});
 assert_eq!(serde_json::json!(*source.borrow().get()),law["value"]);let actual:Vec<serde_json::Value>=state.1.iter().map(|work|serde_json::json!(work)).collect();let expected:Vec<_>=law["actions"].as_array().unwrap().iter().map(|action|action["work"].clone()).collect();assert_eq!(actual,expected);
 assert_eq!(serde_json::json!({"work":state.1.iter().sum::<u64>(),"turns":state.1.len(),"births":state.2}),law["expected"]);assert_eq!(law["stock"]["fuel"].as_u64().unwrap()-state.0.work_remaining(),law["expected"]["work"].as_u64().unwrap());close_owned(source,&mut state.0,4096,10000);
}
