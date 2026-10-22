use semio_framework_test_fixtures::fixture_text;
use super::*;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1024*1024,maximum_release_bytes:1024*1024,maximum_depth:32}}
fn array(law:&serde_json::Value,key:&str)->Vec<usize>{law[key].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize).collect()}
fn close(plan:&mut IndexedEditPlan)->(usize,usize){
 plan.begin_close();let mut born=0;let mut released=0;let mut turns=0;let shape=plan.shape();let limit=64+12*(shape.removed+shape.modified+shape.added);
 while !plan.terminal_is_empty(){
  let d=plan.close_demand(grant().maximum_copy_bytes).unwrap();let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(plan.close_demand(funded.maximum_copy_bytes).unwrap(),d);
  let custody=plan.orders.each_ref().map(|order|order.as_ref().map(|value|(value.as_ptr(),value.len(),value.capacity())));
  let mut denied=vec![RetainedCloneGrant{maximum_items:0,..funded}];if d.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..funded});}if d.depth>0{denied.push(RetainedCloneGrant{maximum_depth:d.depth-1,..funded});}if d.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..funded});}if d.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..funded});}
  for denied in denied{let selected=plan.close_demand(denied.maximum_copy_bytes).unwrap();assert!(!retained_clone_grant_admits(denied,selected));let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(plan.close_demand(funded.maximum_copy_bytes).unwrap(),d);assert_eq!(plan.orders.each_ref().map(|order|order.as_ref().map(|value|(value.as_ptr(),value.len(),value.capacity()))),custody);}
  let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(funded).unwrap());let p=step.progress();assert!(p.fits(funded));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;released+=heap.1;turns+=1;assert!(turns<limit);
 }
 (born,released)
}
/// 🧪️ Actual coordinate owners obey the neutral strict triple corpus and physical retirement receipts.
#[test]
fn indexed_edit_plan_matches_portable_coordinates_and_original_custody(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for law in fixture["cases"].as_array().unwrap(){
  let removed=array(law,"removed");let modified=array(law,"modified");let added=array(law,"added");let pointers=(removed.as_ptr(),modified.as_ptr(),added.as_ptr());
  let shape=IndexedEditShape{base_len:law["base"].as_u64().unwrap()as usize,removed:removed.len(),modified:modified.len(),added:added.len()};
  let(mut plan,heap)=crate::observe_retirement_allocations(||IndexedEditPlan::new(shape));assert_eq!(heap,(0,0));let mut born=0;let mut turns=0;
  while !plan.settled(){
   let state=(plan.kind,plan.phase,plan.cursor,plan.removal,plan.sort);let demand=plan.demand().unwrap();
   for denied in[RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{let(step,heap)=crate::observe_retirement_allocations(||plan.advance(denied,|_,_|panic!("denied source read")).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(state,(plan.kind,plan.phase,plan.cursor,plan.removal,plan.sort));}
   if demand.capacity_bytes>0{let(step,heap)=crate::observe_retirement_allocations(||plan.advance(RetainedCloneGrant{maximum_capacity_bytes:0,..grant()},|_,_|panic!("denied source read")).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
   let(step,heap)=crate::observe_retirement_allocations(||plan.advance(grant(),|kind,ordinal|match kind{IndexedEditKind::Removed=>removed[ordinal],IndexedEditKind::Modified=>modified[ordinal],IndexedEditKind::Added=>added[ordinal]}).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,0));born+=heap.0;turns+=1;assert!(turns<2000);
  }
  let expected=law["fault"].as_str().map(|kind|match kind{"remove"=>IndexedEditFault::Remove,"modify"=>IndexedEditFault::Modify,"add"=>IndexedEditFault::Add,_=>panic!("fixture fault")});assert_eq!(plan.fault(),expected);
  if expected.is_none(){
   let mut cursor=IndexedOutputCursor::default();let mut output=Vec::new();loop{let work=cursor.next(&plan).unwrap();match work{IndexedOutputWork::Original{index}=>output.push(format!("b{index}")),IndexedOutputWork::Added{ordinal}=>output.push(format!("a{ordinal}")),IndexedOutputWork::Removed{..}=>(),IndexedOutputWork::Complete=>break}cursor.advance(&plan).unwrap();}assert_eq!(serde_json::to_value(output).unwrap(),law["output"]);
  }
  assert_eq!(pointers,(removed.as_ptr(),modified.as_ptr(),added.as_ptr()));let(close_born,released)=close(&mut plan);assert_eq!(born+close_born,released);assert_eq!(crate::observe_retirement_allocations(||drop(plan)).1,(0,0));
 }
 eprintln!("[DEBUG] fourteen strict indexed coordinate laws deny source reads and physical changes under absent grants; admitted streams preserve original payload ordinals; every paid directory birth is returned through original close receipts");
}
/// ✂️ Large index directories remain resumable and close original storage at every selected cancellation frontier.
#[test]
fn indexed_edit_plan_wide_order_and_cancellation_remain_bounded(){
 let count=2049;let added:Vec<usize>=(0..count).rev().collect();let pointer=added.as_ptr();
 for cut in[Some(0),Some(1),Some(7),Some(count+12),Some(count*4),None]{
  let mut plan=IndexedEditPlan::new(IndexedEditShape{base_len:0,removed:0,modified:0,added:count});let mut born=0;let mut steps=0;
  while !plan.settled()&&cut.is_none_or(|cut|steps<cut){let(step,heap)=crate::observe_retirement_allocations(||plan.advance(grant(),|kind,ordinal|{assert_eq!(kind,IndexedEditKind::Added);added[ordinal]}).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,0));born+=heap.0;steps+=1;assert!(steps<count*32*12);}
  if cut.is_none(){assert!(plan.ready());let rows=plan.entries(IndexedEditKind::Added).unwrap();for(index,row)in rows.iter().enumerate(){assert_eq!((row.index,row.ordinal),(index,count-1-index));}assert_eq!(plan.output_len().unwrap(),count);}
  let(close_born,released)=close(&mut plan);assert_eq!(born+close_born,released);assert_eq!(added.as_ptr(),pointer);assert_eq!(crate::observe_retirement_allocations(||drop(plan)).1,(0,0));
 }
 eprintln!("[DEBUG] actual 2049-entry indexed owner sorts through shared bounded heap actions and returns exact original backing at five cancellation frontiers plus completion");
}
