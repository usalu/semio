use super::*;
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::controlled::ControlledRetirement};
#[path="../../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;
use effect::{settled,with_effect};
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations as observed;
fn step_observed(owner:&mut ControlledRetirement<LocalizedLabel>,funding:RetainedCloneGrant)->(Result<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize)){let(poll,physical)=with_effect(funding,8,&mut None,|effect|observed(||owner.step(effect)));(settled(poll).result,physical)}
/// 🛑 A denied grant is either a no-progress step or a limit refusal; any other refusal kind is the wrong answer.
fn denied(result:&Result<RetainedCloneStep,OriginalRetirementFailure>)->bool{match result{Ok(step)=>step.progress()==RetainedCloneProgress::default(),Err(failure)=>failure.primary().is_some_and(|error|matches!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit|semio_framework_value::ValueRefusalKind::DepthLimit|semio_framework_value::ValueRefusalKind::WorkLimit))}}
fn cursor_observed(cursor:&mut LocalizedLabelRetirement,funding:RetainedCloneGrant)->(RetirementStep,(usize,usize)){with_effect(funding,8,&mut None,|effect:&mut RetainedAllocationEffect<'_>|observed(||cursor.close_step(effect)))}
fn grant(owner:&ControlledRetirement<LocalizedLabel>)->RetainedCloneGrant{let copy=owner.next_copy_byte_demand().unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}}
fn original(row:&serde_json::Value)->LocalizedLabel{
 let mut cells=std::array::from_fn(|_|std::array::from_fn(|_|Cow::Borrowed("")));
 for(index,input)in row["cells"].as_array().unwrap().iter().enumerate(){let text=input["text"].as_str().unwrap();cells[index/Locale::COUNT][index%Locale::COUNT]=if input["owned"]==true{let mut owned=String::with_capacity(input["capacity"].as_u64().unwrap()as usize);owned.push_str(text);Cow::Owned(owned)}else{Cow::Borrowed(match text{"English"=>"English","Deutsch"=>"Deutsch","Reuse"=>"Reuse","Wiederverwenden"=>"Wiederverwenden",""=>"",_=>panic!("undeclared borrowed fixture")})};}
 LocalizedLabel{cells}
}
#[test]
fn localized_label_original_backing_retirement_obeys_all_physical_grants(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🏷️label/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(Terminology::COUNT*Locale::COUNT,4);
 for row in law["cases"].as_array().unwrap(){
  let label=original(row);let oracle:LocalizedLabel=serde_json::from_value(serde_json::to_value(&label).unwrap()).unwrap();assert_eq!(label,oracle);
  let pointers:Vec<_>=label.cells.iter().flatten().map(|cell|cell.as_ptr()).collect();let original_bytes:usize=label.cells.iter().flatten().map(|cell|match cell{Cow::Owned(text)=>text.capacity(),_=>0}).sum();
  let(mut owner,physical)=observed(||ControlledRetirement::new(label).unwrap_or_else(|(error,_)|panic!("{error}")));assert_eq!(physical,(0,0));
  let mut born=0;let mut released=0;let mut denials=0;
  for _ in 0..1000{
   if owner.terminal_is_empty(){break;}
   let funding=grant(&owner);let(zero,physical)=step_observed(&mut owner,RetainedCloneGrant{maximum_items:0,..funding});let zero=zero.unwrap();assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(physical,(0,0));
   if let Some(label)=owner.original(){for(cell,pointer)in label.cells.iter().flatten().zip(&pointers){assert_eq!(cell.as_ptr(),*pointer);}assert_eq!(label,&oracle);}
   if funding.maximum_capacity_bytes>0{let(result,physical)=step_observed(&mut owner,RetainedCloneGrant{maximum_capacity_bytes:funding.maximum_capacity_bytes-1,..funding});assert_eq!(physical,(0,0));assert!(denied(&result));denials+=1;}
   if funding.maximum_release_bytes>0{let(result,physical)=step_observed(&mut owner,RetainedCloneGrant{maximum_release_bytes:funding.maximum_release_bytes-1,..funding});assert_eq!(physical,(0,0));assert!(denied(&result));denials+=1;}
   if funding.maximum_depth>0{let(result,physical)=step_observed(&mut owner,RetainedCloneGrant{maximum_depth:funding.maximum_depth-1,..funding});assert_eq!(physical,(0,0));assert!(denied(&result));denials+=1;}
   let(step,physical)=step_observed(&mut owner,funding);let step=step.unwrap();let receipt=step.progress();assert!(receipt.fits(funding));assert_eq!(physical,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=physical.0;released+=physical.1;
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,original_bytes+born);let(_,physical)=observed(||drop(owner));assert_eq!(physical,(0,law["required"]["terminalDropBytes"].as_u64().unwrap()as usize));assert!(denials>0);
 }
}

#[test]
fn localized_label_original_backing_direct_cursor_preserves_every_denied_cell(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🏷️label/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let mut cursor=LocalizedLabelRetirement::new(original(row));
  for index in 0..Terminology::COUNT*Locale::COUNT{
   let pointer=cursor.current().unwrap().as_ptr();let capacity=cursor.next_close_byte_demand().unwrap();let funding=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:capacity,maximum_depth:1,..Default::default()};
   for denied in [RetainedCloneGrant{maximum_items:0,..funding},RetainedCloneGrant{maximum_depth:0,..funding}].into_iter().chain((capacity>0).then_some(RetainedCloneGrant{maximum_release_bytes:capacity.saturating_sub(1),..funding})){
    let(_,physical)=cursor_observed(&mut cursor,denied);assert_eq!(physical,(0,0));assert_eq!(cursor.index,index);assert_eq!(cursor.current().unwrap().as_ptr(),pointer);assert_eq!(cursor.next_close_byte_demand(),Some(capacity));
   }
   let(step,physical)=cursor_observed(&mut cursor,funding);assert_eq!(physical,(0,capacity));let RetirementStep::Progress(receipt,_)=step else{panic!("original cell must produce one physical receipt")};assert_eq!(receipt.copied_items,1);assert_eq!(receipt.released_bytes,capacity);assert!(receipt.fits(funding));
  }
  assert!(cursor.terminal_is_empty());let(_,physical)=observed(||drop(cursor));assert_eq!(physical,(0,0));
 }
}
