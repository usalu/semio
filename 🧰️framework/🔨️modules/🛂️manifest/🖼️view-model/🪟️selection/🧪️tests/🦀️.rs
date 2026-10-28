//! 🧪️ The read-only selector preserves the original roster and exact selected comparison on every refusal.
use super::*;
#[test]
fn original_window_roster_selection_is_bounded_and_refuses_foreign_sources(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let mut view=ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);view.window_instances=law["windows"].as_array().unwrap().iter().map(|row|ViewWindowInstance{id:row["id"].as_str().unwrap().into(),window_kind_id:row["windowKindId"].as_str().unwrap().into()}).collect();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 for body in law["bodyBytes"].as_array().unwrap(){for case in law["cases"].as_array().unwrap(){let target=case["target"].as_str().unwrap();let body=body.as_u64().unwrap()as usize;let mut selector=ViewModelWindowSelection::default();
  for _ in 0..1024{if selector.complete(){break}let demand=selector.demands(&view,target,19,body).unwrap();for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||selector.advance(&view,target,19,body,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(selector.demands(&view,target,19,body).unwrap(),demand);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||selector.advance(&view,target,19,body,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(step.progress().fits(grant));assert!(step.progress().copied_bytes<=size_of::<ViewModelWindowSelection>().max(2*64+2*size_of::<usize>()+size_of::<Option<(usize,usize)>>()));
  }assert!(selector.complete());let selected=selector.selected(&view,target,19).unwrap();if let Some(index)=case["index"].as_u64(){assert!(std::ptr::eq(selected.unwrap(),&view.window_instances[index as usize]));}else{assert!(selected.is_none())}assert!(selector.selected(&view,target,20).is_err());let foreign=view.clone();assert!(selector.selected(&foreign,target,19).is_err());
 }}
 let target="x".repeat(8197);view.window_instances=vec![ViewWindowInstance{id:target.clone(),window_kind_id:"wide".into()}];let mut selector=ViewModelWindowSelection::default();let mut turns=0;while !selector.complete(){selector.advance(&view,&target,1,4096,grant).unwrap();turns+=1;assert!(turns<1024)}assert!(turns>8197/64);assert_eq!(selector.selected(&view,&target,1).unwrap().unwrap().window_kind_id,"wide");
}
