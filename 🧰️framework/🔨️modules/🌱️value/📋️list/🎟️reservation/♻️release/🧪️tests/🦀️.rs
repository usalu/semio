//! 🧪️ Paid release selection and effect preserve every retained backing through denial and cancellation.
use super::*;
fn retain_empty_metadata<T>(link:&mut Vec<Page<T>>){match link.first_mut(){Some(Page::Branch{children,..})=>{let index=children.iter().rposition(|child|!child.is_empty()).unwrap();return retain_empty_metadata(&mut children[index])},Some(Page::Leaf{items,slots})=>{assert!(items.is_empty());assert_eq!(*slots,0);assert!(size_of::<T>()==0||items.capacity()==0)},None=>panic!("metadata node required")};link.clear();}
fn observe<T>(run:impl FnOnce()->T)->(T,(usize,usize)){crate::observe_retirement_allocations(run)}
fn seed(rows:usize)->PagedList<u64,4096>{let mut list=PagedList::with_page_items(1);for value in 0..rows{while !list.has_reserved_slot(){let bytes=list.next_allocation_bytes().unwrap();list.reserve_one(bytes,&mut None).unwrap();}list.push_reserved(value as u64).unwrap();}while !list.is_empty(){list.truncate_retired_last().unwrap();}list}
#[test]
fn release_selection_matches_neutral_frontiers_and_exact_work(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut owner=seed(row["rows"].as_u64().unwrap()as usize);
  for _ in 0..row["extraReleaseSteps"].as_u64().unwrap(){assert!(owner.release_empty_page(usize::MAX).unwrap().progressed);}
  if row["retainedEmptyMetadata"].as_bool().unwrap(){let((),heap)=observe(||retain_empty_metadata(&mut owner.root));assert_eq!(heap,(0,0));let selected=owner.select_reservation(4096,PagedList::<u64,4096>::reservation_selection_work_demand()).result.unwrap().unwrap();assert_eq!(selected.demand().capacity_bytes,0);}
  let before=(owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr());let quote=PagedList::<u64,4096>::release_selection_work_demand();
  for denied in [0,quote-1]{let(turn,heap)=observe(||owner.select_release(denied));assert_eq!(heap,(0,0));assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());assert_eq!((owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr()),before);}
  let(turn,heap)=observe(||owner.select_release(quote));assert_eq!(heap,(0,0));assert_eq!(turn.work,row["selectionWork"].as_u64().unwrap());assert!(turn.work<=quote);
  let(header_turn,heap)=observe(||owner.inspect_selection_header(1));assert_eq!(heap,(0,0));assert_eq!(header_turn.work,1);let header=header_turn.result.unwrap().unwrap();let staged_quote=header.release_selection_work_demand();
  for denied in [0,staged_quote-1]{let(turn,heap)=observe(||owner.select_release_from_header(&header,denied));assert_eq!(heap,(0,0));assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());}
  let(staged,heap)=observe(||owner.select_release_from_header(&header,staged_quote));assert_eq!(heap,(0,0));assert_eq!(staged.work,row["selectionWork"].as_u64().unwrap());assert!(staged.work<=staged_quote);
  match(turn.result.as_ref().unwrap(),staged.result.as_ref().unwrap()){(Some(a),Some(b))=>{assert_eq!(a.kind(),b.kind());assert_eq!(a.demand(),b.demand());assert_eq!(b.release_work_demand(),row["releaseWork"].as_u64().unwrap());},(None,None)=>{},_=>panic!("paid header changed release")}
  if let Some(selected)=turn.result.unwrap(){
   let d=selected.demand();assert_eq!(selected.kind().as_str(),row["kind"]);assert_eq!(d.depth,row["depth"].as_u64().unwrap()as usize);
   let g=semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..Default::default()};let work=selected.release_work_demand();
   for(fuel,grant)in[(0,g),(work-1,g),(work,semio_framework_value_control_contract::RetainedCloneGrant{maximum_items:0,..g}),(work,semio_framework_value_control_contract::RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g}),(work,semio_framework_value_control_contract::RetainedCloneGrant{maximum_depth:d.depth-1,..g})]{
    let(p,heap)=observe(||owner.release_selected(&selected,grant,fuel).result.unwrap());assert_eq!(p,PagedListProgress::default());assert_eq!(heap,(0,0));assert_eq!((owner.capacity(),owner.allocated_bytes(),owner.root.as_ptr()),before);
   }
   let(p,heap)=observe(||owner.release_selected(&selected,g,work).result.unwrap());assert!(p.progressed);assert_eq!(p.work,row["releaseWork"].as_u64().unwrap());assert_eq!(heap,(0,d.release_bytes));assert_eq!(p.released_allocation_bytes,d.release_bytes);
   let(p,heap)=observe(||owner.release_selected(&selected,g,work));assert_eq!(p.result.unwrap_err().kind,PagedListRefusalKind::InvariantViolated);assert_eq!(heap,(0,0));
  }else{assert_eq!(row["kind"],"none");}
  while !owner.terminal_is_empty(){assert!(owner.release_empty_page(usize::MAX).unwrap().progressed);}
 }
}
#[test]
fn release_selection_refuses_live_payload_without_dropping_it(){
 let mut owner=seed(1);owner.push_reserved(9).unwrap();let(turn,heap)=observe(||owner.select_release(PagedList::<u64,4096>::release_selection_work_demand()));assert_eq!(turn.result.unwrap_err().kind,PagedListRefusalKind::InvariantViolated);assert!(turn.work>0);assert_eq!(heap,(0,0));assert_eq!(owner.pop(),Some(9));while !owner.terminal_is_empty(){owner.release_empty_page(usize::MAX).unwrap();}
}
