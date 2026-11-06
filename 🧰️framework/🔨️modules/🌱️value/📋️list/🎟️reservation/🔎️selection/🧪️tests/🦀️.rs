//! 🧪️ Reservation selection pays for path inspection without changing or allocating its original list.
use super::*;
#[test]
fn reservation_selection_real_failed_attempt_retains_its_marker_and_frontier(){
 let mut owner=PagedList::<u64,4>::empty();let selected=owner.select_reservation(1,PagedList::<u64,4>::reservation_selection_work_demand()).result.unwrap().unwrap();let d=selected.demand();
 let g=semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..Default::default()};
 let mut actual=None;let((result,heap),refused)=crate::allocation_testing::refuse_one_allocation_on_this_thread(||observe(||owner.reserve_selected(&selected,g,selected.reserve_work_demand(),&mut actual)));
 assert_eq!(refused,1);assert_eq!(heap,(0,0));let error=result.unwrap_err();assert_eq!(error.kind,PagedListRefusalKind::AllocationFailed);assert!(error.work>0&&error.work<=selected.reserve_work_demand());assert_eq!(error.allocated_bytes,0);assert_eq!(actual,Some(0));assert!(owner.terminal_is_empty());
 let mut occupied=Some(73);let(result,heap)=observe(||owner.reserve_selected(&selected,g,selected.reserve_work_demand(),&mut occupied));assert_eq!(result.unwrap_err().work,1);assert_eq!(heap,(0,0));assert_eq!(occupied,Some(73));assert!(owner.terminal_is_empty());
}
#[test]
fn reservation_selection_zero_sized_payload_has_no_allocation_attempt(){
 let mut owner=PagedList::<(),4>::with_page_items(1);
 for _ in 0..2{
  let selected=owner.select_reservation(1,PagedList::<(),4>::reservation_selection_work_demand()).result.unwrap().unwrap();let d=selected.demand();let g=semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..Default::default()};let mut actual=None;
  let(p,heap)=observe(||owner.reserve_selected(&selected,g,selected.reserve_work_demand(),&mut actual).unwrap());assert!(p.progressed);assert_eq!(heap,(p.allocated_bytes,0));assert_eq!(actual,if d.capacity_bytes==0{None}else{Some(p.allocated_bytes)});
 }
 assert_eq!(owner.capacity(),1);
 while !owner.terminal_is_empty(){assert!(owner.release_empty_page(usize::MAX).unwrap().progressed);}
}
#[test]
fn reservation_selection_execution_keeps_denied_and_stale_original_pages(){
 let mut owner=PagedList::<u64,4096>::with_page_items(1);
 for _ in 0..3{
  let selection=owner.select_reservation(1,PagedList::<u64,4096>::reservation_selection_work_demand()).result.unwrap();
  let Some(selection)=selection else{break};
  let d=selection.demand();let g=semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..Default::default()};
  let work=selection.reserve_work_demand();let before=(owner.len(),owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr());
  for(fuel,grant)in[(0,g),(work-1,g),(work,semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:0,..g}),(work,semio_framework_value_control_contract::RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g}),(work,semio_framework_value_control_contract::RetainedCloneGrant{maximum_depth:d.depth-1,..g})]{
   let mut actual=None;let(p,heap)=observe(||owner.reserve_selected(&selection,grant,fuel,&mut actual).unwrap());assert_eq!(p,PagedListProgress::default());assert_eq!(actual,None);assert_eq!(heap,(0,0));assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr()),before);
  }
  let mut actual=None;let(p,heap)=observe(||owner.reserve_selected(&selection,g,work,&mut actual).unwrap());assert!(p.progressed);assert!(p.work>0&&p.work<=work);assert_eq!(heap,(p.allocated_bytes,0));assert_eq!(actual,Some(p.allocated_bytes));assert_eq!(p.allocated_bytes,d.capacity_bytes);
  let mut stale=None;let(p,heap)=observe(||owner.reserve_selected(&selection,g,work,&mut stale));assert_eq!(p.unwrap_err().kind,PagedListRefusalKind::InvariantViolated);assert_eq!(heap,(0,0));assert_eq!(stale,None);
 }
 assert_eq!(owner.capacity(),1);
 while !owner.terminal_is_empty(){assert!(owner.release_empty_page(usize::MAX).unwrap().progressed);}
}
fn observe<T>(run:impl FnOnce()->T)->(T,(usize,usize)){crate::observe_retirement_allocations(run)}
#[test]
fn reservation_selection_matches_neutral_frontiers_and_work_refusals(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut owner=PagedList::<u64,4096>::with_page_items(1);
  for value in 0..row["rows"].as_u64().unwrap(){while !owner.has_reserved_slot(){let bytes=owner.next_allocation_bytes().unwrap();owner.reserve_one(bytes,&mut None).unwrap();}owner.push_reserved(value).unwrap();}
  for _ in 0..row["extraReserveSteps"].as_u64().unwrap(){let bytes=owner.next_allocation_bytes().unwrap();owner.reserve_one(bytes,&mut None).unwrap();}
  let limit=row["limit"].as_u64().unwrap()as usize;let quote=PagedList::<u64,4096>::reservation_selection_work_demand();
  let before=(owner.len(),owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr());
  for denied in [0,quote-1]{let(turn,heap)=observe(||owner.select_reservation(limit,denied));assert_eq!(heap,(0,0));assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr()),before);}
  let(turn,heap)=observe(||owner.select_reservation(limit,quote));assert_eq!(heap,(0,0));assert!(turn.work<=quote);assert_eq!(turn.work,row["selectionWork"].as_u64().unwrap());let selected=turn.result.unwrap();
  match selected{None=>assert_eq!(row["kind"],"none"),Some(selection)=>{assert!(turn.work>0);assert_eq!(selection.kind().as_str(),row["kind"]);assert_eq!(selection.demand().depth,row["depth"].as_u64().unwrap()as usize);assert_eq!(selection.demand().copy_bytes,0);assert_eq!(selection.demand().capacity_bytes,if row["kind"]=="payload"{size_of::<u64>()}else{size_of::<Page<u64>>()});}}
  let(header_turn,heap)=observe(||owner.inspect_selection_header(1));assert_eq!(heap,(0,0));assert_eq!(header_turn.work,1);let header=header_turn.result.unwrap().unwrap();let staged_quote=header.reservation_selection_work_demand(limit);
  for denied in [0,staged_quote-1]{let(turn,heap)=observe(||owner.select_reservation_from_header(&header,limit,denied));assert_eq!(heap,(0,0));assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());}
  let(staged,heap)=observe(||owner.select_reservation_from_header(&header,limit,staged_quote));assert_eq!(heap,(0,0));assert_eq!(staged.work,row["stagedSelectionWork"].as_u64().unwrap());assert!(staged.work<=staged_quote);
  match(selected,staged.result.unwrap()){(Some(a),Some(b))=>{assert_eq!(a.kind(),b.kind());assert_eq!(a.demand(),b.demand());assert_eq!(b.reserve_work_demand(),row["reserveWork"].as_u64().unwrap());},(None,None)=>{},_=>panic!("paid header changed selection")}
  assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr()),before);
  if let Some(selected)=selected{let d=selected.demand();let g=semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..Default::default()};let mut actual=None;let(p,heap)=observe(||owner.reserve_selected(&selected,g,selected.reserve_work_demand(),&mut actual).unwrap());assert!(p.progressed);assert_eq!(p.work,row["reserveWork"].as_u64().unwrap());assert_eq!(heap,(p.allocated_bytes,0));assert_eq!(actual,Some(p.allocated_bytes));}
  while !owner.is_empty(){owner.truncate_retired_last().unwrap();}
  while !owner.terminal_is_empty(){assert!(owner.release_empty_page(usize::MAX).unwrap().progressed);}
 }
}
#[test]
fn reservation_selection_refuses_invalid_extent_without_inspecting_pages(){
 let owner=PagedList::<u64,4>::empty();
 let(turn,heap)=observe(||owner.select_reservation(5,PagedList::<u64,4>::reservation_selection_work_demand()));
 assert_eq!(heap,(0,0));assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap_err().kind,PagedListRefusalKind::OwnershipLimit);
}
